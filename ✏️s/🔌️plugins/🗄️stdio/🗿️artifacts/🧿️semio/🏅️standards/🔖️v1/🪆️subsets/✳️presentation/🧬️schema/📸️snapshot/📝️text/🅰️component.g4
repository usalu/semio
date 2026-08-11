grammar Semio_semio_presentation_snapshot;
document: header body EOF;
header: 'schema' WS 'stdio.semio.presentation.snapshot' NL;
body: PAYLOAD NL?;
WS: ' ';
NL: '\n';
PAYLOAD: [0-9a-f]* ; // hex(serde_json(SemioPresentationSnapshot)) -- see sibling JSON Schema
