use crate::error::ParserError;
use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::Command,
};

//? https://rpm-software-management.github.io/rpm/manual/spec.html
const PREAMBLES: &[&str] = &[
    "Name",
    "Version",
    "Release",
    "Epoch",
    "License",
    "SourceLicense",
    "Group",
    "Summary",
    "URL",
    "BugURL",
    "ModularityLabel",
    "DistTag",
    "VCS",
    "Distribution",
    "Vendor",
    "Packager",
    "BuildRoot",
    "AutoReqProv",
    "AutoReq",
    "AutoProv",
    "Requires",
    "Provides",
    "Conflicts",
    "Obsoletes",
    "Recommends",
    "Suggests",
    "Supplements",
    "Enhances",
    "OrderWithRequires",
    "BuildRequires",
    "BuildConflicts",
    "ExcludeArch",
    "ExclusiveArch",
    "ExcludeOS",
    "ExclusiveOS",
    "BuildArch",
    "BuildArchitectures",
    "Prefixes",
    "Prefix",
    "DocDir",
    "RemovePathPostfixes",
    // list
    "Source#",
    "Patch#",
];

#[derive(Clone)]
struct Package {
    name: String,
    version: Option<String>,
    release: Option<String>,
    epoch: Option<i32>,
    condition: Option<String>,
}
impl Package {
    fn new(name: String) -> Self {
        Package {
            name,
            version: None,
            release: None,
            epoch: None,
            condition: None,
        }
    }
}

struct RPMRequires {
    none: Vec<Package>,
    pre: Vec<Package>,
    post: Vec<Package>,
    preun: Vec<Package>,
    postun: Vec<Package>,
    pretrans: Vec<Package>,
    posttrans: Vec<Package>,
    verify: Vec<Package>,
    interp: Vec<Package>,
    meta: Vec<Package>,
}
impl RPMRequires {
    fn new() -> Self {
        Self {
            none: vec![],
            interp: vec![],
            meta: vec![],
            post: vec![],
            posttrans: vec![],
            postun: vec![],
            pre: vec![],
            pretrans: vec![],
            preun: vec![],
            verify: vec![],
        }
    }
}

struct Scriptlets {
    pre: Option<String>,
    post: Option<String>,
    preun: Option<String>,
    postun: Option<String>,
    pretrans: Option<String>,
    posttrans: Option<String>,
    verify: Option<String>,

    triggerprein: Option<String>,
    triggerin: Option<String>,
    triggerun: Option<String>,
    triggerpostun: Option<String>,

    filetriggerin: Option<String>,
    filetriggerun: Option<String>,
    filetriggerpostun: Option<String>,
    transfiletriggerin: Option<String>,
    transfiletriggerun: Option<String>,
    transfiletriggerpostun: Option<String>,
}
impl Scriptlets {
    fn new() -> Self {
        Self {
            pre: None,
            post: None,
            preun: None,
            postun: None,
            pretrans: None,
            posttrans: None,
            verify: None,
            triggerprein: None,
            triggerin: None,
            triggerun: None,
            triggerpostun: None,
            filetriggerin: None,
            filetriggerun: None,
            filetriggerpostun: None,
            transfiletriggerin: None,
            transfiletriggerun: None,
            transfiletriggerpostun: None,
        }
    }
}

enum ConfigFileMod {
    None,
    MissingOK,
    NoReplace,
}

enum VerifyFileMod {
    FileDigest, // or 'md5'
    Size,
    Link,
    User, // or 'owner'
    Group,
    Mtime,
    Mode,
    Rdev,
    Caps,
}

struct Files {
    // %artifact
    artifact: Vec<String>,
    // %ghost
    ghost: Vec<String>,
    // %config
    config: HashMap<String, ConfigFileMod>,
    // %dir
    dir: Vec<String>,
    // %doc
    doc: Vec<String>,
    // %license
    license: Vec<String>,
    // %readme (obsolete)
    // %verify
    verify: HashMap<String, VerifyFileMod>,
}
impl Files {
    fn new() -> Self {
        Self {
            artifact: vec![],
            ghost: vec![],
            config: HashMap::new(),
            dir: vec![],
            doc: vec![],
            license: vec![],
            verify: HashMap::new(),
        }
    }
}

struct Changelog {
    date: String, // ! any other?
    version: Option<String>,
    maintainer: String,
    email: String,
    message: String,
}

