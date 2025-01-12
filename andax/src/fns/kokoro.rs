use crate::error::{AndaxError as AErr, AndaxRes};
use crate::run::rf;
use regex::Regex;
use rhai::{plugin::*, Dynamic, EvalAltResult as RhaiE};
type Res<T = ()> = Result<T, Box<RhaiE>>;

#[export_module]
pub mod ar {
    #[rhai_fn(return_raw)]
    pub(crate) fn terminate(ctx: NativeCallContext) -> Res {
        Err(Box::new(RhaiE::ErrorRuntime(Dynamic::from(AErr::Exit(false)), ctx.position())))
    }
    #[rhai_fn(return_raw)]
    pub(crate) fn defenestrate(ctx: NativeCallContext) -> Res {
        Err(Box::new(RhaiE::ErrorRuntime(Dynamic::from(AErr::Exit(true)), ctx.position())))
    }
    #[rhai_fn(return_raw)]
    pub(crate) fn json(ctx: NativeCallContext, a: String) -> Res<rhai::Map> {
        ctx.engine().parse_json(a, true)
    }
    #[rhai_fn(return_raw)]
    pub(crate) fn json_arr(ctx: NativeCallContext, a: String) -> Res<rhai::Array> {
        serde_json::from_str(&a).ehdl(&ctx)
    }
    #[rhai_fn(return_raw)]
    pub(crate) fn find(ctx: NativeCallContext, r: &str, text: &str, group: i64) -> Res<String> {
        let captures = Regex::new(r).ehdl(&ctx)?.captures(text);
        let cap = captures.ok_or_else(|| format!("Can't match regex: {r}\nText: {text}"))?;
        Ok(cap
            .get(rf(ctx, group.try_into().map_err(color_eyre::Report::new))?)
            .ok_or_else(|| format!("Can't get group: {r}\nText: {text}"))?
            .as_str()
            .into())
    }
    #[rhai_fn(return_raw)]
    pub(crate) fn sub(ctx: NativeCallContext, r: &str, rep: &str, text: &str) -> Res<String> {
        Ok(Regex::new(r).ehdl(&ctx)?.replace_all(text, rep).into())
    }
}
