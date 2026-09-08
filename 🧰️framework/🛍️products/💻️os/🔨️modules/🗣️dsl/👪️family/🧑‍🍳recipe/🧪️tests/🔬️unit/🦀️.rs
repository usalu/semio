use super::*;

#[semio_framework_async_macros::async_test]
async fn parses_a_typed_call_step_with_a_dotted_target() {
    let step = parse_step_text("step-1: state.set(counter 0)").await.expect("parse_step_text");
    assert_eq!(step, RecipeStep { name: "step-1".to_string(), target: "state.set".to_string(), args: vec!["counter".to_string(), "0".to_string()] });
    assert_eq!(print_step(&step).await, "step-1: state.set(counter 0)");
}

#[semio_framework_async_macros::async_test]
async fn parses_a_step_with_no_args() {
    let step = parse_step_text("step-2: state.reset()").await.expect("parse_step_text");
    assert_eq!(step.args, Vec::<String>::new());
    assert_eq!(print_step(&step).await, "step-2: state.reset()");
}

#[semio_framework_async_macros::async_test]
async fn parses_a_step_with_a_text_argument() {
    let step = parse_step_text("step-3: log.write(\"hello world\")").await.expect("parse_step_text");
    assert_eq!(step.args, vec!["\"hello world\"".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn rejects_a_missing_colon() {
    let err = parse_step_text("step-1 state.set(counter 0)").await.unwrap_err();
    assert!(err.message.contains("Colon"), "unexpected message: {}", err.message);
}

#[semio_framework_async_macros::async_test]
async fn rejects_trailing_content() {
    assert!(parse_step_text("step-1: state.set(counter 0) extra").await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn round_trip_matrix() {
    let sources = vec!["step-1: state.set(counter 0)", "step-2: state.reset()", "step-3: math.add(1 2 3)"];
    for source in sources {
        let step = parse_step_text(source).await.unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
        let printed = print_step(&step).await;
        assert_eq!(printed, source);
        let reparsed = parse_step_text(&printed).await.unwrap_or_else(|e| panic!("reparse of {printed:?} failed: {e:?}"));
        assert_eq!(reparsed, step);
    }
}

/// @emoji 📖️ The fragment's `.grammar` file must at least parse under `dsl_grammar`'s parser.
#[semio_framework_async_macros::async_test]
async fn grammar_file_is_syntactically_valid() {
    let source = include_str!("../../📖️.grammar.semio");
    let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-recipe.grammar must parse");
    assert_eq!(grammar.id, "family-recipe");
    assert!(grammar.productions.len() > 4, "family-recipe should cover named and positional args");
}
