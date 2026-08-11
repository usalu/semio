grammar Stdio_deflate_mutations;
// DeflateMutation wire form: tagged camelCase JSON object, `mutation` discriminates the variant.
document              : noMutation | setSnapshot | setCompressionParams | setPresetDictionary | setPayload ;
noMutation            : '{"mutation":"noMutation"}' ;
setSnapshot           : '{"mutation":"setSnapshot","snapshot":' SNAPSHOT_OBJECT '}' ;
setCompressionParams  : '{"mutation":"setCompressionParams","method":' DIGIT+ ',"windowBits":' DIGIT+ ',"levelHint":' LEVEL_HINT '}' ;
setPresetDictionary   : '{"mutation":"setPresetDictionary","dictId":' ( 'null' | DIGIT+ ) '}' ;
setPayload            : '{"mutation":"setPayload","payload":[' (DIGIT+ (',' DIGIT+)*)? ']}' ;
LEVEL_HINT            : '"fastest"' | '"fast"' | '"default"' | '"maximum"' ;
SNAPSHOT_OBJECT        : .*? ; // see sibling 📸️snapshot facet for the full DeflateSnapshot JSON shape
DIGIT                 : [0-9] ;
