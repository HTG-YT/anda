use anyhow::{anyhow, Result, Ok};
use curl::easy::{Easy, Form, Part};
use hyper::client::{Client};
use log::debug;
use serde_derive::Serialize;
use walkdir::WalkDir;
use std::{env};
use std::{
    path::PathBuf,
    process::{Command, ExitStatus},
    collections::HashMap,
};
trait ExitOkPolyfill {
    fn exit_ok_polyfilled(&self) -> Result<()>;
}

impl ExitOkPolyfill for ExitStatus {
    fn exit_ok_polyfilled(&self) -> Result<()> {
        if self.success() {
            Ok(())
        } else {
            Err(anyhow!("process exited with non-zero status"))
        }
    }
}


#[derive(Debug, Clone, Serialize)]
struct ArtifactUploader {
    pub files: HashMap<String, PathBuf>,
}

impl ArtifactUploader {
    pub fn new(files: HashMap<String, PathBuf>) -> Self {
        Self {
            files,
        }
    }

    pub async fn upload(&self) -> Result<()> {
        // For this part, we will be using libcurl to upload the files to the server.
        // why? because for some reason, reqwest's implementation of multipart forms are kinda broken
        // I've posted a bug here: https://github.com/seanmonstar/reqwest/issues/1585
        // Now we will be relying on 2 libraries to interact with the server.
        // because curl does not have serde serialization.
        let endpoint = format!("{}/artifacts",env::var("ANDA_ENDPOINT")?);
        let build_id = env::var("ANDA_BUILD_ID")?;

        // files is a hashmap of path -> actual file path
        // we need to convert them into a tuple of (path, file)
        // files[path] = actual_path
        let files: Vec<(String, PathBuf)> = self.files.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        let mut easy = Easy::new();
        easy.url(&endpoint)?;
        easy.verbose(true)?;
        easy.post(true)?;

        let mut form = Form::new();
        form.part("build_id").contents(build_id.as_bytes()).add()?;

        // let mut form = multipart::client::lazy::Multipart::new();
        // let mut form = form.add_text("build_id", build_id);

        for file in &files {
            // add to array of form data
            let (path, aa) = file;
            debug!("path: {:?}", path);
            let filep = &format!("files[{}]", path);
            debug!("filep: {:?}", filep);
            let mut f = form.part(filep);
            f.file(aa);
            //file_part.file(aa);
            f.add()?;
        }

        //file_part.add()?;
        //id_part.add()?;
        debug!("form: {:#?}", form);

        easy.httppost(form)?;
        easy.perform()?;

        debug!("res: {:#?}", easy.response_code());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ProjectBuilder {
    root: PathBuf,
}

impl ProjectBuilder {
    pub fn new(root: PathBuf) -> Self {
        ProjectBuilder { root }
    }

    pub async fn push_folder(&self, folder: PathBuf) -> Result<()> {

        let mut hash = HashMap::new();

        for entry in WalkDir::new(&folder) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let file_path = entry.into_path();
                let real_path = file_path.strip_prefix(&folder).unwrap();
                println!("path: {}", real_path.display());
                hash.insert(real_path.display().to_string(), file_path);
            }
        }

        let uploader = ArtifactUploader::new(hash);
        uploader.upload().await?;

        Ok(())
    }


    pub fn dnf_builddep(&self) -> Result<()> {
        let config = crate::config::load_config(&self.root)?;

        let spec_path = config.package.spec.canonicalize()?;

        let builddep_exit = runas::Command::new("dnf")
            .args(&[
                "builddep",
                "-y",
                &spec_path.to_str().unwrap(),
            ])
            .status()?;

        builddep_exit.exit_ok_polyfilled()?;
        Ok(())
    }

    ///  Builds an Andaman project.
    pub async fn build(&self) -> Result<()> {
        // TODO: Move this to a method called `build_rpm` as we support more project types
        let config = crate::config::load_config(&self.root)?;

        self.dnf_builddep()?;

        let rpmbuild_exit = Command::new("rpmbuild")
            .args(vec![
                "-ba",
                config.package.spec.to_str().unwrap(),
                "--define",
                format!("_rpmdir anda-build").as_str(),
                "--define",
                format!("_srcrpmdir anda-build").as_str(),
                "--define",
                "_disable_source_fetch 0",
                "--define",
                format!("_sourcedir {}", tokio::fs::canonicalize(&self.root).await?.to_str().unwrap()).as_str(),
            ])
            .current_dir(&self.root)
            .status()?;

        rpmbuild_exit.exit_ok_polyfilled()?;

        self.push_folder(PathBuf::from("anda-build")).await?;

        Ok(())
    }
}
