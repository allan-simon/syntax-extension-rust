use syntax::{ast, codemap, parse};
use syntax::ext::base;
use parse::Parse;
use syntax::parse::token;
use syntax::ext::base::MacExpr;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;  // trait for expr_uint

use generate::Generate;

/// Defines the state of a `html_template!` macro as it is parsing.
///
///
#[deriving(Clone)]
pub struct HtmlState {
    pub name: Option<ast::Ident>
}

///
///
///
impl HtmlState {
    pub fn new(name: Option<ast::Ident>) -> HtmlState {
        HtmlState {
            name: name,
        }
    }
}

///
///
///
pub fn html_template<'a>(
    cx: &'a mut ExtCtxt,
    sp: codemap::Span,
    name: ast::Ident,
    tokens: Vec<ast::TokenTree>
) -> Box<base::MacResult + 'a> {

    let state: HtmlState = Parse::parse(
        &mut parse::tts_to_parser(
            cx.parse_sess(),
            tokens.into_vec(),
            cx.cfg()
        ),
        (sp, &mut*cx, Some(name))
    );


    base::MacItems::new(
        Some(
            state.generate(
                sp,
                cx,
                ()
            )
        ).into_iter()
    )

}
