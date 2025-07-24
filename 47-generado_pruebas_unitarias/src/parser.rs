use swc_commom::sync::Lrc;
use swc_commom::{FileName, SourceMap};
use syn::parse::Parser;
use swc_ecma_parser::{lexer::lexer, Parser, StringInput, Syntax};
use swc_ecma_ast::*;

pub fn analyze_component(file_content: &str) -> Vec<String>{
    let source_map = Lrc::new(SourceMap::default());
    let fm = source_map.new_source_file(FileName::Custom("componen.jsx".into()), file_content.into());

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let mut props = Vec::new();

    if let Ok(module) = parser.parse_module(){
        for item in module.body {
            if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = item {
                for decl in &var_decl.decls{
                    if let Some(expr) = &decl.init{
                        if let Expr::Fn(fn_expr) = &**expr{
                            props.push(fn_expr.ident.as_ref().unwrap().sym.to_string());
                        }
                    }
                }
            }
        }
    }
    props
}
