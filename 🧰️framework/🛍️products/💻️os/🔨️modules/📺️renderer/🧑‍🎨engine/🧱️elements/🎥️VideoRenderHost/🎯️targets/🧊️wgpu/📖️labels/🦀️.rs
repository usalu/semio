//! 🌐️ What the host tells the user about a video export, in every shell language (English first, German second): the Rust twin of `VIDEO_RENDER_EXPORT_TEXT_V1` in `🎥️VideoRenderHost/🟦️.ts`.

/// 🏷️ One sentence of the export's life.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoRenderText {
    Started,
    Done,
    Cancelled,
    Refused,
    RefusedCapability,
    Failed,
    Progress,
    Cancel,
}

impl VideoRenderText {
    pub const ALL: [Self; 8] = [Self::Started, Self::Done, Self::Cancelled, Self::Refused, Self::RefusedCapability, Self::Failed, Self::Progress, Self::Cancel];

    /// 🔤️ The template in English (`false`) or German (`true`), `{placeholders}` unfilled.
    pub fn template(self, is_de: bool) -> &'static str {
        match (self, is_de) {
            (Self::Started, false) => "Rendering video {filename} — progress and cancel in Task Manager.",
            (Self::Started, true) => "Video {filename} wird gerendert — Fortschritt und Abbruch im Task-Manager.",
            (Self::Done, false) => "Video {filename} exported ({frames} frames).",
            (Self::Done, true) => "Video {filename} exportiert ({frames} Bilder).",
            (Self::Cancelled, false) => "Video export {filename} cancelled.",
            (Self::Cancelled, true) => "Videoexport {filename} abgebrochen.",
            (Self::Refused, false) => "Video export {filename} refused: the program is invalid ({code}).",
            (Self::Refused, true) => "Videoexport {filename} abgelehnt: das Programm ist ungültig ({code}).",
            (Self::RefusedCapability, false) => "Video export {filename} refused: {owner} did not request the {capability} capability.",
            (Self::RefusedCapability, true) => "Videoexport {filename} abgelehnt: {owner} hat die Fähigkeit {capability} nicht angefordert.",
            (Self::Failed, false) => "Video export {filename} failed: {reason}",
            (Self::Failed, true) => "Videoexport {filename} fehlgeschlagen: {reason}",
            (Self::Progress, false) => "Frame {completed} of {total}",
            (Self::Progress, true) => "Bild {completed} von {total}",
            (Self::Cancel, false) => "Cancel video export",
            (Self::Cancel, true) => "Videoexport abbrechen",
        }
    }
}

/// 🌐️ `kind` in English or German with every `{name}` of `values` filled; an unknown placeholder stays as written.
pub fn video_render_text(kind: VideoRenderText, is_de: bool, values: &[(&str, &str)]) -> String {
    let template = kind.template(is_de);
    let mut out = String::with_capacity(template.len() + 32);
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let Some(close) = rest[open..].find('}') else { break };
        let name = &rest[open + 1..open + close];
        match values.iter().find(|(key, _)| *key == name) {
            Some((_, value)) => out.push_str(value),
            None => out.push_str(&rest[open..=open + close]),
        }
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out
}
