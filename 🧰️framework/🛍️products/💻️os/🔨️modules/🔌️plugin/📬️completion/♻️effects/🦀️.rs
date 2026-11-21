//! 📤️ Known host effect fields keep their typed owners through exact deferred retirement.

use semio_framework::{Effect, kernel::ClipboardFragment};
use semio_framework_value::{DslValue, retirement::RetireOwned};

pub struct CompletionEffect(pub Effect);

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub enum CompletionEffectFields {
    Text(String),
    LoadDocument(Vec<u8>, Vec<u8>),
    DownloadMediaExport(String, String, String, Option<String>),
    RequestFileOpen(u64, String, Option<String>, String, Option<DslValue>, bool, Option<u32>),
    SetActiveUtility(String, String),
    OpenDialog(u64, String, Option<DslValue>),
    DispatchAction(u64, String, Option<DslValue>, u64),
    ReplayShellCommand(String, Option<DslValue>),
    ClipboardWrite(ClipboardFragment),
}

impl CompletionEffect {
    /// 🧩️ Identifies effects with a defined original retirement seed.
    pub fn supported(effect: &Effect) -> bool {
        matches!(effect, Effect::Notify { .. } | Effect::Navigate { .. } | Effect::LoadDocument { .. }
            | Effect::OpenExternalUrl { .. } | Effect::SetPanel { .. } | Effect::DownloadMediaExport { .. }
            | Effect::RequestFileOpen { .. } | Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. }
            | Effect::OpenDialog { .. } | Effect::DispatchAction { .. } | Effect::ReplayShellCommand { .. }
            | Effect::ClipboardWrite { .. })
    }

    /// 📐️ Quotes the inline retirement cursor before taking the effect.
    pub fn original_birth(effect: &Effect) -> Option<usize> {
        Self::supported(effect).then_some(size_of::<<Self as RetireOwned>::RetirementCursor>())
    }
}

impl RetireOwned for CompletionEffect {
    type RetirementCursor = <CompletionEffectFields as RetireOwned>::RetirementCursor;

    fn retirement_ready(&self) -> bool { Self::supported(&self.0) }

    fn retirement_seed(self) -> Self::RetirementCursor {
        use CompletionEffectFields as Fields;
        let fields = match self.0 {
            Effect::Notify { message } => Fields::Text(message),
            Effect::Navigate { uri } => Fields::Text(uri),
            Effect::LoadDocument { pack, spr } => Fields::LoadDocument(pack, spr),
            Effect::OpenExternalUrl { url } => Fields::Text(url),
            Effect::SetPanel { panel_json } => Fields::Text(panel_json),
            Effect::DownloadMediaExport { filename, mime_type, data, encoding } => Fields::DownloadMediaExport(filename, mime_type, data, encoding),
            Effect::RequestFileOpen { req, accept, read_as, import_action, args, multiple, max_file_bytes } => {
                Fields::RequestFileOpen(req.0, accept, read_as, import_action, args, multiple, max_file_bytes)
            }
            Effect::SetActiveUtility { window_id, utility_id } => Fields::SetActiveUtility(window_id, utility_id),
            Effect::SetActiveTool { tool_id } => Fields::Text(tool_id),
            Effect::OpenDialog { req, dialog_id, args } => Fields::OpenDialog(req.0, dialog_id, args),
            Effect::DispatchAction { req, action, args, delay_ms } => Fields::DispatchAction(req.0, action, args, delay_ms),
            Effect::ReplayShellCommand { action_id, args } => Fields::ReplayShellCommand(action_id, args),
            Effect::ClipboardWrite { fragment } => Fields::ClipboardWrite(fragment),
            _ => unreachable!("original retirement admission checks completion effect support"),
        };
        fields.retirement_seed()
    }

    fn controlled_retirement_supported() -> bool { true }
}
