    /// 🧭️ Entity id arguments, introductions and tutorials: the entity id granularity argument and every introduction and tutorial declaration law.

    /// 🎯️ LAW: an entity-id argument names a granularity of an interaction its app declares — the selection that verb
    /// otherwise reads — and publishes each id as `entityId` of that kind; one naming an undeclared interaction or
    /// granularity is refused by name when the definition is built.
    #[semio_framework_async_macros::async_test]
    async fn an_entity_id_argument_names_a_granularity_of_a_declared_interaction() {
        use semio_framework::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
        let interaction = || InteractionDefinition {
            id: "graph".into(),
            label: LocalizedLabel::data("Graph"),
            granularities: vec![GranularityDefinition { id: "node".into(), label: LocalizedLabel::data("Node"), icon_id: "circle".into() }],
            hierarchy: HierarchyProvider::Flat,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Multiple], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
        };
        let built = |slug: &'static str, interaction_id: &'static str, granularity_id: &'static str| async move {
            minimal_app(slug)
                .await
                .interaction(interaction())
                .await
                .mutation("rename", LocalizedLabel::data("Rename"))
                .await
                .action_args("rename", vec![ActionArgDef::entity_ids("nodeIds", LocalizedLabel::data("Nodes"), interaction_id, granularity_id)])
                .await
                .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
                .await
                .try_build_definition()
        };
        let definition = built("entity-ids-app", "graph", "node").await.expect("a declared granularity builds");
        let rename = declared_actions(&definition).find(|action| action.id == "rename").expect("declared");
        let schema = rename.args[0].json_schema(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En);
        let items = schema.get("items").expect("an entity-id argument is a list");
        assert_eq!(
            (schema.get("type").and_then(DslValue::as_str), items.get("type").and_then(DslValue::as_str), items.get("x-semio-format").and_then(DslValue::as_str), items.get("x-semio-entity-kind").and_then(DslValue::as_str)),
            (Some("array"), Some("string"), Some("entityId"), Some("graph/node"))
        );
        for (slug, interaction_id, granularity_id) in [("entity-ids-undeclared-interaction", "ast", "node"), ("entity-ids-undeclared-granularity", "graph", "edge")] {
            let error = built(slug, interaction_id, granularity_id).await.expect_err(slug);
            assert_eq!(error.code, "app-definition.invalid", "{slug}");
            assert!(error.message.contains(&format!("{interaction_id}/{granularity_id}")), "{slug}: {}", error.message);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_introduction_injects_start_introduction_action() {
        use semio_framework::ActionKind;
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionStepDefinition, START_INTRODUCTION_ACTION_ID};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let definition = minimal_app("intro-app")
            .await
            .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("welcome", LocalizedLabel::data("Welcome"), LocalizedLabel::data("Hi there"))] })
            .await
            .build_definition();
        let start_introduction = declared_actions(&definition).find(|action| action.id == START_INTRODUCTION_ACTION_ID).expect("startIntroduction injected");
        assert_eq!(start_introduction.kind, ActionKind::View);
        assert!(!start_introduction.in_palette, "the shell-owned Introduce App command owns palette discovery");
    }

    #[semio_framework_async_macros::async_test]
    async fn no_introduction_means_no_start_introduction_action() {
        use semio_framework::onboarding::START_INTRODUCTION_ACTION_ID;
        let definition = minimal_app("no-intro-app").await.build_definition();
        assert!(!declared_actions(&definition).any(|action| action.id == START_INTRODUCTION_ACTION_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_with_no_steps() {
        use semio_framework::onboarding::IntroductionDefinition;
        let __base = minimal_app("empty-intro-app").await;
        let __chain = __base.introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![] }).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_duplicate_introduction_step_ids() {
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionStepDefinition};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let __base = minimal_app("dupe-step-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")), IntroductionStepDefinition::new("step", LocalizedLabel::data("B"), LocalizedLabel::data("b"))],
            })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_introducing_undeclared_window_kind() {
        use semio_framework::window_element_id;
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionStepDefinition};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let __base = minimal_app("bad-window-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(window_element_id("missing"))] })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_introducing_undeclared_panel_tab() {
        use semio_framework::{panel_tab_element_id, panel_tab_first_draggable_element_id};
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionStepDefinition};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let __base = minimal_app("bad-panel-tab-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(panel_tab_element_id("missing"))] })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
        let __base = minimal_app("bad-panel-tab-first-draggable-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce(panel_tab_first_draggable_element_id("missing"))],
            })
            .await;
        let result_first_draggable = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result_first_draggable.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_targeting_malformed_element_id() {
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionStepDefinition};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let __base = minimal_app("bad-element-app").await;
        let __chain =
            __base.introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce("not-camel-case")] }).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
        let __base = minimal_app("bad-element-show-app").await;
        let __chain =
            __base.introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).show(vec!["not-camel-case".into()])] }).await;
        let result_show = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result_show.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_introduction_step_introducing_escape_hatch_element_id() {
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionStepDefinition};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let definition = minimal_app("good-escape-hatch-app")
            .await
            .introduction(IntroductionDefinition { title: LocalizedLabel::data("Welcome"), steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).introduce("ui.custom.thing")] })
            .await
            .build_definition();
        let introduction = definition.introduction.expect("introduction present");
        assert_eq!(introduction.steps.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_interacting_on_undeclared_utility() {
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let __base = minimal_app("bad-interaction-utility-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).interact(vec![IntroductionInteraction::utility("missing", "Activate").await])],
            })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_introduction_step_interacting_on_undeclared_window_kind() {
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let __base = minimal_app("bad-interaction-window-app").await;
        let __chain = __base
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![IntroductionStepDefinition::new("step", LocalizedLabel::data("A"), LocalizedLabel::data("a")).interact(vec![IntroductionInteraction::orbit("missing", "Orbit").await])],
            })
            .await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_introduction_with_declared_window_utility_and_action_targets() {
        use semio_framework::window_element_id;
        use semio_framework::onboarding::{IntroductionDefinition, IntroductionInteraction, IntroductionStepDefinition};
        use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework::InteractiveJobClassification;
        let definition = minimal_app("good-intro-app")
            .await
            .mutation("addLayer", LocalizedLabel::data("Add Layer"))
            .await
            .utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush)
            .await
            .window_kind_utilities("main", vec!["brush".into()])
            .await
            .window_kind_action_refs("main", vec!["addLayer".into()])
            .await
            .introduction(IntroductionDefinition {
                title: LocalizedLabel::data("Welcome"),
                steps: vec![
                    IntroductionStepDefinition::new("welcome", LocalizedLabel::data("Welcome"), LocalizedLabel::data("Hi")),
                    IntroductionStepDefinition::new("main-window", LocalizedLabel::data("Main Window"), LocalizedLabel::data("…")).introduce(window_element_id("main")),
                    IntroductionStepDefinition::new("brush-utility", LocalizedLabel::data("Brush"), LocalizedLabel::data("…")).introduce("brush").interact(vec![IntroductionInteraction::utility("brush", "Activate Brush").await]),
                    IntroductionStepDefinition::new("add-layer", LocalizedLabel::data("Add Layer"), LocalizedLabel::data("…")).interact(vec![IntroductionInteraction::action("addLayer", "Add a Layer").await]),
                    IntroductionStepDefinition::new("navigate-main", LocalizedLabel::data("Navigate"), LocalizedLabel::data("…")).interact(vec![IntroductionInteraction::pan("main", "Pan").await, IntroductionInteraction::zoom("main", "Zoom").await]),
                ],
            })
            .await
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
            .await
            .build_definition();
        let introduction = definition.introduction.expect("introduction present");
        assert_eq!(introduction.steps.len(), 5);
    }

    async fn minimal_tutorial(id: &str) -> TutorialDefinition {
        use semio_framework::onboarding::{TutorialBase, TutorialDefinition, TutorialTracks, TutorialUiSnapshot};
        TutorialDefinition {
            id: id.into(),
            title: LocalizedLabel::data("Tutorial"),
            description: None,
            duration_ms: 10_000,
            chapters: vec![],
            base: TutorialBase { document_dsl: None, example_id: None, ui: TutorialUiSnapshot::default(), cameras: vec![] },
            tracks: TutorialTracks::default(),
            recorded_at: None,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_tutorial_injects_start_tutorial_action() {
        use semio_framework::ActionKind;
        use semio_framework::onboarding::START_TUTORIAL_ACTION_ID;
        let definition = minimal_app("tutorial-app").await.tutorial(minimal_tutorial("welcome-tour").await).await.build_definition();
        let start_tutorial = declared_actions(&definition).find(|action| action.id == START_TUTORIAL_ACTION_ID).expect("startTutorial injected");
        assert_eq!(start_tutorial.kind, ActionKind::View);
        assert!(!start_tutorial.in_palette, "the shell-owned Play Tutorial command owns palette discovery");
    }

    #[semio_framework_async_macros::async_test]
    async fn no_tutorial_means_no_start_tutorial_action_but_record_is_always_injected() {
        use semio_framework::onboarding::{RECORD_TUTORIAL_ACTION_ID, START_TUTORIAL_ACTION_ID};
        let definition = minimal_app("no-tutorial-app").await.build_definition();
        assert!(!declared_actions(&definition).any(|action| action.id == START_TUTORIAL_ACTION_ID));
        assert!(declared_actions(&definition).any(|action| action.id == RECORD_TUTORIAL_ACTION_ID), "recordTutorial is injected unconditionally — recording needs no app declaration");
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tutorial_failing_structural_validation() {
        let mut tutorial = minimal_tutorial("out-of-range-tour").await;
        tutorial.duration_ms = 100;
        tutorial.chapters.push(semio_framework::onboarding::TutorialChapter { id: "late".into(), at: 999_999, title: LocalizedLabel::data("Late"), body: None });
        let base = minimal_app("bad-structural-tutorial-app").await;
        let __chain = base.tutorial(tutorial).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_duplicate_tutorial_ids() {
        let tour_a = minimal_tutorial("tour").await;
        let tour_b = minimal_tutorial("tour").await;
        let __base = minimal_app("dupe-tutorial-app").await;
        let __chain = __base.tutorial(tour_a).await.tutorial(tour_b).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tutorial_event_referencing_undeclared_action() {
        use semio_framework::onboarding::{TutorialEvent, TutorialEventKind};
        let mut tutorial = minimal_tutorial("bad-event-tour").await;
        tutorial.tracks.events = vec![TutorialEvent { at: 10, kind: TutorialEventKind::Action { action: "missingAction".into(), args: None } }];
        let base = minimal_app("bad-tutorial-event-app").await;
        let __chain = base.tutorial(tutorial).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tutorial_ui_change_referencing_undeclared_utility() {
        use semio_framework::onboarding::{TutorialUiChange, TutorialUiKeyframe, TutorialUiSample};
        let mut tutorial = minimal_tutorial("bad-ui-change-tour").await;
        tutorial.tracks.ui = vec![TutorialUiKeyframe { at: 10, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveUtility { window_id: "main".into(), utility_id: Some("missing".into()) }] } }];
        let base = minimal_app("bad-tutorial-ui-app").await;
        let __chain = base.tutorial(tutorial).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_tutorial_gesture_targeting_malformed_element_id() {
        use semio_framework::onboarding::{IntroductionGesture, IntroductionPoint, TutorialGestureCue};
        let mut tutorial = minimal_tutorial("bad-gesture-tour").await;
        tutorial.tracks.gestures = vec![TutorialGestureCue { at: 10, duration_ms: 200, gesture: IntroductionGesture::LeftClick { at: IntroductionPoint::Element { id: "not-camel-case".into(), offset: None } }, cursor: None }];
        let base = minimal_app("bad-tutorial-gesture-app").await;
        let __chain = base.tutorial(tutorial).await;
        let result = std::panic::catch_unwind(move || __chain.build_definition());
        assert!(result.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_accepts_tutorial_with_declared_action_utility_and_gesture_targets() {
        use semio_framework::window_element_id;
        use semio_framework::onboarding::{IntroductionGesture, IntroductionPoint, TutorialEvent, TutorialEventKind, TutorialGestureCue, TutorialUiChange, TutorialUiKeyframe, TutorialUiSample};
        let mut tutorial = minimal_tutorial("good-tour").await;
        tutorial.tracks.events = vec![TutorialEvent { at: 10, kind: TutorialEventKind::Action { action: "addLayer".into(), args: None } }];
        tutorial.tracks.ui = vec![TutorialUiKeyframe { at: 20, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveUtility { window_id: "main".into(), utility_id: Some("brush".into()) }] } }];
        tutorial.tracks.gestures = vec![TutorialGestureCue { at: 30, duration_ms: 200, gesture: IntroductionGesture::LeftClick { at: IntroductionPoint::Element { id: window_element_id("main"), offset: None } }, cursor: None }];
        let definition =
            minimal_app("good-tutorial-app").await.mutation("addLayer", LocalizedLabel::data("Add Layer")).await.utility_simple("brush", LocalizedLabel::data("Brush"), IconName::Paintbrush).await.tutorial(tutorial).await.interactive_jobs(semio_framework::InteractiveJobClassification::Migrated).await.build_definition();
        assert_eq!(definition.tutorials.len(), 1);
        assert_eq!(definition.tutorials[0].id, "good-tour");
    }
