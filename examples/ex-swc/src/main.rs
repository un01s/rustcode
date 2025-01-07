use std::{path::Path, sync::Arc};
use swc::config::JscTarget;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_parser::{EsConfig, Syntax};

fn main() {
    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));
    let c = swc::Compiler::new(cm.clone(), handler.clone());
    let fm = cm
        .load_file(Path::new("test.js"))
        .expect("failed to load file");
    let result = c.parse_js(
        fm,
        JscTarget::Es2020,
        Syntax::Es(EsConfig::default()),
        true,
        false,
    );
    dbg!(result);
}

