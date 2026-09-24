/// 🕹️ The NATIVE twin of the wgpu browser bridge's `wgpuInvocationFromFrames`: a framework-reserved
/// interaction verb is admitted with an empty result and answered on a LATER turn by a frame that
/// names no command sequence, and this target's dispatch must fold both.
///
/// Fixture: `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/🕹️reserved-verb-answer/🔣️.json` — the same file the
/// TypeScript law `🧪️tests/🕹️wgpu-selection-roundtrip/🟦️.ts` answers, so the two hosts are pinned to one
/// contract rather than to each other's code.
mod reserved_verb_answer_tests {
    use super::*;

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎭️actor/🧫️fixtures/🕹️reserved-verb-answer/🔣️.json")).expect("reserved verb answer fixture parses")
    }

    /// 📦️ One `AppFrame::Invocation` exactly as the fixture's `admission`/`settled` half describes it.
    fn invocation_frame(half: &serde_json::Value) -> AppFrame {
        let output = match &half["output"] {
            serde_json::Value::Null => Vec::new(),
            value => encode_wire(&dsl::json::from_json_str::<DslValue>(&value.to_string()).expect("fixture output is a dsl value")).expect("fixture output encodes"),
        };
        let scope = half["uiScope"].clone();
        let ui_scope = match scope["kind"].as_str().expect("fixture scope kind") {
            "none" => semio_framework::kernel::UiDirtyScope::None,
            "full" => semio_framework::kernel::UiDirtyScope::Full,
            _ => semio_framework::kernel::UiDirtyScope::Partial {
                window_bodies: scope["windowBodies"].as_array().map(|rows| rows.iter().map(|row| row.as_str().expect("body").to_string()).collect()).unwrap_or_default(),
                panel_bodies: scope["panelBodies"].as_array().map(|rows| rows.iter().map(|row| row.as_str().expect("body").to_string()).collect()).unwrap_or_default(),
                utilities: scope["utilities"].as_bool().unwrap_or(false),
                tools: scope["tools"].as_bool().unwrap_or(false),
                engagements: scope["engagements"].as_bool().unwrap_or(false),
                measures: scope["measures"].as_bool().unwrap_or(false),
                labels: scope["labels"].as_bool().unwrap_or(false),
            },
        };
        AppFrame::Invocation {
            in_reply_to: half["replySequence"].as_u64().expect("fixture reply sequence"),
            output,
            diagnostics: Vec::new(),
            ui_scope: encode_wire(&ui_scope).expect("fixture scope encodes"),
            history_patch: Vec::new(),
            messages: Vec::new(),
            mutations: Vec::new(),
            inverse_group: Vec::new(),
        }
    }

    fn outcome(frames: Vec<AppFrame>) -> crate::kernel_runtime::ExchangeOutcome {
        crate::kernel_runtime::ExchangeOutcome { frames, surfaces: Default::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() }
    }

    fn selected_ids(result: &InvocationResult) -> Vec<String> {
        result
            .output
            .get("interactionView")
            .and_then(|view| view.get("selectedIds"))
            .and_then(DslValue::as_array)
            .map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_owned)).collect())
            .unwrap_or_default()
    }

    fn scope_kind(scope: &semio_framework::kernel::UiDirtyScope) -> &'static str {
        match scope {
            semio_framework::kernel::UiDirtyScope::Full => "full",
            semio_framework::kernel::UiDirtyScope::None => "none",
            semio_framework::kernel::UiDirtyScope::Partial { .. } => "partial",
        }
    }

    #[test]
    fn a_reserved_verbs_settled_answer_reaches_the_native_shell() {
        let fixture = fixture();
        let sequence = fixture["commandSequence"].as_u64().expect("command sequence");
        assert_eq!(fixture["admission"]["replySequence"].as_u64(), Some(sequence), "the admission answers the caller");
        assert_eq!(fixture["settled"]["replySequence"].as_u64(), Some(UNCORRELATED_REPLY_SEQUENCE), "the settled answer names no caller");

        let mut folded = outcome(vec![invocation_frame(&fixture["admission"]), invocation_frame(&fixture["settled"])]);
        let result = invocation_from_frames(&mut folded, sequence).expect("the dispatch resolves");
        let expect = &fixture["expect"];
        assert_eq!(scope_kind(&result.ui_scope), expect["foldedUiScopeKind"].as_str().expect("folded scope kind"));
        assert_eq!(selected_ids(&result), expect["foldedSelectedIds"].as_array().expect("folded ids").iter().map(|id| id.as_str().expect("id").to_string()).collect::<Vec<_>>());
        let semio_framework::kernel::UiDirtyScope::Partial { window_bodies, .. } = &result.ui_scope else { panic!("the folded scope is the app-declared partial one") };
        assert_eq!(*window_bodies, expect["foldedWindowBodies"].as_array().expect("window bodies").iter().map(|body| body.as_str().expect("body").to_string()).collect::<Vec<_>>());
    }

    #[test]
    fn folding_the_admission_alone_reproduces_the_defect_this_law_closes() {
        let fixture = fixture();
        let sequence = fixture["commandSequence"].as_u64().expect("command sequence");
        let mut admission_only = outcome(vec![invocation_frame(&fixture["admission"])]);
        let result = invocation_from_frames(&mut admission_only, sequence).expect("the dispatch resolves");
        let expect = &fixture["expect"];
        assert_eq!(scope_kind(&result.ui_scope), expect["frameLaneOnlyUiScopeKind"].as_str().expect("frame-lane scope kind"), "an admission alone refreshes nothing");
        assert!(selected_ids(&result).is_empty(), "an admission alone carries no interaction view");
    }

    #[test]
    fn an_uncorrelated_answer_never_satisfies_the_sequence_the_caller_waits_on() {
        let fixture = fixture();
        let sequence = fixture["commandSequence"].as_u64().expect("command sequence");
        let mut settled_only = outcome(vec![invocation_frame(&fixture["settled"])]);
        let error = invocation_from_frames(&mut settled_only, sequence).expect_err("a frame that answers no caller cannot resolve a call");
        assert_eq!(error, format!("plugin sent no Invocation for seq {sequence}"));
    }
}
