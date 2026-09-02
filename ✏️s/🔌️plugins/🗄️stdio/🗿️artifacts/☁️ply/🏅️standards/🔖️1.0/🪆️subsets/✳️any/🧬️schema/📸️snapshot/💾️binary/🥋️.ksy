meta:
  id: stdio_ply_snapshot
  endian: le
doc: |
  Real PLY structure. The header (magic + format + comment/element/property declarations) is
  always ASCII text, terminated by the literal line `end_header\n` — captured below as a
  newline-delimited text block scanned up to that terminator line (see `header_lines`). The
  body's real per-element/per-property byte layout is declared BY the header at parse time
  (arbitrary named elements, scalar or count-prefixed list properties, 8 possible per-cell
  widths, either declared endianness) — a genuinely runtime-determined schema that Kaitai's
  static `seq:`/`type:` graph cannot express without per-file codegen (no first-class "read N
  properties whose types come from a previously-parsed string list" primitive exists here).
  `body_bytes` therefore reads the remainder as a plain byte array; the real per-element/property
  walk this represents is `decode_body_binary` in the Rust engine (mounted at
  `⚙️engine/🦀️.rs`), which parses the SAME header text this file's `header_lines`
  exposes and then reads exactly the widths the sibling `📡️.protocol.semio` documents.
seq:
  - id: header_lines
    type: header_line
    repeat: until
    repeat-until: _.text == "end_header"
  - id: body_bytes
    type: u1
    repeat: eos
types:
  header_line:
    seq:
      - id: text
        type: strz
        encoding: ASCII
        terminator: 0x0a
        include: false
