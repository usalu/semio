grammar Stdio_semio_video_snapshot;
// Envelope header + hex(JSON) body — see sibling 🔣️component.json for the JSON payload's real
// (schema/streams[kind,codec,width,height,rate,samples]) structure.
document: header NEWLINE body EOF;
header: 'schema' ' ' 'stdio.semio.video';
body: HEXBYTE*;
HEXBYTE: [0-9a-f] [0-9a-f];
NEWLINE: '\n';
