mod agent_lane_preview_tests {
    use super::*;

    std::thread_local! {
        static SCRIPTED_CLOCK: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((1_000_000, 1)) };
    }

    /// ⏱️ The harness clock: every read advances it by the case's `clockMicrosPerRead` (0 freezes it).
    fn scripted_now_us() -> Option<u64> {
        SCRIPTED_CLOCK.with(|clock| {
            let (now, stride) = clock.get();
            clock.set((now + stride, stride));
            Some(now)
        })
    }

    enum ScriptedStep {
        Progress,
        Checkpoint,
        Yield,
        Complete,
        Cancelled,
        Fault(Vec<u8>),
    }

    /// 🎭️ A job that reports its script, one entry per step, and repeats the last entry forever.
    struct ScriptedPreviewJob {
        script: Vec<ScriptedStep>,
        cursor: usize,
        closing: bool,
        released: bool,
        outcomes: crate::reserved_job_outcomes::ReservedJobOutcomes,
        pending: Option<(crate::reserved_job_outcomes::ReservedOutcomeKind, Vec<u8>)>,
    }

    impl ScriptedPreviewJob {
        fn new(script: Vec<ScriptedStep>) -> Self {
            Self { script, cursor: 0, closing: false, released: false, outcomes: crate::reserved_job_outcomes::ReservedJobOutcomes::new(), pending: None }
        }
    }

    impl semio_framework_job::InteractiveJob for ScriptedPreviewJob {
        fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
            use crate::reserved_job_outcomes::ReservedOutcomeKind;
            use semio_framework_job::JobOutcomeBorrow;
            if cx.is_cancelled() {
                return JobOutcomeBorrow::admit_cancelled(cx);
            }
            if self.outcomes.is_delivered() {
                self.outcomes.retire(cx)?;
                return Ok(None);
            }
            if let Some((kind, source)) = self.pending.as_ref() {
                let outcome = self.outcomes.advance(*kind, source, cx)?;
                if outcome.is_some() {
                    self.pending = None;
                }
                return Ok(outcome);
            }
            let entry = &self.script[self.cursor.min(self.script.len() - 1)];
            self.cursor += 1;
            self.pending = Some(match entry {
                ScriptedStep::Progress => (ReservedOutcomeKind::Preview, br#"{"en":"Working","de":"Arbeitet"}"#.to_vec()),
                ScriptedStep::Checkpoint => {
                    let applied_progress = self.cursor as u64;
                    (ReservedOutcomeKind::Checkpoint { applied_progress }, applied_progress.to_le_bytes().to_vec())
                }
                ScriptedStep::Yield => return JobOutcomeBorrow::admit_yield(cx),
                ScriptedStep::Complete => (ReservedOutcomeKind::Commit, Vec::new()),
                ScriptedStep::Cancelled => return JobOutcomeBorrow::admit_cancelled(cx),
                ScriptedStep::Fault(detail) => (ReservedOutcomeKind::Fault, detail.clone()),
            });
            Ok(None)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
            self.outcomes.borrow_outcome(descriptor)
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.outcomes.close_demands()?.copy_bytes) }
        fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.outcomes.close_demands()?.capacity_bytes) }
        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.outcomes.close_demands()?.release_bytes) }
        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.outcomes.close_demands()?.depth) }

        fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
            if !self.closing {
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            }
            self.pending = None;
            if !self.outcomes.terminal_is_empty() {
                return crate::reserved_job_outcomes::close_result(self.outcomes.close_step(grant));
            }
            self.released = true;
            semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.released && self.outcomes.terminal_is_empty()
        }
    }

    fn scripted_step(value: &serde_json::Value) -> ScriptedStep {
        match (value.as_str(), value.get("fault")) {
            (Some("progress"), _) => ScriptedStep::Progress,
            (Some("checkpoint"), _) => ScriptedStep::Checkpoint,
            (Some("yield"), _) => ScriptedStep::Yield,
            (Some("complete"), _) => ScriptedStep::Complete,
            (Some("cancelled"), _) => ScriptedStep::Cancelled,
            (None, Some(serde_json::Value::String(detail))) => ScriptedStep::Fault(detail.as_bytes().to_vec()),
            (None, Some(detail @ serde_json::Value::Object(_))) => ScriptedStep::Fault(serde_json::to_vec(detail).expect("typed fault wire")),
            _ => panic!("unknown script entry {value}"),
        }
    }

    /// ⚖️ LAW: every case of the language-agnostic agent-lane preview fixture answers exactly its verdict when its
    /// scripted job runs through the real preview driver (a session the driver could not close would answer
    /// `interactive-job.preview-close` instead).
    #[test]
    fn agent_lane_preview_verdicts_match_the_language_agnostic_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🤖️agent-lane-preview-verdicts.json")).expect("agent-lane preview fixture parses");
        assert_eq!(fixture["budget"]["wallMicros"].as_u64(), Some(AGENT_LANE_PREVIEW_WALL_US), "the fixture states the driver's wall budget");
        assert_eq!(fixture["budget"]["turns"].as_u64(), Some(AGENT_LANE_PREVIEW_TURNS), "the fixture states the driver's turn budget");
        let cases = fixture["cases"].as_array().expect("cases");
        assert!(cases.len() >= 9, "the fixture keeps every verdict's case");
        for case in cases {
            let name = case["name"].as_str().expect("case name");
            let stride = case["clockMicrosPerRead"].as_u64().expect("clock stride");
            SCRIPTED_CLOCK.with(|clock| clock.set((1_000_000, stride)));
            let job = ScriptedPreviewJob::new(case["script"].as_array().expect("script").iter().map(scripted_step).collect());
            let cancel = semio_framework_job::root_cancel_token();
            if case["cancelled"].as_bool() == Some(true) {
                cancel.cancel_now();
            }
            let params = semio_framework_job::BatchJobParams {
                operation: semio_framework_job::allocate_operation_id(),
                generation: semio_framework_job::Generation(1),
                cancel,
                config: semio_framework_job::BatchDriveConfig { retained: crate::app::artifact_app_laws::fixture_mounted_policy().maintenance, site: "agent_lane_preview_law", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1_000, step_budget_us: 7_500 },
                now_us: scripted_now_us,
            };
            let verdict = drive_agent_lane_preview(job, params, crate::app::artifact_app_laws::fixture_mounted_policy().close, name);
            let expected = &case["verdict"];
            match (expected["steps"].as_u64(), &expected["fault"]) {
                (Some(steps), _) => assert_eq!(verdict.as_ref().map_err(|fault| fault.code.0.clone()), Ok(&steps), "{name}"),
                (None, fault) => {
                    let refused = verdict.expect_err(name);
                    assert_eq!(refused.code.0, fault["code"].as_str().expect("fault code"), "{name}: {}", refused.message);
                    if let Some(message) = fault["message"].as_str() {
                        assert_eq!(refused.message, message, "{name}");
                    }
                }
            }
        }
    }

    /// 🚧️ LAW: the agent lane fails closed — every case of the language-agnostic carriage fixture, its host effects decoded
    /// from the kernel `Effect`'s wire form, answers exactly its uncarried lanes and its verdict, and the fixture names
    /// every lane the preview refuses in the order it names them, and the presentation lanes.
    #[test]
    fn agent_lane_carriage_matches_the_language_agnostic_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🤖️agent-lane-carriage.json")).expect("agent-lane carriage fixture parses");
        let lanes = fixture["lanes"].as_array().expect("lanes").iter().map(|lane| lane.as_str().expect("lane name")).collect::<Vec<_>>();
        assert_eq!(lanes, AGENT_LANE_UNCARRIED_LANES, "the fixture names every uncarried lane in refusal order");
        let presentation = fixture["presentation"].as_array().expect("presentation").iter().map(|lane| lane.as_str().expect("lane name")).collect::<Vec<_>>();
        assert_eq!(presentation, AGENT_LANE_PRESENTATION_LANES, "the fixture names the presentation lanes");
        let interaction_verbs = fixture["interactionVerbs"].as_array().expect("interactionVerbs").iter().map(|verb| verb.as_str().expect("verb")).collect::<Vec<_>>();
        assert_eq!(interaction_verbs, INTERACTION_ACTION_IDS, "the fixture names the framework's interaction verbs");
        let cases = fixture["cases"].as_array().expect("cases");
        assert!(cases.len() >= 15, "the fixture keeps every lane family's case");
        for case in cases {
            let name = case["name"].as_str().expect("case name");
            let publication = &case["publication"];
            let effects = serde_json::from_value::<Vec<Effect>>(publication["effects"].clone()).expect(name);
            let count = |key: &str| publication[key].as_u64().expect(key) as usize;
            let publication = AgentLanePublication {
                carried_ops: count("carriedOps"),
                effects: &effects,
                window_config_ops: count("windowConfigOps"),
                extension_calls: count("extensionCalls"),
                events: count("events"),
                selection_writes: count("selectionWrites"),
                tasks: count("tasks"),
                presence: count("presence"),
                transient: count("transient"),
                window_transient: count("windowTransient"),
            };
            let expected = case["uncarried"].as_array().expect("uncarried").iter().map(|lane| lane.as_str().expect("lane")).collect::<Vec<_>>();
            assert_eq!(agent_lane_uncarried_lanes(&publication), expected, "{name}");
            let verdict = match agent_lane_carriage(&publication) {
                AgentLaneCarriage::Carried => "carried",
                AgentLaneCarriage::NoEffect => "noEffect",
                AgentLaneCarriage::Uncarried(_) => "uncarried",
            };
            assert_eq!(Some(verdict), case["verdict"].as_str(), "{name}");
        }
    }
}
