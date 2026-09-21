mod example_source_tests {
    use super::*;

    /// 🪪️ Contract §1 fixture — a canonical id built via `surface_app_id` from a fixture `Dialect`,
    /// not a hand-written pre-migration string like the retired `"puzzle2d-play"`/`"demo-play"`.
    fn canonical_test_app_id(slug: &str) -> String {
        surface_app_id(&ArtifactDialect { artifact_kind: format!("s.test.example-source.{slug}"), standard: "1".into(), subset: "*".into() }, AppRole::Editor)
    }

    #[semio_framework_async_macros::async_test]
    async fn example_source_converts_into_example_definition_and_registers_on_app() {
        let source = ExampleSource::new("nakagin", LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm"), "{\"kind\":\"demo\"}", "building");
        assert_eq!(source.id(), "nakagin");
        assert_eq!(source.document(), "{\"kind\":\"demo\"}");
        assert_eq!(source.payload(), source.document_json());
        assert!(source.deferred_body_asset(&ArtifactDialect { artifact_kind: "s.test.example-source.puzzle2d-play".into(), standard: "1".into(), subset: "*".into() }, MediaType { class: MediaClass::Text, form: MediaForm::Any }).is_none(), "an inline body declares no asset of its own");
        let dialect = ArtifactDialect { artifact_kind: "s.test.example-source.puzzle2d-play".into(), standard: "1".into(), subset: "*".into() };
        let definition = source.clone().into_example_definition(dialect.clone());
        assert_eq!(definition.id, "nakagin");
        assert_eq!(definition.artifact_json, "{\"kind\":\"demo\"}");
        assert_eq!(definition.dialect, dialect, "an example is stamped with the dialect it was authored for, never with one app of it");
        let app = App::from_builder(
            App::builder(canonical_test_app_id("puzzle2d-play"), LocalizedLabel::data("Puzzle"))
                .await
                .document(["semio", "puzzle"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), "puzzle.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await,
        )
        .await
        .example_source(&source)
        .await;
        assert_eq!(app.examples.len(), 1);
        assert_eq!(app.examples[0].id, "nakagin");
        assert_eq!(app.examples[0].icon_id, IconName::from("building"));
    }

    /// 📚️ A DEFERRED leaf never runs its producer to answer identity, declares its asset from the
    /// AUTHORED bytes, and leaves the manifest row body-less so the descriptor stays bounded.
    ///
    /// 🐛️ `🧩️puzzle`'s `capsule-dream` parsed 3 035 200 B of DSL while the bundle was assembled, so
    /// `describe` never reached its own emitter — `📓️a3-descriptor-regeneration.md` §3.
    #[semio_framework_async_macros::async_test]
    async fn a_deferred_example_declares_its_authored_bytes_without_producing_the_document() {
        static SOURCE_BYTES: &[u8] = b"kind demo\n";
        fn produce() -> String {
            panic!("a deferred example body must not be produced to answer the manifest or the descriptor");
        }
        let source = ExampleSource::deferred("capsule-dream", LocalizedLabel::data("Capsule Dream"), "building", ".dsl.semio", SOURCE_BYTES, produce);
        let dialect = ArtifactDialect { artifact_kind: "s.puzzle.5d".into(), standard: "1".into(), subset: "*".into() };
        let declaration = source.deferred_body_asset(&dialect, MediaType { class: MediaClass::Text, form: MediaForm::Any }).expect("a deferred body declares its asset");
        assert_eq!(declaration.name, "📚️examples/s.puzzle.5d.1.-/capsule-dream.dsl.semio");
        assert_eq!(declaration.size_bytes, SOURCE_BYTES.len() as u64);
        assert_eq!(declaration.sha256, semio_framework_hash::sha256_hex(SOURCE_BYTES), "the declaration hashes the authored bytes, which needs no parse");
        let definition = source.into_example_definition(dialect);
        assert_eq!(definition.id, "capsule-dream");
        assert!(definition.artifact_json.is_empty(), "a deferred body travels as its declared asset, never inside the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn example_delegates_to_example_source() {
        let app = App::from_builder(
            App::builder(canonical_test_app_id("demo-play"), LocalizedLabel::data("Demo"))
                .await
                .document(["semio", "demo"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), "demo.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await,
        )
        .await
        .example("default", LocalizedLabel::native("Default", "Standard"), "{}", "file")
        .await;
        assert_eq!(app.examples.len(), 1);
        assert_eq!(app.examples[0].id, "default");
    }
}
