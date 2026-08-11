// 🅰️ `stdio.bmp`'s wire text (after the `semio ...` preamble line is stripped) IS a
// lowercase-hex dump of the real on-disk BMP bytes — see ../💾️binary/🥋️component.ksy for
// the field-by-field byte layout those bytes decode to (BITMAPFILEHEADER + BITMAPINFOHEADER
// + optional masks/palette + pixel data). Names the real encoding rather than a placeholder.
grammar Stdio_bmp_snapshot;

document : hexByte* EOF ;
hexByte  : HEXDIG HEXDIG ;
HEXDIG   : [0-9a-f] ;
