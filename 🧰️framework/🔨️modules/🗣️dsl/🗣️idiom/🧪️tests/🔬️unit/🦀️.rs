use super::*;

#[derive(Clone, Debug, PartialEq)]
struct GreetAst {
    name: String,
}

struct GreetIdiom;

impl DslIdiom for GreetIdiom {
    const LANG: &'static str = "greet";
    type Ast = GreetAst;

    // 🚫️async: E4 fn-pointer slot — DslIdiom::parse must stay sync, see the trait's own tag.
    fn parse(text: &str) -> Result<Self::Ast, TextError> {
        text.strip_prefix("hello ").map(|name| GreetAst { name: name.trim().to_string() }).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected 'hello <name>'", TextSpan::at(1, 1)))
    }

    // 🚫️async: E4 fn-pointer slot — see parse above
    fn print(ast: &Self::Ast) -> String {
        format!("hello {}", ast.name)
    }

    // 🚫️async: E4 fn-pointer slot — see parse above
    fn classify(_text: &str) -> Vec<(TokenClass, TextSpan)> {
        Vec::new()
    }
}

#[semio_framework_async_macros::async_test]
async fn dsl_idiom_round_trips_through_its_own_parse_and_print() {
    let ast = GreetIdiom::parse("hello world").expect("parse");
    assert_eq!(ast, GreetAst { name: "world".to_string() });
    assert_eq!(GreetIdiom::print(&ast), "hello world");
    assert_eq!(GreetIdiom::parse(&GreetIdiom::print(&ast)), Ok(ast), "idiom round trip law");
}

#[semio_framework_async_macros::async_test]
async fn dsl_idiom_registry_resolves_by_lang_and_canonicalizes_through_the_hooks() {
    register_idiom(hooks_for::<GreetIdiom>());
    let hooks = idiom("greet").expect("registered idiom must be found by its LANG id");
    assert_eq!(hooks.lang, "greet");
    let canonical = (hooks.canonicalize)("hello   world").expect("canonicalize");
    assert_eq!(canonical, "hello world", "canonicalize normalizes through parse -> print");
    assert!((hooks.canonicalize)("not a greeting").is_err(), "a malformed idiom body must surface the idiom's own parse error");
    assert!(idiom("never-registered-lang").is_none(), "an unregistered lang must resolve to None, never a default/error");
}
