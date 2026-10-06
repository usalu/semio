meta:
  id: stdio_deflate_snapshot
  endian: be
  title: RFC1950 zlib stream (stdio.deflate binary/pack representation)
seq:
  - id: cmf
    type: u1
    doc: low nibble = compression method (8 = deflate), high nibble = CINFO (window bits)
  - id: flg
    type: u1
    doc: bits 6-7 FLEVEL, bit 5 FDICT, bits 0-4 FCHECK (cmf*256+flg must be a multiple of 31)
  - id: dict_id
    type: u4
    if: (flg & 0x20) != 0
    doc: preset-dictionary Adler-32 id (DICTID), present only when flg.FDICT is set
  - id: compressed_data
    size: _io.size - _io.pos - 4
    doc: raw RFC1951 DEFLATE stream (opaque bitstream; no bit-level Huffman grammar expressible here)
  - id: adler32
    type: u4
    doc: Adler-32 checksum (RFC1950 §2.3) of the DECOMPRESSED payload, not of compressed_data
