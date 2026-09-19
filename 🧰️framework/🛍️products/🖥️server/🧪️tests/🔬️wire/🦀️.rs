//! 🔌️ The Rust half of the server product's cross-language wire gate.
//!
//! Both twins are held against one document — `🧫️fixtures/🔌️wire/🔣️.json` — instead of against each
//! other: this target deserializes every vector into the contract type its `type` names and
//! serializes it straight back, asserting the result is byte-identical to the fixture, and
//! `🟦️.ts`'s own suite does the same against the same file. A serde rename, an added field or a
//! dropped `Option` therefore fails *both* suites, which is the only arrangement under which two
//! implementations of one contract can be said to agree.
//!
//! It also pins the route table. `base_router` is the single place the gateway's own twelve `route`
//! calls live, and a typed client whose route list is a *copy* of that list silently 404s one
//! release later, so the fixture's table is checked against the gateway source itself here and in
//! TypeScript.

use serde::Serialize;
use serde_json::Value;

use server::contract::{CommandEnvelope, CommandOutcome, CommandReceipt, EphemeralFrame, EventRecord, HybridLogicalClock, Principal, QueryConsistency, QueryEnvelope, QueryResult, Rejection, ServerInstanceDefinition, TraceContext};
use protocol::causal::FrontierSummary;
use server::contract::ActorKey;

/// 🧫️ The shared contract document, read from the product root rather than copied into this target.
const FIXTURE: &str = include_str!("../../🧫️fixtures/🔌️wire/🔣️.json");

/// 🛣️ The gateway source, so the declared route table is checked against the router that exists.
const GATEWAY: &str = include_str!("../../🔨️modules/📡️gateway/🦀️.rs");

/// 🔁️ Deserialize one vector into `T` and serialize it back, asserting nothing moved.
fn round_trip<T>(name: &str, json: &Value)
where
    T: serde::de::DeserializeOwned + Serialize,
{
    let value: T = serde_json::from_value(json.clone()).unwrap_or_else(|error| panic!("{name}: {error}"));
    assert_eq!(&serde_json::to_value(&value).expect("the contract type serializes"), json, "{name}");
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the wire fixture is valid JSON")
}

#[test]
fn every_wire_vector_round_trips_through_its_contract_type() {
    let document = fixture();
    assert_eq!(document["schema"], "semio.framework.server.wire/v1");
    let vectors = document["vectors"].as_array().expect("vectors is an array").clone();
    assert!(!vectors.is_empty());

    let mut covered: Vec<String> = Vec::new();
    for vector in &vectors {
        let name = vector["name"].as_str().expect("every vector is named");
        let kind = vector["type"].as_str().expect("every vector names its type");
        let json = &vector["json"];
        match kind {
            "actorKey" => round_trip::<ActorKey>(name, json),
            "principal" => round_trip::<Principal>(name, json),
            "hybridLogicalClock" => round_trip::<HybridLogicalClock>(name, json),
            "traceContext" => round_trip::<TraceContext>(name, json),
            "frontierSummary" => round_trip::<FrontierSummary>(name, json),
            "eventRecord" => round_trip::<EventRecord>(name, json),
            "ephemeralFrame" => round_trip::<EphemeralFrame>(name, json),
            "commandEnvelope" => round_trip::<CommandEnvelope>(name, json),
            "commandReceipt" => round_trip::<CommandReceipt>(name, json),
            "rejection" => round_trip::<Rejection>(name, json),
            "commandOutcome" => round_trip::<CommandOutcome>(name, json),
            "queryConsistency" => round_trip::<QueryConsistency>(name, json),
            "queryEnvelope" => round_trip::<QueryEnvelope>(name, json),
            "queryResult" => round_trip::<QueryResult>(name, json),
            "serverInstanceDefinition" => round_trip::<ServerInstanceDefinition>(name, json),
            other => panic!("{name}: the fixture names a wire type this crate does not serve: {other}"),
        }
        covered.push(kind.to_string());
    }

    covered.sort();
    covered.dedup();
    assert_eq!(covered.len(), 15, "every contract type carried by a route needs at least one vector");
}

#[test]
fn vector_names_are_unique_so_a_failure_names_one_document() {
    let document = fixture();
    let mut names: Vec<String> = document["vectors"].as_array().expect("vectors").iter().map(|vector| vector["name"].as_str().expect("named").to_string()).collect();
    let total = names.len();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), total);
}

#[test]
fn the_declared_route_table_is_the_router_the_gateway_mounts() {
    let document = fixture();
    let declared: Vec<(String, String)> = document["routes"]
        .as_array()
        .expect("routes is an array")
        .iter()
        .map(|route| (route["method"].as_str().expect("method").to_string(), route["path"].as_str().expect("path").to_string()))
        .collect();

    let body = GATEWAY.split_once("fn base_router<I: ServerInstance>").expect("the gateway declares base_router").1;
    let block = body.split_once("\n}").expect("base_router has a body").0;
    let mut mounted: Vec<(String, String)> = Vec::new();
    for line in block.lines().filter(|line| line.contains(".route(\"")) {
        let path = line.split_once(".route(\"").expect("a route call").1.split_once('"').expect("a quoted path").0.to_string();
        let verbs = line.split_once(&format!("{path}\"")).expect("the verbs follow the path").1;
        for verb in ["get", "post", "put", "head"] {
            let mut rest = verbs;
            while let Some((before, after)) = rest.split_once(&format!("{verb}(")) {
                if !before.ends_with(|character: char| character.is_alphanumeric() || character == '_') {
                    mounted.push((verb.to_uppercase(), path.clone()));
                }
                rest = after;
            }
        }
    }

    assert_eq!(mounted.len(), declared.len(), "the fixture and base_router disagree on how many routes exist: {mounted:?}");
    for entry in &declared {
        assert!(mounted.contains(entry), "base_router does not mount {entry:?}");
    }
}