struct RPMSpec {
    globals: HashMap<String, String>,
    defines: HashMap<String, String>,

    // %description
    description: Option<String>,
    // %prep
    prep: Option<String>,
    // %generate_buildrequires
    generate_buildrequires: Option<String>,
    // %conf
    conf: Option<String>,
    // %build
    build: Option<String>,
    // %install
    install: Option<String>,
    // %check
    check: Option<String>,

    scriptlets: Scriptlets,
    files: Files,              // %files
    changelog: Vec<Changelog>, // %changelog

    //* preamble
    name: Option<String>,
    version: Option<String>,
    release: Option<String>,
    epoch: Option<i32>,
    license: Option<String>,
    sourcelicense: Option<String>,
    group: Option<String>,
    summary: Option<String>,
    sources: HashMap<i16, String>,
    patches: HashMap<i16, String>,
    // icon
    // nosource nopatch
    url: Option<String>,
    bugurl: Option<String>,
    modularitylabel: Option<String>,
    disttag: Option<String>,
    vsc: Option<String>,
    distribution: Option<String>,
    vendor: Option<String>,
    packager: Option<String>,
    // buildroot
    autoreqprov: bool,
    autoreq: bool,
    autoprov: bool,
    requires: RPMRequires,
    provides: Vec<Package>,
    conflicts: Vec<Package>,
    obsoletes: Vec<Package>,
    suggests: Vec<Package>,
    // recommends suggests supplements enhances
    orderwithrequires: Vec<Package>,
    buildrequires: Vec<Package>,
    buildconflicts: Vec<Package>,
    excludearch: Vec<String>,
    exclusivearch: Vec<String>,
    excludeos: Vec<String>,
    exclusiveos: Vec<String>,
    buildarch: Vec<String>, // BuildArchitectures BuildArch
    prefix: Option<String>, // Prefixes Prefix
    docdir: Option<String>,
    removepathpostfixes: Vec<String>,
}

impl RPMSpec {
    fn new() -> Self {
        Self {
            globals: HashMap::new(),
            defines: HashMap::new(),
            description: None,
            prep: None,
            generate_buildrequires: None,
            conf: None,
            build: None,
            install: None,
            check: None,
            scriptlets: Scriptlets::new(),
            files: Files::new(),
            changelog: vec![],
            name: None,
            version: None,
            release: None,
            epoch: None,
            license: None,
            sourcelicense: None,
            group: None,
            summary: None,
            sources: HashMap::new(),
            patches: HashMap::new(),
            // icon
            // nosource nopatch
            url: None,
            bugurl: None,
            modularitylabel: None,
            disttag: None,
            vsc: None,
            distribution: None,
            vendor: None,
            packager: None,
            // buildroot
            autoreqprov: true,
            autoreq: true,
            autoprov: true,
            requires: RPMRequires::new(),
            provides: vec![],
            conflicts: vec![],
            obsoletes: vec![],
            suggests: vec![],
            // recommends suggests supplements enhances
            orderwithrequires: vec![],
            buildrequires: vec![],
            buildconflicts: vec![],
            excludearch: vec![],
            exclusivearch: vec![],
            excludeos: vec![],
            exclusiveos: vec![],
            buildarch: vec![], // BuildArchitectures BuildArch
            prefix: None,      // Prefixes Prefix
            docdir: None,
            removepathpostfixes: vec![],
        }
    }
}

struct SpecParser {
    rpm: RPMSpec,
    errors: Vec<Result<(), ParserError>>,
}

impl SpecParser {
    fn parse_multiline(&self, sline: &str) {
        todo!();
    }
    fn parse_macro(&self, sline: &str) {
        // run rpm --eval
    }

