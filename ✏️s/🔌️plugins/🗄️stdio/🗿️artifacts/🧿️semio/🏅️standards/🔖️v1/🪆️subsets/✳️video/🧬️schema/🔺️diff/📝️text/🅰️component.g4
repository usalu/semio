grammar Stdio_semio_video_diff;
// Video wave: one line, empty if unchanged. 'streams=' index-keyed triple, recursive at the
// samples level. NOTE every option tag is THREE separate tokens ('[' '0' ']' / '[' '1' ',' ... ']')
// — a combined '[0]' single token never matches since '[' and ']' tokenize separately.
diff: (STREAMS_EQ triple)? EOF;
triple: '[' indexList '];[' modifiedList '];[' addedList ']';
indexList: (INDEX (',' INDEX)*)?;
modifiedList: (modifiedEntry (',' modifiedEntry)*)?;
modifiedEntry: INDEX ':' streamDiff;
addedList: (addedEntry (',' addedEntry)*)?;
addedEntry: INDEX ':' stream;
streamDiff: '[' optKind ',' optCodec ',' optWidth ',' optHeight ',' optRate ',' optSamples ']';
optKind: '[0]' | '[1,' KIND ']';
optSamples: '[0]' | '[1,' triple ']';
optWidth: '[0]' | '[1,' INDEX ']';
optHeight: '[0]' | '[1,' INDEX ']';
optCodec: '[0]' | '[1,' HEXSTR ']';
optRate: '[0]' | '[1,' rational ']';
stream: '[' KIND ',' HEXSTR ',' INDEX ',' INDEX ',' rational ',' sampleList ']';
sampleList: '[' (sample (',' sample)*)? ']';
sample: '[' INDEX ',' BOOL ',' HEXSTR ']';
rational: '[' INDEX ',' INDEX ']';
STREAMS_EQ: 'streams=';
KIND: 'V' | 'A' | 'S';
BOOL: '0' | '1';
INDEX: [0-9]+;
HEXSTR: [0-9a-f]*;
