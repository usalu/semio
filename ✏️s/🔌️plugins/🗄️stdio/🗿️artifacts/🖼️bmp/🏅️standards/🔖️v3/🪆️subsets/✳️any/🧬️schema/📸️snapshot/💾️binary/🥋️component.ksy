meta:
  id: stdio_bmp_snapshot
  endian: le
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary) wrapping the REAL
  on-disk BMP bytes: BITMAPFILEHEADER + core BITMAPINFOHEADER + optional BI_BITFIELDS
  masks + optional palette + pixel data. See engine::decode_bmp / engine::encode_bmp for
  the authoritative reader/writer (this documents the byte layout those functions walk).
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.bmp.pack v1"
  - id: bmp_signature
    contents: [0x42, 0x4d]
    doc: "BM"
  - id: file_size
    type: u4
  - id: reserved1
    type: u2
  - id: reserved2
    type: u2
  - id: data_offset
    type: u4
    doc: byte offset of pixel_data from the start of bmp_signature
  - id: header_size
    type: u4
    doc: ">= 40; only the 40-byte core BITMAPINFOHEADER is decoded field-by-field (documented decode scope)"
  - id: width
    type: s4
  - id: height
    type: s4
    doc: sign selects row order (negative = top-down, positive = bottom-up)
  - id: planes
    type: u2
  - id: bits_per_pixel
    type: u2
    doc: "1, 4, 8, 16, 24, or 32"
  - id: compression
    type: u4
    doc: "0 = BI_RGB, 3 = BI_BITFIELDS; other values rejected"
  - id: image_size
    type: u4
  - id: x_pixels_per_meter
    type: s4
  - id: y_pixels_per_meter
    type: s4
  - id: colors_used
    type: u4
  - id: colors_important
    type: u4
  - id: bitfield_masks
    type: u4
    repeat: expr
    repeat-expr: 'compression == 3 ? (header_size == 40 ? 3 : (header_size >= 56 ? 4 : 3)) : 0'
    doc: r,g,b[,a] channel masks — present only when compression = BI_BITFIELDS
  - id: palette
    type: palette_entry
    repeat: expr
    repeat-expr: 'bits_per_pixel <= 8 ? (colors_used != 0 ? colors_used : (1 << bits_per_pixel)) : 0'
    doc: present only when bits_per_pixel <= 8
  - id: pixel_data
    size-eos: true
    doc: 'height rows, each row padded to a 4-byte boundary; on-disk row order per height''s sign'
types:
  palette_entry:
    seq:
      - id: b
        type: u1
      - id: g
        type: u1
      - id: r
        type: u1
      - id: reserved
        type: u1
