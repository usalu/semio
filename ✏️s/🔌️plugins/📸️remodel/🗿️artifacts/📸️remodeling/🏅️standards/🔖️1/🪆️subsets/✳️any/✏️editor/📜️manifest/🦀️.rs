//! 📜️ Manifest of the remodeling editor: the app definition stitched from one definition per taxonomy node. See [`super`].

use super::*;

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/utility declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_remodeling_app() -> AppDefinition {
    Editor::builder(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::REMODELING_DIALECT)
            .document(["semio", "remodeling"])
            .artifact_kind(semio_s_artifact_remodel_remodeling_schema::standards::v1::subsets::any::schema::artifact_kind())
            // 🔌️ `photos:in`/`mesh:out` — `2d.image`/`3d.mesh` are declared by `shooting`/`lowpoly`
            // respectively (reused here, not redeclared).
            .media_input(remodeling_photos_in_port())
            .media_output(remodeling_mesh_out_port())
            .icon_id("remodeling-app")
            .mode_def(capture::definition())
            .mode_def(model::definition())
            .mode_def(analyze::definition())
            .default_mode_id(model::REMODELING_PLAY_MODE_MODEL)
            .window_kind_def(model::windows::model::definition())
            .window_kind_def(capture::windows::frames::definition())
            .window_kind_def(analyze::windows::report::definition())
            // 🕹️ The "assets" interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)
            // replaces the deleted `RemodelingSelection` config field — both windows that carry the "select"
            // utility (`remodeling-main`'s World3d viewport, `remodeling-frames`' Canvas2d frame view) declare it.
            .interaction(InteractionDefinition {
                id: "assets".into(),
                label: LocalizedLabel::native("Assets", "Assets"),
                granularities: vec![GranularityDefinition { id: "asset".into(), label: LocalizedLabel::native("Asset", "Asset"), icon_id: "image".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(model::windows::model::REMODELING_PLAY_WINDOW_MAIN, vec![InteractionRef::new("assets")])
            .window_kind_interactions(capture::windows::frames::REMODELING_PLAY_WINDOW_FRAMES, vec![InteractionRef::new("assets")])
            .default_layout(model::layout())
            .named_layout(capture::layout())
            .named_layout(analyze::layout())
            .mode_layout(capture::REMODELING_PLAY_MODE_CAPTURE, capture::REMODELING_PLAY_LAYOUT_CAPTURE)
            .mode_layout(analyze::REMODELING_PLAY_MODE_ANALYZE, analyze::REMODELING_PLAY_LAYOUT_ANALYZE)
            .panel_tab_def(document::definition())
            .panel_tab_def(media::definition())
            .panel_tab_def(results::definition())
            .panel_tab_def(parameters::definition())
            .panel_tab_def(calibration_panel::definition())
            .panel_tab_def(tracks::definition())
            .panel_tab_def(quality::definition())
            // 🏗️ The reconstruction tool: a framework ToolRun, started, paused, stepped, aborted and finalized
            // only through the reserved tool-run actions.
            .tool(model::tools::reconstruction::definition())
            // 📥️ Ingestion.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("importFrames", LocalizedLabel::native("Import Frames", "Frames importieren"), ActionKind::Shell, "hard-drive") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importFramePayload", LocalizedLabel::native("Import Frame Payload", "Bild-Payload importieren"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("importVideo", LocalizedLabel::native("Import Video", "Video importieren"), ActionKind::Shell, "hard-drive") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importVideoFramePayload", LocalizedLabel::native("Import Video Frame Payload", "Video-Frame-Payload importieren"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importVideoDone", LocalizedLabel::native("Import Video Done", "Video-Import abgeschlossen"), ActionKind::Mutation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importVideoBytesPayload", LocalizedLabel::native("Import Video Bytes Payload", "Video-Byte-Payload importieren"), ActionKind::Mutation) })
            .action_with(ActionDefinition::bounded_catalog("importAbort", LocalizedLabel::native("Abort Import", "Import abbrechen"), ActionKind::Mutation))
            .mutation("addStream", LocalizedLabel::native("Add Stream", "Stream hinzufügen"))
            .action_args("addStream", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value(&"Stream"),
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![ActionArgOption::new("image-sequence", LocalizedLabel::native("Image Sequence", "Bildsequenz")), ActionArgOption::new("video", LocalizedLabel::native("Video", "Video"))]).default_value(&"image-sequence"),
                ActionArgDef::text("cameraId", LocalizedLabel::native("Camera Id", "Kamera-Id")).default_value(&""),
            ])
            .mutation("removeStream", LocalizedLabel::native("Remove Stream", "Stream entfernen"))
            .action_destructive("removeStream")
            .action_args("removeStream", vec![ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required()])
            .mutation("setStreamSync", LocalizedLabel::native("Set Stream Sync", "Stream-Synchronisation festlegen"))
            .action_args("setStreamSync", vec![ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required(), ActionArgDef::number("syncOffsetMs", LocalizedLabel::native("Sync Offset (ms)", "Sync-Versatz (ms)")).default_value(&0)])
            // 🎯️ Calibration / GCPs.
            .mutation("editCalibration", LocalizedLabel::native("Edit Calibration", "Kalibrierung bearbeiten"))
            .action_args("editCalibration", vec![
                ActionArgDef::text("cameraId", LocalizedLabel::native("Camera Id", "Kamera-Id")).required(),
                ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")),
                ActionArgDef::select("model", LocalizedLabel::native("Model", "Modell"), vec![ActionArgOption::new("pinhole", LocalizedLabel::native("Pinhole", "Lochkamera")), ActionArgOption::new("brownConrady", LocalizedLabel::native("Brown-Conrady", "Brown-Conrady")), ActionArgOption::new("fisheye", LocalizedLabel::native("Fisheye", "Fischauge"))]).default_value(&"pinhole"),
                ActionArgDef::number("fx", LocalizedLabel::native("fx", "fx")).default_value(&1000),
                ActionArgDef::number("fy", LocalizedLabel::native("fy", "fy")).default_value(&1000),
                ActionArgDef::number("cx", LocalizedLabel::native("cx", "cx")).default_value(&0),
                ActionArgDef::number("cy", LocalizedLabel::native("cy", "cy")).default_value(&0),
                ActionArgDef::number("skew", LocalizedLabel::native("Skew", "Scherung")).default_value(&0),
                ActionArgDef::number("k1", LocalizedLabel::native("k1", "k1")).default_value(&0),
                ActionArgDef::number("k2", LocalizedLabel::native("k2", "k2")).default_value(&0),
                ActionArgDef::number("k3", LocalizedLabel::native("k3", "k3")).default_value(&0),
                ActionArgDef::number("p1", LocalizedLabel::native("p1", "p1")).default_value(&0),
                ActionArgDef::number("p2", LocalizedLabel::native("p2", "p2")).default_value(&0),
                ActionArgDef::toggle("locked", LocalizedLabel::native("Locked", "Gesperrt")).default_value(&false),
            ])
            .mutation("calibrateCameras", LocalizedLabel::native("Calibrate Cameras", "Kameras kalibrieren"))
            .mutation("addGcp", LocalizedLabel::native("Add Ground Control Point", "Passpunkt hinzufügen"))
            .action_args("addGcp", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value(&"GCP"),
                ActionArgDef::number("worldX", LocalizedLabel::native("World X", "Welt X")).default_value(&0),
                ActionArgDef::number("worldY", LocalizedLabel::native("World Y", "Welt Y")).default_value(&0),
                ActionArgDef::number("worldZ", LocalizedLabel::native("World Z", "Welt Z")).default_value(&0),
            ])
            .mutation("removeGcp", LocalizedLabel::native("Remove Ground Control Point", "Passpunkt entfernen"))
            .action_destructive("removeGcp")
            .action_args("removeGcp", vec![ActionArgDef::text("gcpId", LocalizedLabel::native("GCP Id", "Passpunkt-Id")).required()])
            .mutation("placeGcpObservation", LocalizedLabel::native("Place GCP Observation", "Passpunkt-Beobachtung setzen"))
            .action_args("placeGcpObservation", vec![
                ActionArgDef::text("gcpId", LocalizedLabel::native("GCP Id", "Passpunkt-Id")).required(),
                ActionArgDef::text("streamId", LocalizedLabel::native("Stream Id", "Stream-Id")).required(),
                ActionArgDef::number("frameIndex", LocalizedLabel::native("Frame Index", "Frame-Index")).required(),
                ActionArgDef::number("pixelX", LocalizedLabel::native("Pixel X", "Pixel X")).required(),
                ActionArgDef::number("pixelY", LocalizedLabel::native("Pixel Y", "Pixel Y")).required(),
            ])
            // ⚙️ 8 param-group setters, one per `ReconstructionParams` sub-struct.
            .mutation("setIngestParams", LocalizedLabel::native("Set Ingest Params", "Ingest-Parameter festlegen"))
            .action_args("setIngestParams", vec![
                ActionArgDef::number("frameSampleStride", LocalizedLabel::native("Frame Sample Stride", "Frame-Abtastschrittweite")).default_value(&5),
                ActionArgDef::number("maxFrames", LocalizedLabel::native("Max Frames", "Max. Frames")).default_value(&200),
                ActionArgDef::number("downscaleLongEdgePx", LocalizedLabel::native("Downscale Long Edge (px)", "Verkleinerung lange Kante (px)")).default_value(&1600),
                ActionArgDef::slider("minSharpness", LocalizedLabel::native("Min Sharpness", "Min. Schärfe"), 0.0, 1.0).default_value(&0.3),
            ])
            .mutation("setFeatureParams", LocalizedLabel::native("Set Feature Params", "Feature-Parameter festlegen"))
            .action_args("setFeatureParams", vec![
                ActionArgDef::select("detector", LocalizedLabel::native("Detector", "Detektor"), vec![ActionArgOption::new("orb", LocalizedLabel::native("ORB", "ORB")), ActionArgOption::new("akaze", LocalizedLabel::native("AKAZE", "AKAZE")), ActionArgOption::new("harris", LocalizedLabel::native("Harris", "Harris"))]).default_value(&"orb"),
                ActionArgDef::number("targetCount", LocalizedLabel::native("Target Count", "Ziel-Anzahl")).default_value(&4000),
                ActionArgDef::number("octaves", LocalizedLabel::native("Octaves", "Oktaven")).default_value(&4),
                ActionArgDef::slider("edgeThreshold", LocalizedLabel::native("Edge Threshold", "Kanten-Schwelle"), 1.0, 50.0).default_value(&10.0),
            ])
            .mutation("setMatchParams", LocalizedLabel::native("Set Match Params", "Match-Parameter festlegen"))
            .action_args("setMatchParams", vec![
                ActionArgDef::select("matcher", LocalizedLabel::native("Matcher", "Matcher"), vec![ActionArgOption::new("brute-force", LocalizedLabel::native("Brute Force", "Brute Force")), ActionArgOption::new("kd-tree", LocalizedLabel::native("KD-Tree", "KD-Baum"))]).default_value(&"brute-force"),
                ActionArgDef::slider("ratioTest", LocalizedLabel::native("Ratio Test", "Verhältnistest"), 0.1, 1.0).default_value(&0.8),
                ActionArgDef::toggle("crossCheck", LocalizedLabel::native("Cross Check", "Kreuzprüfung")).default_value(&true),
                ActionArgDef::number("sequentialWindow", LocalizedLabel::native("Sequential Window", "Sequenzielles Fenster")).default_value(&8),
                ActionArgDef::number("maxPairsPerFrame", LocalizedLabel::native("Max Pairs Per Frame", "Max. Paare pro Frame")).default_value(&16),
                ActionArgDef::toggle("loopClosure", LocalizedLabel::native("Loop Closure", "Schleifenschluss")).default_value(&true),
            ])
            .mutation("setSfmParams", LocalizedLabel::native("Set SfM Params", "SfM-Parameter festlegen"))
            .action_args("setSfmParams", vec![
                ActionArgDef::number("ransacIterations", LocalizedLabel::native("RANSAC Iterations", "RANSAC-Iterationen")).default_value(&1000),
                ActionArgDef::slider("ransacThresholdPx", LocalizedLabel::native("RANSAC Threshold (px)", "RANSAC-Schwelle (px)"), 0.1, 10.0).default_value(&2.0),
                ActionArgDef::number("minTrackLength", LocalizedLabel::native("Min Track Length", "Min. Spurlänge")).default_value(&3),
                ActionArgDef::number("baMaxIterations", LocalizedLabel::native("BA Max Iterations", "BA Max. Iterationen")).default_value(&50),
                ActionArgDef::select("robustLoss", LocalizedLabel::native("Robust Loss", "Robuster Verlust"), vec![ActionArgOption::new("l2", LocalizedLabel::native("L2", "L2")), ActionArgOption::new("huber", LocalizedLabel::native("Huber", "Huber")), ActionArgOption::new("cauchy", LocalizedLabel::native("Cauchy", "Cauchy"))]).default_value(&"huber"),
                ActionArgDef::slider("huberDeltaPx", LocalizedLabel::native("Huber Delta (px)", "Huber-Delta (px)"), 0.1, 10.0).default_value(&1.5),
            ])
            .mutation("setDenseParams", LocalizedLabel::native("Set Dense Params", "Dense-Parameter festlegen"))
            .action_args("setDenseParams", vec![
                ActionArgDef::select("resolution", LocalizedLabel::native("Resolution", "Auflösung"), vec![ActionArgOption::new("low", LocalizedLabel::native("Low", "Niedrig")), ActionArgOption::new("medium", LocalizedLabel::native("Medium", "Mittel")), ActionArgOption::new("high", LocalizedLabel::native("High", "Hoch"))]).default_value(&"medium"),
                ActionArgDef::number("windowRadiusPx", LocalizedLabel::native("Window Radius (px)", "Fensterradius (px)")).default_value(&3),
                ActionArgDef::number("minViewConsistency", LocalizedLabel::native("Min View Consistency", "Min. Ansichtskonsistenz")).default_value(&3),
                ActionArgDef::slider("confidenceThreshold", LocalizedLabel::native("Confidence Threshold", "Konfidenzschwelle"), 0.0, 1.0).default_value(&0.5),
                ActionArgDef::number("maxPoints", LocalizedLabel::native("Max Points", "Max. Punkte")).default_value(&500_000),
            ])
            .mutation("setMeshParams", LocalizedLabel::native("Set Mesh Params", "Mesh-Parameter festlegen"))
            .action_args("setMeshParams", vec![
                ActionArgDef::slider("tsdfVoxelSizeMm", LocalizedLabel::native("TSDF Voxel Size (mm)", "TSDF-Voxelgröße (mm)"), 1.0, 20.0).default_value(&5.0),
                ActionArgDef::slider("tsdfTruncationMm", LocalizedLabel::native("TSDF Truncation (mm)", "TSDF-Trunkierung (mm)"), 2.0, 60.0).default_value(&20.0),
                ActionArgDef::number("decimateTargetTriangles", LocalizedLabel::native("Decimate Target Triangles", "Ziel-Dreiecke (Dezimierung)")).default_value(&200_000),
                ActionArgDef::number("smoothingIterations", LocalizedLabel::native("Smoothing Iterations", "Glättungs-Iterationen")).default_value(&2),
                ActionArgDef::toggle("textureEnabled", LocalizedLabel::native("Texture Enabled", "Textur aktiviert")).default_value(&true),
                ActionArgDef::select("textureSize", LocalizedLabel::native("Texture Size", "Texturgröße"), vec![ActionArgOption::new("1024", LocalizedLabel::native("1024", "1024")), ActionArgOption::new("2048", LocalizedLabel::native("2048", "2048")), ActionArgOption::new("4096", LocalizedLabel::native("4096", "4096"))]).default_value(&"2048"),
                ActionArgDef::toggle("guaranteeWatertight", LocalizedLabel::native("Guarantee Watertight", "Wasserdichtheit garantieren")).default_value(&true),
                ActionArgDef::number("holeFillMaxBoundaryVerts", LocalizedLabel::native("Hole Fill Max Boundary Verts", "Max. Randpunkte für Lochfüllung")).default_value(&512),
                ActionArgDef::toggle("selfIntersectionCheck", LocalizedLabel::native("Self-Intersection Check", "Selbstüberschneidungsprüfung")).default_value(&false),
            ])
            .mutation("setMotionParams", LocalizedLabel::native("Set Motion Params", "Bewegungs-Parameter festlegen"))
            .action_args("setMotionParams", vec![
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).default_value(&false),
                ActionArgDef::number("maxTracks", LocalizedLabel::native("Max Tracks", "Max. Spuren")).default_value(&64),
                ActionArgDef::number("trackWindowPx", LocalizedLabel::native("Track Window (px)", "Spurfenster (px)")).default_value(&21),
                ActionArgDef::slider("minTrackQuality", LocalizedLabel::native("Min Track Quality", "Min. Spurqualität"), 0.0, 1.0).default_value(&0.3),
                ActionArgDef::number("minTrackLengthFrames", LocalizedLabel::native("Min Track Length (frames)", "Min. Spurlänge (Frames)")).default_value(&5),
            ])
            .mutation("setGeoParams", LocalizedLabel::native("Set Geo Params", "Geo-Parameter festlegen"))
            .action_args("setGeoParams", vec![
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).default_value(&false),
                ActionArgDef::number("originLon", LocalizedLabel::native("Origin Longitude", "Ursprung Längengrad")).default_value(&0),
                ActionArgDef::number("originLat", LocalizedLabel::native("Origin Latitude", "Ursprung Breitengrad")).default_value(&0),
                ActionArgDef::number("originAlt", LocalizedLabel::native("Origin Altitude", "Ursprung Höhe")).default_value(&0),
                ActionArgDef::slider("gsdM", LocalizedLabel::native("Ground Sample Distance (m)", "Bodenauflösung (m)"), 0.01, 1.0).default_value(&0.05),
                ActionArgDef::slider("dsmCellM", LocalizedLabel::native("DSM Cell Size (m)", "DOM-Zellgröße (m)"), 0.01, 5.0).default_value(&0.1),
                ActionArgDef::slider("dtmFilterRadiusM", LocalizedLabel::native("DTM Filter Radius (m)", "DGM-Filterradius (m)"), 0.1, 10.0).default_value(&2.0),
                ActionArgDef::number("orthoMaxPx", LocalizedLabel::native("Ortho Max (px)", "Ortho Max. (px)")).default_value(&4096),
            ])
            // 🧹️ Clear/reset.
            .mutation("resetPlaceholderMesh", LocalizedLabel::native("Reset Placeholder Mesh", "Platzhalter-Mesh zurücksetzen"))
            .mutation("clearSparse", LocalizedLabel::native("Clear Sparse Cloud", "Dünne Punktwolke löschen"))
            .action_destructive("clearSparse")
            .mutation("clearDense", LocalizedLabel::native("Clear Dense Cloud", "Dichte Punktwolke löschen"))
            .action_destructive("clearDense")
            .mutation("clearMeshResult", LocalizedLabel::native("Clear Mesh", "Mesh löschen"))
            .action_destructive("clearMeshResult")
            .mutation("clearTracks", LocalizedLabel::native("Clear Tracks", "Spuren löschen"))
            .action_destructive("clearTracks")
            .mutation("clearGeoProducts", LocalizedLabel::native("Clear Geo Products", "Geo-Produkte löschen"))
            .action_destructive("clearGeoProducts")
            .mutation("clearResult", LocalizedLabel::native("Clear Result", "Ergebnis löschen"))
            .action_destructive("clearResult")
            // 👁️ View-only runtime actions.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setLayerVisibility", LocalizedLabel::native("Set Layer Visibility", "Ebenensichtbarkeit festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setFrameCursor", LocalizedLabel::native("Set Frame Cursor", "Frame-Cursor festlegen"), ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setReportTable", LocalizedLabel::native("Set Report Table", "Berichtstabelle festlegen"), ActionKind::View) })
            // 📤️ Export.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::bounded_catalog("exportQcReport", LocalizedLabel::native("Export QC Report", "QC-Bericht exportieren"), ActionKind::Shell) })
            // 🧰️ Utility groups — an exclusive per-window set (active utility is host-owned); which
            // window exposes which is declared by that window's own `definition()`.
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer-2") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("sculpt", LocalizedLabel::native("Sculpt", "Formen"), "paintbrush") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("measure", LocalizedLabel::native("Measure", "Messen"), "scaling") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("gcpPlace", LocalizedLabel::native("Place GCP", "Passpunkt setzen"), "crosshair") })
            // 👁️ Internal (non-palette) view actions the panels/windows dispatch — declared so they are
            // real `ActionDefinition`s the classification gate and the tool-proof catalog can both see.
            // 🎬️ Example picker — one option per `crate::editor::remodeling::examples::REMODELING_EXAMPLES`
            // entry, so appending an example there is the only edit a new example needs.
            .action_with(ActionDefinition { in_palette: true, ..ActionDefinition::new("setActiveExample", LocalizedLabel::native("Load Example", "Beispiel laden"), ActionKind::Mutation, "panel-left") })
            .action_destructive("setActiveExample")
            .action_args("setActiveExample", vec![ActionArgDef::select(
                "exampleId",
                LocalizedLabel::native("Example", "Beispiel"),
                semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::examples::REMODELING_EXAMPLES.iter().map(|example| ActionArgOption::new(example.id, semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::examples::example_label(example))).collect(),
            )
            .default_value(&semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::examples::REMODELING_EXAMPLE_BOOT_ID)])
            // 🧵️ Phase-8 dispositions. Every declared row is `Migrated`: each is backed by the
            // exact-owner `RemodelingRetainedCommandJobFactory` proof in `🧵️RetainedCommands` and by the
            // document/config one-item store preparations in `📬️StorePreparation`, so UI dispatch (which
            // rejects anything that is not `Migrated`) and the release-blocking
            // `validate_interactive_job_classification` gate both admit it. The framework demands set
            // EQUALITY here: `validate_tool_job_rows` computes `expected = TOOL_JOB_IDS ∩ migrated` and
            // faults `interactive-job.catalog-incomplete` unless it equals the proof set.
            //
            .action_interactive_job("importFrames", InteractiveJobClassification::Migrated)
            .action_interactive_job("importFramePayload", InteractiveJobClassification::Migrated)
            .action_interactive_job("importVideo", InteractiveJobClassification::Migrated)
            .action_interactive_job("importVideoFramePayload", InteractiveJobClassification::Migrated)
            .action_interactive_job("importVideoDone", InteractiveJobClassification::Migrated)
            .action_interactive_job("importVideoBytesPayload", InteractiveJobClassification::Migrated)
            .action_interactive_job("importAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("addStream", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStream", InteractiveJobClassification::Migrated)
            .action_interactive_job("setStreamSync", InteractiveJobClassification::Migrated)
            .action_interactive_job("editCalibration", InteractiveJobClassification::Migrated)
            .action_interactive_job("calibrateCameras", InteractiveJobClassification::Migrated)
            .action_interactive_job("addGcp", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeGcp", InteractiveJobClassification::Migrated)
            .action_interactive_job("placeGcpObservation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setIngestParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFeatureParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setMatchParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSfmParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setDenseParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setMeshParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setMotionParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGeoParams", InteractiveJobClassification::Migrated)
            .action_interactive_job("resetPlaceholderMesh", InteractiveJobClassification::Migrated)
            .action_destructive("resetPlaceholderMesh")
            .action_interactive_job("clearSparse", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearDense", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearMeshResult", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearTracks", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearGeoProducts", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearResult", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLayerVisibility", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFrameCursor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setReportTable", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportQcReport", InteractiveJobClassification::Migrated)
            .action_destructive("exportQcReport")
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            // 🎯️ Typed channel surface — `io()` is this same information's single source of truth,
            // reused here rather than duplicated.
            .io(remodeling_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old default-example
            // registration (`default_remodeling_scene().print_dsl()` fed to `.example("default", …)`)
            // and the no-op `.workflow("remodeling", …)` call are dropped here (not silently: reported
            // in this packet's migration notes). The subset's own `📚️examples/🎬️demo` facet
            // (`crate::examples::…`, real content, pre-existing) is the modern,
            // role-agnostic replacement surface for this.
            .action_describe("importFrames", LocalizedLabel::native("Opens the host's file picker for PNG or JPEG images and videos; every chosen file is imported as frames for the reconstruction.", "Öffnet die Dateiauswahl des Hosts für PNG- oder JPEG-Bilder und Videos; jede gewählte Datei wird als Einzelbilder für die Rekonstruktion importiert."))
            .action_describe("importFramePayload", LocalizedLabel::native("Adds one image (name and data-URL payload) as a frame; the first image of a batch starts a new image-sequence stream, later ones join it.", "Fügt ein Bild (Name und Data-URL-Inhalt) als Einzelbild hinzu; das erste Bild eines Stapels beginnt einen neuen Bildsequenz-Stream, weitere schließen sich an."))
            .action_describe("importVideo", LocalizedLabel::native("Opens the host's video picker; the chosen video is sampled into frames according to the ingest parameters (stride, frame limit, downscaling).", "Öffnet die Videoauswahl des Hosts; das gewählte Video wird gemäß den Ingest-Parametern (Schrittweite, Bildgrenze, Verkleinerung) in Einzelbilder zerlegt."))
            .action_describe("importVideoFramePayload", LocalizedLabel::native("Adds one sampled video frame (data URL) to the current video stream, skipping frames too blurry for the ingest sharpness threshold.", "Fügt dem aktuellen Video-Stream ein abgetastetes Videobild (Data-URL) hinzu und überspringt Bilder, die für die Schärfeschwelle des Ingests zu unscharf sind."))
            .action_describe("importAbort", LocalizedLabel::native("Reverts the import that is still streaming frames into the last stream, leaving no edit, no history row and no trace.", "Verwirft den Import, der noch Bilder in den letzten Stream streamt, ohne Bearbeitung, Verlaufszeile oder Spur."))
            .action_describe("importVideoDone", LocalizedLabel::native("Records the imported video's source metadata (name, duration, frame count, size and codec) on the last stream once sampling has finished.", "Hält nach Abschluss der Abtastung die Quelldaten des importierten Videos (Name, Dauer, Bildanzahl, Größe und Codec) am letzten Stream fest."))
            .action_describe("importVideoBytesPayload", LocalizedLabel::native("Imports a whole video file as bytes when the host cannot sample frames itself; the plugin decodes and samples it.", "Importiert eine ganze Videodatei als Bytes, wenn der Host keine Einzelbilder abtasten kann; das Plugin dekodiert und tastet sie selbst ab."))
            .action_describe("addStream", LocalizedLabel::native("Adds an empty media stream (image sequence or video) with the given name, optionally bound to a calibrated camera by id.", "Fügt einen leeren Medien-Stream (Bildsequenz oder Video) mit dem angegebenen Namen hinzu, optional an eine kalibrierte Kamera gebunden."))
            .action_describe("removeStream", LocalizedLabel::native("Removes one media stream by id together with all of its frames.", "Entfernt einen Medien-Stream anhand seiner Id samt aller seiner Einzelbilder."))
            .action_describe("setStreamSync", LocalizedLabel::native("Sets the time offset in milliseconds that aligns one stream with the other streams.", "Legt den Zeitversatz in Millisekunden fest, der einen Stream mit den anderen Streams abgleicht."))
            .action_describe("editCalibration", LocalizedLabel::native("Sets the intrinsics of one camera: model, focal lengths fx and fy, principal point cx and cy, skew, radial k1 to k3 and tangential p1 and p2 distortion, and whether it is locked.", "Legt die inneren Parameter einer Kamera fest: Modell, Brennweiten fx und fy, Hauptpunkt cx und cy, Scherung, radiale Verzeichnung k1 bis k3, tangentiale p1 und p2 und ob sie gesperrt ist."))
            .action_describe("calibrateCameras", LocalizedLabel::native("Creates an initial calibration for every camera a stream is bound to that has none yet, estimated from the size of that stream's first frame.", "Erzeugt für jede an einen Stream gebundene Kamera ohne Kalibrierung eine Anfangskalibrierung, geschätzt aus der Größe des ersten Bildes dieses Streams."))
            .action_describe("addGcp", LocalizedLabel::native("Adds a ground control point with a name and known world coordinates X, Y and Z for georeferencing the reconstruction.", "Fügt einen Passpunkt mit Namen und bekannten Weltkoordinaten X, Y und Z zur Georeferenzierung der Rekonstruktion hinzu."))
            .action_describe("removeGcp", LocalizedLabel::native("Removes one ground control point by id from the reconstruction.", "Entfernt einen Passpunkt anhand seiner Id aus der Rekonstruktion."))
            .action_describe("placeGcpObservation", LocalizedLabel::native("Marks where a ground control point appears in one frame of a stream, at pixel x and y.", "Markiert, wo ein Passpunkt in einem Einzelbild eines Streams erscheint, an Pixel x und y."))
            .action_describe("setIngestParams", LocalizedLabel::native("Sets how frames are ingested: sampling stride, maximum frame count, downscaling of the long edge in pixels and the minimum sharpness a frame needs.", "Legt fest, wie Einzelbilder aufgenommen werden: Abtastschrittweite, maximale Bildanzahl, Verkleinerung der langen Kante in Pixeln und die nötige Mindestschärfe."))
            .action_describe("setFeatureParams", LocalizedLabel::native("Sets the feature detection parameters: detector, target feature count, octaves and edge threshold.", "Legt die Parameter der Merkmalserkennung fest: Detektor, angestrebte Merkmalsanzahl, Oktaven und Kantenschwelle."))
            .action_describe("setMatchParams", LocalizedLabel::native("Sets the feature matching parameters: matcher, ratio test, cross check, sequential window, maximum pairs per frame and loop closure.", "Legt die Parameter der Merkmalszuordnung fest: Zuordner, Verhältnistest, Kreuzprüfung, sequenzielles Fenster, maximale Paare pro Bild und Schleifenschluss."))
            .action_describe("setSfmParams", LocalizedLabel::native("Sets the structure-from-motion parameters: RANSAC iterations and threshold, minimum track length, bundle adjustment iterations and the robust loss with its Huber delta.", "Legt die Structure-from-Motion-Parameter fest: RANSAC-Iterationen und -Schwelle, minimale Spurlänge, Iterationen der Bündelausgleichung und die robuste Verlustfunktion mit Huber-Delta."))
            .action_describe("setDenseParams", LocalizedLabel::native("Sets the dense reconstruction parameters: resolution, window radius, minimum view consistency, confidence threshold and maximum point count.", "Legt die Parameter der dichten Rekonstruktion fest: Auflösung, Fensterradius, minimale Ansichtskonsistenz, Konfidenzschwelle und maximale Punktanzahl."))
            .action_describe("setMeshParams", LocalizedLabel::native("Sets the meshing parameters: TSDF voxel size and truncation, target triangle count, smoothing, texturing and texture size, watertightness, hole filling and self-intersection check.", "Legt die Vernetzungsparameter fest: TSDF-Voxelgröße und -Abschneidung, angestrebte Dreieckszahl, Glättung, Texturierung und Texturgröße, Wasserdichtheit, Lochfüllung und Selbstschnittprüfung."))
            .action_describe("setMotionParams", LocalizedLabel::native("Sets the motion tracking parameters: whether tracking runs, maximum tracks, window size, minimum track quality and minimum track length in frames.", "Legt die Parameter der Bewegungsverfolgung fest: ob verfolgt wird, maximale Spuren, Fenstergröße, minimale Spurqualität und minimale Spurlänge in Bildern."))
            .action_describe("setGeoParams", LocalizedLabel::native("Sets the georeferencing parameters: whether geo products are made, origin longitude, latitude and altitude, ground sample distance, DSM cell size, DTM filter radius and orthophoto size.", "Legt die Georeferenzierungsparameter fest: ob Geoprodukte entstehen, Ursprungslänge, -breite und -höhe, Bodenauflösung, DSM-Zellgröße, DTM-Filterradius und Orthofotogröße."))
            .action_describe("resetPlaceholderMesh", LocalizedLabel::native("Replaces the reconstructed mesh with the built-in placeholder mesh; the previous mesh result is discarded.", "Ersetzt das rekonstruierte Netz durch das eingebaute Platzhalternetz; das bisherige Netzergebnis wird verworfen."))
            .action_describe("clearSparse", LocalizedLabel::native("Discards the sparse point cloud of the reconstruction.", "Verwirft die dünne Punktwolke der Rekonstruktion."))
            .action_describe("clearDense", LocalizedLabel::native("Discards the dense point cloud of the reconstruction.", "Verwirft die dichte Punktwolke der Rekonstruktion."))
            .action_describe("clearMeshResult", LocalizedLabel::native("Discards the reconstructed mesh, leaving the point clouds and other results in place.", "Verwirft das rekonstruierte Netz; Punktwolken und übrige Ergebnisse bleiben erhalten."))
            .action_describe("clearTracks", LocalizedLabel::native("Discards every motion track of the reconstruction.", "Verwirft alle Bewegungsspuren der Rekonstruktion."))
            .action_describe("clearGeoProducts", LocalizedLabel::native("Discards the georeferenced products (surface and terrain models and orthophoto) of the reconstruction.", "Verwirft die georeferenzierten Produkte (Oberflächen- und Geländemodell sowie Orthofoto) der Rekonstruktion."))
            .action_describe("clearResult", LocalizedLabel::native("Discards every reconstruction result at once: mesh, sparse and dense clouds, camera trajectory, tracks, geo products and QC report.", "Verwirft alle Rekonstruktionsergebnisse auf einmal: Netz, dünne und dichte Punktwolke, Kameratrajektorie, Spuren, Geoprodukte und QS-Bericht."))
            .action_describe("exportQcReport", LocalizedLabel::native("Writes the reconstruction's quality-control report to a downloaded remodeling-qc-report.ops file on the user's machine; without a report nothing happens.", "Schreibt den Qualitätsbericht der Rekonstruktion in eine heruntergeladene Datei remodeling-qc-report.ops auf dem Rechner des Nutzers; ohne Bericht geschieht nichts."))
            .action_describe("setActiveExample", LocalizedLabel::native("Loads one of the plugin's bundled remodeling examples (streams, calibration and parameters) into the document, replacing what differs, by example id.", "Lädt eines der mitgelieferten Remodeling-Beispiele (Streams, Kalibrierung und Parameter) in das Dokument und ersetzt Abweichendes, anhand der Beispiel-Id."))
            .build_definition()
}
//#endregion 🔖️Manifest
