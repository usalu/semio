meta:
  id: stdio_png_snapshot
  endian: be
doc: |
  Shared `.semio` binary envelope (store::semio_format::wrap_binary) wrapping a `stdio.png`
  payload: the REAL PNG 1.2 file bytes (§5 signature + repeating length-prefixed CRC-32'd
  chunks) that `crate::artifacts::png::engine::{encode_png,decode_png}` produce/consume.
seq:
  - id: envelope_magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
    endian: le
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.png.pack v1"
  - id: payload
    type: png_file
    doc: The real PNG §5-conformant file bytes.
types:
  png_file:
    doc: A conformant PNG 1.2 file — §5 signature followed by a chunk stream ending at IEND.
    seq:
      - id: png_signature
        contents: [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]
      - id: chunks
        type: chunk
        repeat: until
        repeat-until: _.chunk_type == "IEND"
  chunk:
    doc: One §5.3 chunk — 4-byte big-endian length, 4-byte ASCII type, `length` bytes of
      data, 4-byte CRC-32 over (type + data).
    seq:
      - id: length
        type: u4
      - id: chunk_type
        type: str
        size: 4
        encoding: ASCII
      - id: data
        size: length
        type:
          switch-on: chunk_type
          cases:
            '"IHDR"': ihdr_body
            '"PLTE"': plte_body
            '"gAMA"': gama_body
            '"cHRM"': chrm_body
            '"sRGB"': srgb_body
            '"pHYs"': phys_body
            '"tIME"': time_body
      - id: crc32
        type: u4
  ihdr_body:
    doc: §11.2.2 — the always-present image header chunk.
    seq:
      - id: width
        type: u4
      - id: height
        type: u4
      - id: bit_depth
        type: u1
      - id: color_type
        type: u1
      - id: compression_method
        type: u1
        doc: Always 0 per spec, validated (not modeled as a mutable field).
      - id: filter_method
        type: u1
        doc: Always 0 per spec, validated (not modeled as a mutable field).
      - id: interlace_method
        type: u1
  plte_body:
    doc: §11.2.3 — palette entries, 3 bytes (R,G,B) each.
    seq:
      - id: entries
        type: rgb
        repeat: eos
  rgb:
    seq:
      - id: r
        type: u1
      - id: g
        type: u1
      - id: b
        type: u1
  gama_body:
    doc: §11.3.5.2 — image gamma * 100000.
    seq:
      - id: gamma
        type: u4
  chrm_body:
    doc: §11.3.5.2 — CIE xy chromaticities, each * 100000.
    seq:
      - id: white_x
        type: u4
      - id: white_y
        type: u4
      - id: red_x
        type: u4
      - id: red_y
        type: u4
      - id: green_x
        type: u4
      - id: green_y
        type: u4
      - id: blue_x
        type: u4
      - id: blue_y
        type: u4
  srgb_body:
    doc: §11.3.5.3 — rendering intent (0=perceptual, 1=relative colorimetric, 2=saturation, 3=absolute colorimetric).
    seq:
      - id: rendering_intent
        type: u1
  phys_body:
    doc: §11.3.5.4 — pixel-per-unit density.
    seq:
      - id: pixels_per_unit_x
        type: u4
      - id: pixels_per_unit_y
        type: u4
      - id: unit_specifier
        type: u1
        doc: 0 = unknown, 1 = meter.
  time_body:
    doc: §11.3.6.1 — last modification time, UTC.
    seq:
      - id: year
        type: u2
      - id: month
        type: u1
      - id: day
        type: u1
      - id: hour
        type: u1
      - id: minute
        type: u1
      - id: second
        type: u1
