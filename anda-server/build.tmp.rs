use npm_rs::*;
use std::fs;
// use std::env;
// use std::path::PathBuf;

fn main() {

    let old_pwd = std::env::current_dir().unwrap();
    // change current directory to anda-frontend
    std::env::set_current_dir("../anda-frontend").unwrap();
    let exit_status = NpmEnv::default()
       .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
       .init_env()
       .run("build")
       .exec().unwrap();
    std::env::set_current_dir(old_pwd).unwrap();

    // if symlink already exists
    let symlink = fs::read_link("dist");
    /* if symlink.is_ok() {
        // check if symlink is correct
        let symlink_path = symlink.unwrap();
        if symlink_path.to_str().unwrap() == "../anda-frontend/dist" {
            println!("symlink already exists");
            return;
        } else {
            std::os::unix::fs::symlink("../anda-frontend/dist", "dist").unwrap();
        }
    } else {
        std::os::unix::fs::symlink("../anda-frontend/dist", "dist").unwrap();
    } */


    if let Ok(symlink_path) = symlink {
        if symlink_path.to_str().unwrap() == "../anda-frontend/dist" {
            println!("symlink already exists");
            return;
        } else {
            std::os::unix::fs::symlink("../anda-frontend/dist", "dist").unwrap();
        }
    } else {
        std::os::unix::fs::symlink("../anda-frontend/dist", "dist").unwrap();
    }

    // copy anda-frontend/dist folder to anda-server/dist folder
    //std::os::unix::fs::symlink("../anda-frontend/dist", "dist").unwrap();

    println!("{:?}", exit_status);
}
