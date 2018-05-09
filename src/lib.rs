#![feature(plugin_registrar, rustc_private)]
#![allow(dead_code)]

// TODO Find a way to enable doc tests

/// Lower case macro
///
/// # Examples
///
/// ```
/// #![feature(plugin)]
/// #![plugin(casing_macros)]
/// assert_eq!("a string", to_lower!("A String"));
/// assert_eq!("identifier", to_lower!(stringify!(Identifier)));
/// ```
#[macro_export]
macro_rules! to_lower { ($e:expr) => { /* Plugin builtin */ } }

/// Upper case macro
///
/// # Examples
///
/// ```
/// #![feature(plugin)]
/// #![plugin(casing_macros)]
/// assert_eq!("A STRING", to_lower!("A String"));
/// assert_eq!("IDENTIFIER", to_lower!(stringify!(Identifier)));
/// ```
#[macro_export]
macro_rules! to_upper { ($e:expr) => { /* Plugin builtin */ } }

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::codemap::Span;
use syntax::ast;
use syntax::ast::Name;
use syntax::tokenstream::TokenTree;
use syntax::ext::base;

#[plugin_registrar]
fn plugin_regsitrar(reg: &mut Registry) {
    reg.register_macro("to_upper", expand_upper);
    reg.register_macro("to_lower", expand_lower);
}

fn expand_lower<'cx>(cx: &'cx mut ExtCtxt, sp: Span, tts: &[TokenTree])
                        -> Box<base::MacResult + 'cx> {
    expand_cased(cx, sp, tts, |s| { s.to_lowercase() })
}

fn expand_upper<'cx>(cx: &'cx mut ExtCtxt, sp: Span, tts: &[TokenTree])
                        -> Box<base::MacResult + 'cx> {
    expand_cased(cx, sp, tts, |s| { s.to_uppercase() })
}

fn expand_cased<'cx, T>(cx: &'cx mut ExtCtxt, sp: Span, tts: &[TokenTree], transform: T)
                        -> Box<base::MacResult + 'cx>
    where T: Fn(&str) -> String
{
    let es = match base::get_exprs_from_tts(cx, sp, tts) {
        Some(e) => e,
        None => return base::DummyResult::expr(sp)
    };

    let mut it = es.iter();
    let res = match it.next() {
        // FIXME (#13910): nested matches are necessary to get through Gc<>
        Some(expr) => match expr.node {
            ast::ExprKind::Lit(ref lit) => match lit.node {
                ast::LitKind::Str(ref s, _) => Some((s, lit.span)),
                _ => {
                    cx.span_err(expr.span, "expected a string literal");
                    None
                }
            },
            _ => {
                cx.span_err(expr.span, "expected a string literal");
                None
            }
        },
        None => {
            cx.span_err(sp, "expected 1 argument, found 0");
            None
        }
    };
    match (res, it.count()) {
        (Some((s, span)), 0) => {
            let new_s = transform(&s.as_str());
            base::MacEager::expr(cx.expr_str(span, Name::intern(&new_s)))
        }
        (_, rest) => {
            if rest > 0 {
                cx.span_err(sp, &format!("expected 1 argument, found {}", rest+1));
            }
            base::DummyResult::expr(sp)
        }
    }
}
