mod rpm;
mod tsunagu;

use anda_config::Manifest;
use anyhow::Result;
use log::warn;
use rhai::{Engine, Scope};
use serde_json::Value;
use std::path::PathBuf;
use tsunagu::ehdl;

fn gen_en(rpmspec: rpm::RPMSpec) -> (Engine, Scope<'static>) {
    let mut sc = Scope::new();
    sc.push("rpm", rpmspec);
    sc.push("USER_AGENT", tsunagu::USER_AGENT);
    let mut en = Engine::new();
    en.register_fn("get", |a: String| ehdl(tsunagu::get(a)))
        .register_fn("get", |a: String| ehdl(tsunagu::get(a)))
        .register_fn("gh", |a: String| ehdl(tsunagu::gh(a)))
        .register_fn("json", |a: String| ehdl(tsunagu::json(a)))
        .register_fn("get_json", |o: Value, i: String| {
            ehdl(tsunagu::get_json(o, i))
        })
        .register_fn("get_json", |o: Value, i: i64| {
            ehdl(tsunagu::get_json_i(o, i))
        })
        .register_fn("string_json", |a: Value| ehdl(tsunagu::string_json(a)))
        .register_fn("i64_json", |a: Value| ehdl(tsunagu::i64_json(a)))
        .register_fn("f64_json", |a: Value| ehdl(tsunagu::f64_json(a)))
        .register_fn("bool_json", |a: Value| ehdl(tsunagu::bool_json(a)))
        .build_type::<tsunagu::Req>()
        .build_type::<rpm::RPMSpec>();
    (en, sc)
}

pub fn update_pkgs(cfg: Manifest) -> Result<()> {
    for (name, proj) in cfg.project {
        if let Some(rpm) = proj.rpm {
            let spec = rpm.spec;
            if rpm.update.is_none() {
                continue;
            }
            let mut scr = rpm.update.unwrap();
            if scr.is_empty() {
                scr = "update.rhai".into();
            }
            let rpmspec = rpm::RPMSpec::new(name.clone(), &scr, spec)?;
            let (en, mut sc) = gen_en(rpmspec);

            match en.run_file_with_scope(&mut sc, PathBuf::from(&scr)) {
                Ok(()) => {
                    let rpm = sc
                        .get_value::<rpm::RPMSpec>("rpm")
                        .expect("No rpm object in rhai scope");
                    if rpm.changed {
                        rpm.write()?
                    }
                }
                Err(err) => {
                    let e = *err;
                    warn!("Fail {name}:\n{e}");
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    fn run_update(rpmspec: rpm::RPMSpec) -> Result<()> {
        // FIXME can we avoid clone()
        let name = rpmspec.name.clone();
        let scr = rpmspec.chkupdate.clone();
        let (en, mut sc) = gen_en(rpmspec);

        match en.run_file_with_scope(&mut sc, scr) {
            Ok(()) => {
                let rpm = sc
                    .get_value::<rpm::RPMSpec>("rpm")
                    .expect("No rpm object in rhai scope");
                if rpm.changed {
                    rpm.write()?;
                }
                Ok(())
            }
            Err(err) => {
                let e = *err;
                warn!("Fail {}:\n{e}", name);
                Err(anyhow!(e.to_string()))
            }
        }
    }

    #[test]
    fn run_rhai() -> Result<()> {
        run_update(rpm::RPMSpec::new(
            "umpkg".into(),
            "tests/test.rhai",
            "tests/umpkg.spec",
        )?)?;
        Ok(())
    }
}
