grammar Stdio_xml_diff;
// XmlDiff's text form is standard JSON (see the sibling JSON Schema facet for the shape) --
// intentionally not re-deriving a JSON grammar here.
document: JSON_VALUE EOF;
JSON_VALUE: .*? ;
