grammar Stdio_semio_drawing_snapshot_text;
// 📖️ Real wire shape (`store::ArtifactDsl`): a `.semio` preamble line, then the JSON-encoded
// SemioDrawingSnapshot as hex digits (field structure fully typed in the sibling 🔣️component.json).
document: PREAMBLE NEWLINE hexBody EOF;
PREAMBLE: 'semio stdio.semio.drawing.dsl v1';
hexBody: HEXDIGIT*;
HEXDIGIT: [0-9a-f];
NEWLINE: '\n';
