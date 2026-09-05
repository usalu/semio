grammar Stdio_semio_video_snapshot;
// Video wave: real structured DSL body — two lines, `schema=<hex>` then `streams=[<stream>,...]`
// — mirroring 📖️.grammar.semio's real dialect productions (descriptive, not test-parsed).
document: ARTIFACT_MARK schemaLine streamsLine EOF;
schemaLine: 'schema' '=' HEXSTR;
streamsLine: 'streams' '=' '[' (stream (',' stream)*)? ']';
stream: '[' KIND ',' HEXSTR ',' INDEX ',' INDEX ',' rational ',' '[' (sample (',' sample)*)? ']' ']';
sample: '[' INDEX ',' BOOL ',' HEXSTR ']';
rational: '[' INDEX ',' INDEX ']';
ARTIFACT_MARK: 'stdio.semio.video';
KIND: 'V' | 'A' | 'S';
BOOL: '0' | '1';
INDEX: [0-9]+;
HEXSTR: [0-9a-f]*;
