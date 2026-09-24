use super::*;

fn projection_of(source: &str) -> String {
    serde_json::to_string(&parse(source).expect("parses").projection()).expect("serialises")
}

#[test]
fn projects_the_canonical_shape() {
    assert_eq!(projection_of("{ repo }"), r#"{"operation":"query","selections":[{"alias":null,"arguments":[],"fields":[],"name":"repo"}]}"#);
    assert_eq!(projection_of("mutation Open { ticketOpen }"), r#"{"operation":"mutation","selections":[{"alias":null,"arguments":[],"fields":[],"name":"ticketOpen"}]}"#);
    assert_eq!(projection_of("{ first: repo }"), r#"{"operation":"query","selections":[{"alias":"first","arguments":[],"fields":[],"name":"repo"}]}"#);
    assert_eq!(projection_of("{ t(v: $id) }"), r#"{"operation":"query","selections":[{"alias":null,"arguments":[{"name":"v","value":{"kind":"variable","name":"id"}}],"fields":[],"name":"t"}]}"#);
    assert_eq!(projection_of("query Q($id: ID!) { t @include(if: true) }"), r#"{"operation":"query","selections":[{"alias":null,"arguments":[],"fields":[],"name":"t"}]}"#);
}

#[test]
fn reproduces_the_reference_diagnostics() {
    let cases: &[(&str, &str)] = &[
        ("", r#"expected "{" at 0, got """#),
        ("{", "unterminated selection set"),
        ("{ 1 }", "expected field name at 2"),
        ("{ a(1: 2) }", "expected argument name at 4"),
        ("{ a(b) }", r#"expected ":" at 5, got ")""#),
        ("{ a: }", "expected aliased field name"),
        ("{ a(b: $) }", "expected variable name"),
        ("{ a } extra", r#"unexpected token "extra" at 6"#),
        ("{ a(b: \"unterminated) }", "invalid value at 0"),
        ("{ a(b: {1: 2}) }", "expected object field"),
        ("{ a. }", "unterminated selection set"),
        ("fragment F on T { a }", r#"expected "{" at 0, got "fragment""#),
        ("{ ... on T { a } }", "unterminated selection set"),
    ];
    for (source, expected) in cases {
        let error = parse(source).expect_err(source);
        assert_eq!(&error.message, expected, "source {source:?}");
    }
}

#[test]
fn applies_defaults_only_to_absent_arguments() {
    let document = parse("{ tickets(state: $state, limit: $limit) }").expect("parses");
    let defaults: Map<String, Json> = serde_json::from_value(json!({ "state": "open", "limit": 10, "cursor": "start" })).expect("defaults");
    let variables: Map<String, Json> = serde_json::from_value(json!({ "limit": 5 })).expect("variables");
    let args = coerce_arguments(&document.selections[0].arguments, &defaults, &variables);
    assert_eq!(args["limit"], json!(5));
    assert_eq!(args["state"], Json::Null);
    assert_eq!(args["cursor"], json!("start"));
}

#[test]
fn round_trips_the_ast_through_json() {
    let document = parse("{ a(b: [1, {c: \"d\"}]) { e } }").expect("parses");
    let encoded = serde_json::to_string(&document).expect("encodes");
    let decoded: Document = serde_json::from_str(&encoded).expect("decodes");
    assert_eq!(decoded, document);
}

/// 🗄️ A minimal record set every executor test runs against.
fn records() -> RecordingContext {
    RecordingContext::from_text(
        r#"{
          "rootDir": "/repo",
          "now": "2026-09-06 08:00:00",
          "technologies": [{"name": "repo", "root": "/repo", "kind": "🧰️", "emoji": "", "bundles": []}],
          "bundles": [{"name": "repo/go", "root": "repo/go", "technologyName": "repo", "kind": "library", "emoji": ""}],
          "statutes": [{"kind": "code/section/empty", "policyId": "repo/lint", "priority": "high", "reason": "Empty section", "solution": "Remove it", "autofixable": true}],
          "tickets": [{"year": 2026, "month": 9, "day": 6, "slug": "PORT-EXECUTOR", "title": "Port Executor", "status": "open", "goal": "repo",
                       "folderPath": "tickets/2026/09/06/PORT-EXECUTOR",
                       "interactions": [{"kind": "ticket.open", "date": "2026-09-06 08:00:00", "author": "Ueli <ueli@semio-tech.com>", "system": "cli", "client": "claude-code", "checkpoint": "abc", "prompt": "port it", "llm": "opus", "effort": "high"}]}],
          "goals": [{"id": "repo", "title": "Repo", "description": "d", "prompt": "p", "status": "open", "client": "claude-code", "llm": "opus", "dates": {"due": "2026-12-31"}}]
        }"#,
    )
    .expect("records")
}

#[test]
fn serves_the_committed_sdl() {
    assert_eq!(render_sdl(&build_schema()), include_str!("../../🧫️fixtures/📜️served-schema/🔗️.graphql"));
}

#[test]
fn projects_scalars_lists_and_enums() {
    let context = records();
    let executor = Executor::new(&context);
    assert_eq!(
        executor.execute_json("{ repo { id name path } }", &Map::new()).expect("executes"),
        "{\n  \"repo\": {\n    \"id\": \"repo:compose\",\n    \"name\": \"compose\",\n    \"path\": \"/repo\"\n  }\n}"
    );
    assert_eq!(
        executor.execute_json("{ tickets { slug status client } }", &Map::new()).expect("executes"),
        "{\n  \"tickets\": [\n    {\n      \"client\": \"CLAUDE_CODE\",\n      \"slug\": \"PORT-EXECUTOR\",\n      \"status\": \"OPEN\"\n    }\n  ]\n}"
    );
}

#[test]
fn honours_aliases_typename_and_arguments() {
    let context = records();
    let executor = Executor::new(&context);
    assert_eq!(
        executor.execute_json("{ first: tickets(year: 2026, status: OPEN) { __typename } }", &Map::new()).expect("executes"),
        "{\n  \"first\": [\n    {\n      \"__typename\": \"Ticket\"\n    }\n  ]\n}"
    );
    assert_eq!(executor.execute_json("{ tickets(year: 2025) { slug } }", &Map::new()).expect("executes"), "{\n  \"tickets\": []\n}");
}

#[test]
fn derives_the_ticket_dates_and_identity() {
    let context = records();
    let executor = Executor::new(&context);
    let rendered = executor.execute_json("{ ticket(year: 2026, month: 9, day: 6, slug: \"PORT-EXECUTOR\") { id dates { started finished } } }", &Map::new()).expect("executes");
    assert_eq!(rendered, "{\n  \"ticket\": {\n    \"dates\": {\n      \"finished\": null,\n      \"started\": \"2026-09-06T08:00:00Z\"\n    },\n    \"id\": \"🎫portexecutor\"\n  }\n}");
}

#[test]
fn reports_unknown_fields_and_unresolvable_ids() {
    let context = records();
    let executor = Executor::new(&context);
    assert_eq!(executor.execute("{ nope }", &Map::new()).expect_err("fails").message, "graphql errors: [unknown field \"nope\" on Query]");
    assert_eq!(executor.execute("{ repo { nope } }", &Map::new()).expect_err("fails").message, "graphql errors: [repo: unknown field \"nope\" on Repo]");
    assert_eq!(executor.execute("{ node(id: \"nope\") { __typename } }", &Map::new()).expect_err("fails").message, "graphql errors: [node: invalid node id format: nope]");
    assert_eq!(executor.execute("{ repo", &Map::new()).expect_err("fails").message, "graphql errors: [unterminated selection set]");
}

#[test]
fn records_every_mutation_it_applies() {
    let context = records();
    let rendered = Executor::new(&context)
        .execute_json("mutation { ticketClose(input: {year: 2026, month: 9, day: 6, slug: \"PORT-EXECUTOR\", summary: \"done\"}) { status } }", &Map::new())
        .expect("executes");
    assert_eq!(rendered, "{\n  \"ticketClose\": {\n    \"status\": \"CLOSED\"\n  }\n}");
    let events = context.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["kind"], json!("ticket.close"));
    assert_eq!(events[0]["payload"]["summary"], json!("done"));
}

#[test]
fn resolves_a_statute_through_a_breach() {
    let context = RecordingContext::from_text(
        r#"{
          "rootDir": "/repo",
          "statutes": [{"kind": "code/section/empty", "policyId": "repo/lint", "priority": "high", "reason": "Empty section", "solution": "Remove it", "autofixable": true}],
          "analyze": {"breachs": [{"id": "b1", "summary": "empty", "kind": "code/section/empty", "scope": "a.rs"}], "metrics": {"total": 1, "byPriority": {"high": 1, "medium": 0, "low": 0}, "autofixable": 1}}
        }"#,
    )
    .expect("records");
    let rendered = Executor::new(&context).execute_json("{ analyze { metrics { total } breachs { kindId priority autofixable kind { id reason } } } }", &Map::new()).expect("executes");
    assert_eq!(
        rendered,
        "{\n  \"analyze\": {\n    \"breachs\": [\n      {\n        \"autofixable\": true,\n        \"kind\": {\n          \"id\": \"Code#Section#Empty\",\n          \"reason\": \"Empty section\"\n        },\n        \"kindId\": \"code/section/empty\",\n        \"priority\": \"HIGH\"\n      }\n    ],\n    \"metrics\": {\n      \"total\": 1\n    }\n  }\n}"
    );
}

#[test]
fn reports_the_operation_type() {
    assert_eq!(operation_type("{ a }").expect("query"), "query");
    assert_eq!(operation_type("mutation { a }").expect("mutation"), "mutation");
    assert!(validate("{").is_err());
}
