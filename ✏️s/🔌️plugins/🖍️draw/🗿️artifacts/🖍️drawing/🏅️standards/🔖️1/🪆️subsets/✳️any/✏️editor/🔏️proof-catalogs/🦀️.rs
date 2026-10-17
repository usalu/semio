//! 🔏️ Drawing proof catalogs: the gesture, path, image, export and bounded command proof records.

//#region 🧾️ProofCatalogs
/// 🧾️ One `bounded_first_step_tool_proofs!` invocation per owned factory — the macro emits the whole
/// `bounded_first_step_tool_proofs()` body for a single factory, so an app with two factories declares
/// two catalogs and concatenates them in its `ArtifactEditor` item (process3d/flow precedent).
pub(super) struct DrawingGestureProofs;

impl DrawingGestureProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingGestureOperationJobFactory",
        factory_type: DrawingGestureOperationJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
        tools: ["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"]
    }
}

pub(super) struct DrawingPathProofs;

impl DrawingPathProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingPathCommandJobFactory",
        factory_type: DrawingPathCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
        tools: ["editPath"]
    }
}

pub(super) struct DrawingImageProofs;

impl DrawingImageProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingImageCommandJobFactory",
        factory_type: DrawingImageCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(89_481_584, 4_096, 1, 262_144, 7_500, 1, 1),
        tools: ["importImage"]
    }
}

pub(super) struct DrawingExportProofs;

impl DrawingExportProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingExportCommandJobFactory",
        factory_type: DrawingExportCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(4_096, 4_096, 1, 67_108_864, 7_500, 1, 1),
        tools: ["exportDocument"]
    }
}

pub(super) struct DrawingBoundedProofs;

impl DrawingBoundedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingBoundedCommandJobFactory",
        factory_type: DrawingBoundedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "setSnapshot", "commitDocument", "loadDocumentJson", "setActiveExample", "setSelectedOpacity", "engagementSubmit",
            "addLayer", "dropLayerKind", "moveLayer", "deleteLayer", "duplicateLayer", "toggleLayerVisible", "combineBoolean",
            "patchLayer", "patchLayers", "setCamera", "setCameraZoom", "engagementInput", "editSelection", "editFill",
            "deleteSelection",
    "nudgeSelectionLeft", "nudgeSelectionLeftFast", "nudgeSelectionRight", "nudgeSelectionRightFast", "nudgeSelectionUp", "nudgeSelectionUpFast", "nudgeSelectionDown", "nudgeSelectionDownFast",
        ]
    }
}
//#endregion 🧾️ProofCatalogs