    // returns true if it passes the check
    fn preamble_check(&mut self, name: String, ln: usize) -> bool {
        if !PREAMBLES.contains(&name.as_str()) {
            self.errors
                .push(Err(ParserError::UnknownPreamble(ln, name)));
            return false;
        }
        true
    }
    fn parse_requires(&mut self, sline: &str, ln: usize) -> bool {
        lazy_static! {
            static ref RE1: Regex =
                Regex::new(r"(?m)^Requires(?:\(([\w,\s]+)\))?:\s*(.+)$").unwrap();
            static ref RE2: Regex =
                Regex::new(r"(?m)([\w-]+)(?:\s*([>=<]{1,2})\s*([\d._~^]+))?").unwrap();
        }
        if let Some(caps) = RE1.captures(sline) {
            let spkgs = &caps[caps.len()].trim();
            let mut pkgs = vec![];
            for cpkg in RE2.captures_iter(spkgs) {
                let mut pkg = Package::new(cpkg[cpkg.len() - 1].to_string());
                if cpkg.len() == 3 {
                    // get rid of spaces I guess
                    pkg.condition = Some(format!("{}{}", &cpkg[1], &cpkg[2]));
                }
                pkgs.push(pkg);
            }
            let modifiers = if caps.len() == 2 {
                &caps[2]
            } else {
                "none"
            };
            for modifier in modifiers.split(',') {
                let modifier = modifier.trim();
                let pkgs = pkgs.to_vec();
                match modifier {
                    "none" => self.rpm.requires.none.extend(pkgs),
                    "pre" => self.rpm.requires.pre.extend(pkgs),
                    "post" => self.rpm.requires.post.extend(pkgs),
                    "preun" => self.rpm.requires.preun.extend(pkgs),
                    "postun" => self.rpm.requires.postun.extend(pkgs),
                    "pretrans" => self.rpm.requires.pretrans.extend(pkgs),
                    "posttrans" => self.rpm.requires.posttrans.extend(pkgs),
                    "verify" => self.rpm.requires.verify.extend(pkgs),
                    "interp" => self.rpm.requires.interp.extend(pkgs),
                    "meta" => self.rpm.requires.meta.extend(pkgs),
                    // _ => bail!("Unknown Modifier '{}' for Requires", modifier),
                    _ => {
                        self.errors
                            .push(Err(ParserError::UnknownModifier(ln, modifier.to_string())));
                    }
                }
            }
            true
        } else {
            false
        }
    }
    fn load_macros(&mut self) -> Result<()> {
        // run rpm --showrc | grep "^Macro path"
        let paths = String::from_utf8(
            Command::new("sh")
                .args([
                    "-c",
                    "rpm --showrc|grep '^Macro path'|sed 's/Macro path: //'",
                ])
                .output()?
                .stdout,
            )?
            .split(':');
        todo!();
        // for path in paths, read => regex => hashmap
        Ok(())
    }
    fn parse<R: std::io::Read>(&mut self, bufread: BufReader<R>) -> Result<()> {
        let re = Regex::new(r"(\w+):\s*(.+)").unwrap();
        let re_dnl = Regex::new(r"^%dnl\b").unwrap();
        let re_digit = Regex::new(r"\d+$").unwrap();
        let mut preambles: HashMap<String, Vec<String>> = HashMap::new();
        let mut list_preambles: HashMap<String, HashMap<i16, String>> = HashMap::new();
        'll: for (line_number, line) in bufread.lines().enumerate() {
            let line = line?;
            let sline = line.trim();
            // * we have to parse %macros here (just like rpm)
            if sline.is_empty() || sline.starts_with('#') || re_dnl.is_match(sline) {
                continue;
            }
            // Check for Requires special preamble syntax first
            if self.parse_requires(sline, line_number) {
                continue;
            }
            // only then do we check for other preambles
            for cap in re.captures_iter(sline) {
                // key already exists
                if preambles.contains_key(&cap[1]) {
                    if re_digit.is_match(&cap[1]) {
                        self.errors
                            .push(Err(ParserError::Duplicate(line_number, cap[1].to_string())));
                        continue 'll;
                    }
                    preambles.get_mut(&cap[1]).unwrap().push(cap[2].to_string());
                    continue 'll;
                }
                // check for list_preambles
                if let Some(digitcap) = re_digit.captures(&cap[1]) {
                    let sdigit = &digitcap[0];
                    let digit: i16 = sdigit.parse()?;
                    let name = &cap[1][..cap[1].len() - sdigit.len()];
                    let sname = name.to_string();
                    if !self.preamble_check(format!("{}#", name), line_number) {
                        continue 'll;
                    }
                    if !list_preambles.contains_key(&sname) {
                        list_preambles.insert(name.to_string(), HashMap::new());
                    }
                    match &mut list_preambles.get_mut(&sname) {
                        Some(hm) => hm.insert(digit, cap[2].to_string()),
                        None => bail!("BUG: added HashMap gone"),
                    };
                } else {
                    let name = cap[1].to_string();
                    if !self.preamble_check(name, line_number) {
                        continue 'll;
                    }
                    // create key with new value (normal preambles)
                    preambles.insert(cap[1].to_string(), vec![cap[2].to_string()]);
                }
            }
            // ! error?
        }
        preambles
            .iter()
            .map(|(k, v)| self.set_preamble(k, v))
            .collect::<Result<Vec<_>>>()?;
        list_preambles
            .iter()
            .map(|(k, v)| self.set_list_preamble(k, v))
            .collect::<Result<Vec<_>>>()?;
        if !self.errors.is_empty() {
            return Err(anyhow!("{:#?}", self.errors));
        }
        Ok(())
    }

    fn set_list_preamble(&mut self, name: &str, value: &HashMap<i16, String>) -> Result<()> {
        let value = value.to_owned();
        let rpm = &mut self.rpm;
        match name {
            "Source" => rpm.sources = value,
            "Patch" => rpm.patches = value,
            _ => bail!("BUG: failed to match preamble '{}'", name),
        }
        Ok(())
    }

    fn set_preamble(&mut self, name: &String, value: &Vec<String>) -> Result<()> {
        let rpm = &mut self.rpm;
        match name.as_str() {
            "Name" => rpm.name = _ssin(value),
            "Version" => rpm.version = _ssin(value),
            "Release" => rpm.release = _ssin(value),
            "Epoch" => {
                rpm.epoch = _ssin(value).map(|x| x.parse().expect("Failed to decode epoch to int"))
            }
            "License" => rpm.license = _ssin(value),
            "SourceLicense" => rpm.sourcelicense = _ssin(value),
            "Group" => rpm.group = _ssin(value), // ! confirm?
            "Summary" => rpm.summary = _ssin(value),
            "URL" => rpm.url = _ssin(value),
            "BugURL" => rpm.bugurl = _ssin(value),
            "ModularityLabel" => rpm.modularitylabel = _ssin(value),
            "DistTag" => rpm.disttag = _ssin(value),
            "VCS" => {}
            "Distribution" => rpm.distribution = _ssin(value),
            "Vendor" => rpm.vendor = _ssin(value),
            "Packager" => rpm.packager = _ssin(value),
            "AutoReqProv" => rpm.autoreqprov = _sbin(value)?,
            "AutoReq" => rpm.autoreq = _sbin(value)?,
            "AutoProv" => rpm.autoprov = _sbin(value)?,
            "Provides" => {}
            "Conflicts" => {}
            "Obsoletes" => {}
            "Recommends" => {}
            "Suggests" => {}
            "Supplements" => {}
            "Enhances" => {}
            "OrderWithRequires" => {}
            "BuildRequires" => {}
            "BuildConflicts" => {}
            "ExcludeArch" => {}
            "ExclusiveArch" => {}
            "ExcludeOS" => {}
            "ExclusiveOS" => {}
            "BuildArch" => {}
            "BuildArchitectures" => {}
            "Prefixes" => {}
            "Prefix" => {}
            "DocDir" => {}
            "RemovePathPostfixes" => {}
            _ => bail!("BUG: failed to match preamble '{}'", name),
        }
        Ok(())
    }
    fn parse_macros(line: &str) -> Result<()> {
        Ok(())
    }
    fn new() -> Self {
        Self {
            rpm: RPMSpec::new(),
            errors: vec![],
        }
    }
}

fn _single(value: &Vec<String>) -> &String {
    assert_eq!(value.len(), 1);
    &value[0]
}
fn _ssin(value: &Vec<String>) -> Option<String> {
    Some(_single(value).to_owned())
}
fn _sbin(value: &Vec<String>) -> Result<bool> {
    Ok(_single(value).to_owned().parse()?)
}

mod tests {
    use std::fs::File;

    use super::*;
    #[test]
    fn parse_spec() -> Result<()> {
        let f = File::open("../tests/test.spec")?;
        let f = BufReader::new(f);

        let mut sp = SpecParser::new();
        sp.parse(f)?;
        println!("{}", sp.rpm.name.unwrap_or_default());
        println!("{}", sp.rpm.summary.unwrap_or_default());
        Ok(())
    }
}
