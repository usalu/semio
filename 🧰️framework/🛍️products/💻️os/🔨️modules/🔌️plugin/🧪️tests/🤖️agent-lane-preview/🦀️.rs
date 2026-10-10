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
    }

    impl ScriptedPreviewJob {
        fn payload(cx: &mut semio_framework_job::StepContext<'_>, stream: semio_framework_job::JobPayloadStream, bytes: &[u8]) -> semio_framework_job::RetainedJobPayload {
            cx.payload_from_bytes(stream, bytes).unwrap_or_else(|rejected| {
                drop(rejected.into_source());
                semio_framework_job::RetainedJobPayload::empty(stream)
            })
        }
    }

    impl semio_framework_job::InteractiveJob for ScriptedPreviewJob {
        fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
            if cx.is_cancelled() {
                return semio_framework_job::StepOutcome::Cancelled;
            }
            let entry = &self.script[self.cursor.min(self.script.len() - 1)];
            self.cursor += 1;
            match entry {
                ScriptedStep::Progress => semio_framework_job::StepOutcome::PreviewReady(Self::payload(cx, semio_framework_job::JobPayloadStream::Preview, br#"{"en":"Working","de":"Arbeitet"}"#)),
                ScriptedStep::Checkpoint => {
                    let applied_progress = self.cursor as u64;
                    semio_framework_job::StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: Self::payload(cx, semio_framework_job::JobPayloadStream::CheckpointState, &applied_progress.to_le_bytes()), applied_progress })
                }
                ScriptedStep::Yield => semio_framework_job::StepOutcome::Yield,
                ScriptedStep::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                }),
                ScriptedStep::Cancelled => semio_framework_job::StepOutcome::Cancelled,
                ScriptedStep::Fault(detail) => semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: Self::payload(cx, semio_framework_job::JobPayloadStream::Fault, detail) }),
            }
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
        fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }

        fn close_step(&mut self, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
            if !self.closing {
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            }
            self.released = true;
            semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.released
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
            let job = ScriptedPreviewJob { script: case["script"].as_array().expect("script").iter().map(scripted_step).collect(), cursor: 0, closing: false, released: false };
            let cancel = semio_framework_job::root_cancel_token();
            if case["cancelled"].as_bool() == Some(true) {
                cancel.cancel_now();
            }
            let params = semio_framework_job::BatchJobParams {
                operation: semio_framework_job::allocate_operation_id(),
                generation: semio_framework_job::Generation(1),
                cancel,
                config: semio_framework_job::BatchDriveConfig { work_grant: semio_framework_job::retained_work::NO_RETAINED_WORK, site: "agent_lane_preview_law", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1_000, step_budget_us: 7_500 },
                now_us: scripted_now_us,
            };
            let verdict = drive_agent_lane_preview(job, params, name);
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
