//! 🔌 Connects the base PDF controller's typed commands to its PNG admittance owners.

use super::*;
use semio_s_artifact_stdio_pdf_editor_document::{
    IMPORT, INSERT, INSERT_OPERATOR, REPLACE, WIRE_BYTES, PngDecodeWork, PngInput, PngInvocation,
    PngPickerWork, PngTarget, import_rows, png_invocation,
};
pub use semio_s_artifact_stdio_pdf_editor_document::{CONTRACT, admits_png_action as admits, png_actions as actions};
use semio_framework_value::DslValue;
use semio_plugin::__job::{OriginalTurnFailure, StepContext};
use semio_plugin::retained_command::ArtifactCommandWork;

type App = semio_plugin::EditorApp<Pdf17Editor>;
type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<Pdf17EditorCommand>;
semio_s_artifact_stdio_pdf_editor_document::pdf_local_image_import_factory!(PdfLocalImageImport, App);

pub fn from_action(action: &str, args: Option<&DslValue>) -> Result<Pdf17EditorCommand, Fault> {
    Ok(match png_invocation(action, args)? {
        PngInvocation::Pick { target: PngTarget { page, operator, x, y, width, height }, guard } =>
            Pdf17EditorCommand::PickImage { page, operator, x, y, width, height, guard },
        PngInvocation::Import { target: PngTarget { page, operator, x, y, width, height }, revision, payload } =>
            Pdf17EditorCommand::ImportImage { page, operator, revision, x, y, width, height, payload },
    })
}

fn target(page: u32, operator: u32, [x, y, width, height]: [f64; 4]) -> PngTarget {
    PngTarget { page, operator, x, y, width, height }
}

pub fn native_work(tool: &'static str) -> Box<dyn ArtifactCommandWork<App>> {
    if matches!(tool, INSERT | REPLACE) {
        Box::new(PngPickerWork::<App>::new(tool, |command| match command {
            Command::Native(Pdf17EditorCommand::PickImage { page, operator, x, y, width, height, guard }) =>
                Ok((target(*page, *operator, [*x, *y, *width, *height]), guard.as_str())),
            _ => Err(Fault::from("PDF image picker command mismatch")),
        }))
    } else {
        semio_s_artifact_stdio_contract::editing::bounded_native_edit_work::<Pdf17Editor>(tool)
    }
}

pub fn command_id(command: &Pdf17EditorCommand) -> Option<&'static str> {
    match command {
        Pdf17EditorCommand::PickImage { operator: INSERT_OPERATOR, .. } => Some(INSERT),
        Pdf17EditorCommand::PickImage { .. } => Some(REPLACE),
        Pdf17EditorCommand::ImportImage { .. } => Some(IMPORT),
        _ => None,
    }
}

pub fn build(request: &mut Option<semio_plugin::ArtifactOwnedToolJobRequest<App>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    if !request.as_ref().is_some_and(|request| request.tool_id == IMPORT) {
        return Ok(false);
    }
    semio_plugin::retained_command::producer::admit_retained_tool_producer(
        request, output, cx, Pdf17Editor::command_id, WIRE_BYTES, import_rows(), false, |_| PngDecodeWork::new(IMPORT, png_input),
    )
}

fn png_input(command: &Command) -> Option<PngInput<'_>> {
    let Command::Native(Pdf17EditorCommand::ImportImage { page, operator, revision, x, y, width, height, payload }) = command else { return None };
    Some(PngInput { placement: target(*page, *operator, [*x, *y, *width, *height]).placement(), revision, payload })
}
