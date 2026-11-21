//! 📋 Resolves the manifest's exact checked file export for the mounted app.
use semio_framework::{
    ActionDefinition, AppDefinition, DslValue, MediaType,
    NATURAL_FILE_PORT_ID, OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID,
    manifest::{EXPORT_MEDIA_FILE_ACTION_ID, EXPORT_SNAPSHOT_FILE_ACTION_ID, MediaPortDirection},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeMediaDeclaration {
    pub(super) port: String,
    pub(super) format_kind: String,
    pub(super) mime: String,
    pub(super) media_type: MediaType,
    filename: String,
    pub(super) reference: bool,
}

impl NativeMediaDeclaration {
    /// 🔎 Resolves a native Save from a mounted application's static declaration.
    pub fn for_app(app: &AppDefinition, action_id: &str) -> Option<Self> {
        let natural = action_id == SAVE_ARTIFACT_FILE_ACTION_ID;
        let checked_id = if natural { EXPORT_MEDIA_FILE_ACTION_ID } else { action_id };
        let ([port, filename, mime, kind], media_type) = declared_fields(app, checked_id)?;
        let reference = port == NATURAL_FILE_PORT_ID && natural_reference(app, kind, mime);
        if natural && !reference { return None; }
        Some(Self { port: port.into(), format_kind: kind.into(), mime: mime.into(), media_type, filename: filename.into(), reference })
    }

    pub fn filename(&self) -> &str { &self.filename }

    /// 📂 Uses the manifest-owned extension in the native Save picker.
    pub fn extension(&self) -> Option<&str> {
        self.filename.rsplit_once('.').map(|(_, extension)| extension)
            .filter(|value| !value.is_empty())
    }
}

fn portable_name(name: &str) -> bool {
    if name.is_empty() || name.encode_utf16().take(129).count() > 128 || name.ends_with(['.', ' '])
        || name.chars().any(|c| c <= '\u{1f}' || c == '\u{7f}' || "/\\:*?\"<>|".contains(c))
    { return false; }
    let stem = name.split('.').next().unwrap_or_default().to_ascii_lowercase();
    !matches!(stem.as_str(), "con" | "prn" | "aux" | "nul")
        && !stem.strip_prefix("com").or_else(|| stem.strip_prefix("lpt"))
            .is_some_and(|digit| digit.len() == 1 && matches!(digit.as_bytes()[0], b'1'..=b'9'))
}

fn mime_valid(mime: &str) -> bool {
    let token = |part: &str| {
        !part.is_empty() && part.bytes().all(|byte| matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-'))
    };
    mime.split_once('/').is_some_and(|(class, form)| token(class) && token(form))
}

/// 🎛️ Resolves the agreed defaults from the action roster and its single declared output port.
fn declared_fields<'a>(app: &'a AppDefinition, action_id: &str) -> Option<([&'a str; 4], MediaType)> {
    if !matches!(action_id, EXPORT_MEDIA_FILE_ACTION_ID | EXPORT_SNAPSHOT_FILE_ACTION_ID) { return None; }
    let mut common: Option<[&str; 4]> = None;
    for action in actions(app).filter(|action| action.id == action_id) {
        let value = |id| argument(action, id)?.as_str();
        let row = [value("outputPort")?, value("filename")?, value("mediaType")?, value("formatKind")?];
        if common.is_some_and(|existing| existing != row) { return None; }
        common = Some(row);
    }
    let [port, filename, mime, kind] = common?;
    if port.is_empty() || kind.is_empty() || !portable_name(filename) || !mime_valid(mime) { return None; }
    let mut matches = app.io.ports.iter().filter(|declared| declared.id == port);
    let owner = matches.next()?;
    if matches.next().is_some() || owner.direction != MediaPortDirection::Out { return None; }
    Some(([port, filename, mime, kind], owner.media_type))
}

fn actions(app: &AppDefinition) -> impl Iterator<Item = &ActionDefinition> {
    app.actions.iter().chain(app.window_kinds.iter().flat_map(|window| window.actions.iter()))
}

fn argument<'a>(action: &'a ActionDefinition, id: &str) -> Option<&'a DslValue> {
    let mut fields = action.args.iter().filter(|arg| arg.id == id);
    let selected = fields.next()?.default.as_ref()?;
    fields.next().is_none().then_some(selected)
}

fn natural_reference(app: &AppDefinition, kind: &str, mime: &str) -> bool {
    let mut extension = None;
    [SAVE_ARTIFACT_FILE_ACTION_ID, OPEN_ARTIFACT_FILE_ACTION_ID].into_iter().all(|id| {
        let mut declared = actions(app).filter(|action| action.id == id).peekable();
        declared.peek().is_some() && declared.all(|action| {
            let Some(suffix) = argument(action, "extension").and_then(DslValue::as_str).and_then(|s| s.strip_prefix('.'))
                else { return false; };
            if suffix.is_empty() || !suffix.bytes().all(|byte| byte.is_ascii_alphanumeric())
                || extension.is_some_and(|expected| expected != suffix)
            { return false; }
            extension = Some(suffix);
            argument(action, "formatKind").and_then(DslValue::as_str) == Some(kind)
                && argument(action, "mediaType").and_then(DslValue::as_str) == Some(mime)
                && argument(action, "binary").and_then(DslValue::as_bool) == Some(true)
        })
    })
}
