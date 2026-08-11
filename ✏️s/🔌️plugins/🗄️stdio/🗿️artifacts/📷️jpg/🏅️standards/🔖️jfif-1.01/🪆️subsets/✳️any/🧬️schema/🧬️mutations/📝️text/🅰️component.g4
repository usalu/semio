grammar Stdio_jpg_mutations;
// JpgMutation's text form is standard JSON (see the sibling JSON Schema facet for the shape;
// `OpText` goes through `serde_json` directly) -- intentionally not re-deriving a JSON grammar.
document: JSON_VALUE EOF;
JSON_VALUE: .*? ;
