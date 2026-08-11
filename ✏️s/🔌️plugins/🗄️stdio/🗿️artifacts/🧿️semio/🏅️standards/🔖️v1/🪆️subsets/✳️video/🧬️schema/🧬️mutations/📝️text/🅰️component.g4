grammar Stdio_semio_video_mutations;
// "no-mutation" or "<keyword> arg=value ...", space-separated.
op: NO_MUTATION
  | SET_SNAPSHOT_KW ' snapshot=' value
  | INSERT_STREAM_KW ' index=' INDEX ' stream=' streamValue
  | REMOVE_STREAM_KW ' index=' INDEX
  | SET_STREAM_META_KW ' index=' INDEX ' kind=' KIND ' codec=' HEXSTR ' width=' INDEX ' height=' INDEX ' rate=' rational
  | INSERT_SAMPLE_KW ' stream-index=' INDEX ' index=' INDEX ' sample=' sampleValue
  | REMOVE_SAMPLE_KW ' stream-index=' INDEX ' index=' INDEX
  | SET_SAMPLE_DATA_KW ' stream-index=' INDEX ' index=' INDEX ' data=' HEXSTR
  | SET_SAMPLE_FLAGS_KW ' stream-index=' INDEX ' index=' INDEX ' pts=' INDEX ' key=' BOOL
  ;
value: '[' HEXSTR ',' streamList ']'; // whole SemioVideoSnapshot: [schema, streams]
streamList: '[' (streamValue ',')* ']';
streamValue: '[' KIND ',' HEXSTR ',' INDEX ',' INDEX ',' rational ',' sampleList ']';
sampleList: '[' (sampleValue ',')* ']';
sampleValue: '[' INDEX ',' BOOL ',' HEXSTR ']';
rational: '[' INDEX ',' INDEX ']';
NO_MUTATION: 'no-mutation';
SET_SNAPSHOT_KW: 'set-snapshot';
INSERT_STREAM_KW: 'insert-stream';
REMOVE_STREAM_KW: 'remove-stream';
SET_STREAM_META_KW: 'set-stream-meta';
INSERT_SAMPLE_KW: 'insert-sample';
REMOVE_SAMPLE_KW: 'remove-sample';
SET_SAMPLE_DATA_KW: 'set-sample-data';
SET_SAMPLE_FLAGS_KW: 'set-sample-flags';
KIND: 'V' | 'A' | 'S';
BOOL: '0' | '1';
INDEX: [0-9]+;
HEXSTR: [0-9a-f]*;
