use swc_common::input::SourceFileInput;
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::{error::Error, lexer::Lexer, Parser, Syntax, TsConfig};

#[allow(dead_code)]
pub fn parse_module(source: &str, use_tsx: bool) -> (Result<Module, Error>, usize) {
    let line_count = source.lines().count();
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        swc_common::FileName::Custom("input.ts".to_string()),
        source.to_string(),
    );

    let ts_config = TsConfig {
        tsx: use_tsx,
        decorators: false,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: true,
    };

    let lexer = Lexer::new(
        Syntax::Typescript(ts_config),
        EsVersion::Es2020,
        SourceFileInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let parsed = parser.parse_module();

    (parsed, line_count)
}
