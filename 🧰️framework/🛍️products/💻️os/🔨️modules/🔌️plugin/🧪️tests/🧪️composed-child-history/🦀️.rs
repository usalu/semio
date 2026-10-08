//! 🧩️ Composed-child history laws of the plugin runtime (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12), driven
//! by the language-agnostic fixture `🧫️fixtures/🧫️composed-child-history/🔣️.json` whose cases the TypeScript twin (`🟦️.ts`
//! beside this file) derives independently: a reload backfills every unlogged member edit — joined to the parent row its
//! transaction also landed, else one row per transaction (or per edit without one), in moment order, labelled from its
//! declared intent or first leaf.

use super::*;
use std::collections::{HashMap, HashSet};

const COMPOSED_CHILD_HISTORY_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🧫️composed-child-history/🔣️.json");

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, got {value}"))
}

fn label(value: &Value) -> LocalizedLabel {
    LocalizedLabel::native(text(&value["en"]), text(&value["de"]))
}

fn resolved(label: &LocalizedLabel) -> (String, String) {
    (label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string(), label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_string())
}

/// 🧩️ One fixture history as the member history visitor reads it.
fn history(value: &Value) -> time_travel::MemberEditHistory {
    let store = text(&value["store"]).to_string();
    let mutations = value.get("firstLabel").map(|first| MutationView {
        mutation_id: format!("{}:0", text(&value["editId"])),
        position: 0,
        op_index: 0,
        label: label(first),
        worst: None,
        messages: Vec::new(),
        superseded: false,
        withdrawn: false,
        editable: true,
        withdrawable: true,
        store: Some(store.clone()),
    });
    time_travel::MemberEditHistory {
        store,
        edit_id: text(&value["editId"]).to_string(),
        started_at: text(&value["startedAt"]).to_string(),
        timestamp: Some(protocol::HybridLogicalTimestamp { actor: 0, physical_ms: value["at"].as_u64().expect("moment"), logical: 0 }),
        transaction: value["transaction"].as_str().map(|id| protocol::TransactionRef { id: id.to_string(), tool: "s.test@1/*#editor#drag".into() }),
        intent_label: value.get("intentLabel").map(label),
        op_count: value["opCount"].as_u64().expect("operation count") as usize,
        op_lines: value["opLines"].as_array().expect("operation lines").iter().map(|line| text(line).to_string()).collect(),
        mutations: mutations.into_iter().collect(),
    }
}

/// ⚖️ LAW: every fixture case backfills exactly its expected attachments and rows, in moment order, labelled in English and
/// German from declared intent, the first leaf or the first printed operation.
#[test]
fn member_backfill_answers_every_fixture_case() {
    let fixture: Value = serde_json::from_str(COMPOSED_CHILD_HISTORY_FIXTURE_JSON).expect("composed-child history fixture parses");
    for case in fixture["cases"].as_array().expect("cases") {
        let id = text(&case["id"]);
        let histories: Vec<time_travel::MemberEditHistory> = case["histories"].as_array().expect("histories").iter().map(history).collect();
        let logged: HashSet<&str> = case["logged"].as_array().expect("logged").iter().map(text).collect();
        let parents: HashMap<String, String> = case["parentTransactions"].as_object().expect("parent transactions").iter().map(|(transaction, edit)| (transaction.clone(), text(edit).to_string())).collect();
        let backfill = time_travel::member_backfill(histories, &logged, &parents);
        let expected = &case["expected"];
        let attached: HashMap<String, Vec<String>> = expected["attached"].as_object().expect("attached").iter().map(|(parent, edits)| (parent.clone(), edits.as_array().expect("edits").iter().map(|edit| text(edit).to_string()).collect())).collect();
        assert_eq!(backfill.attached, attached, "{id}: attachments");
        let groups = expected["groups"].as_array().expect("groups");
        assert_eq!(backfill.groups.len(), groups.len(), "{id}: row count {:?}", backfill.groups);
        for (group, expected) in backfill.groups.iter().zip(groups) {
            assert_eq!(group.at.map(|at| at.physical_ms), expected["at"].as_u64(), "{id}: moment");
            assert_eq!(group.transaction.as_deref(), expected["transaction"].as_str(), "{id}: transaction");
            assert_eq!(group.edit_ids, expected["editIds"].as_array().expect("edit ids").iter().map(|edit| text(edit).to_string()).collect::<Vec<_>>(), "{id}: edits");
            assert_eq!(resolved(&group.label), resolved(&label(&expected["label"])), "{id}: label");
            assert_eq!(group.started_at, text(&expected["startedAt"]), "{id}: start");
        }
        eprintln!("[DEBUG] composed-child-history neutral case={id} bilingual-labels=true");
    }
}
