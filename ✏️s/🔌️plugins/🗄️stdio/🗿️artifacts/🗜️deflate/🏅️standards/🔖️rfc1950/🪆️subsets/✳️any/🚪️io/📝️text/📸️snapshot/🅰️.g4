grammar Stdio_deflate_snapshot;
// Hex-pair-encoded RFC1950 zlib stream (the `.zz` DSL text form).
document       : cmf flg dictId? compressedData adler32 ;
cmf            : HEXDIG HEXDIG ;                                       // CM (low nibble) / CINFO (high nibble)
flg            : HEXDIG HEXDIG ;                                       // FLEVEL/FDICT/FCHECK
dictId         : HEXDIG HEXDIG HEXDIG HEXDIG HEXDIG HEXDIG HEXDIG HEXDIG ; // present only when flg.FDICT set
compressedData : (HEXDIG HEXDIG)* ;                                    // raw RFC1951 DEFLATE stream, hex-encoded
adler32        : HEXDIG HEXDIG HEXDIG HEXDIG HEXDIG HEXDIG HEXDIG HEXDIG ;
HEXDIG         : [0-9a-fA-F] ;
