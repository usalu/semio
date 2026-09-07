meta:
  id: remodeling_snapshot
  title: semio remodeling snapshot — .spk document container (post SEMIO-envelope unwrap)
  endian: le
doc: |
  What `RemodelingSnapshot::encode_pack_with` produces once `semio_format::unwrap_binary` has
  peeled the framework SEMIO envelope: the framework-generic `.spk` document container written by
  `os_pack::encode_document` (🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs). Segment order for this
  artifact is symbols(3), one-or-more document(4), manifest(1), end(0).

  A segment's `payload` stays an opaque byte run in `seq` because it is deflate-compressed whenever
  `flags & 1` is set; the structures it carries once decompressed are spelled out as unreferenced
  `types` below, and they are exact, not approximate:

  * the symbols(3) payload is `os_pack::format::encode_symbols` — `count varint, count*(len varint,
    utf8)` (`symbols_segment`);
  * the document(4) payloads, concatenated in file order, are one `encode_record_fields` body —
    `field_count varint, field_count*(field_id varint, tagged value)` with entries always sorted by
    ascending field id and `FieldValue::Absent` fields never written (`record_fields`). That is why
    the stream is self-describing yet fully declarative: every value starts with a one-byte tag from
    the `value_tag` enum, so an unknown field id still decodes and round-trips.

  Top-level field ids come from `#[derive(dsl::DslRecord)]`, which assigns `id = <declaration
  index>` (0-based, `🗣️dsl/✨️derive/🦀️.rs`). For `RemodelingSnapshot::__dsl_spec()` they are:

  | id | key                | Rust field           | Shape                    | value tag |
  |----|--------------------|----------------------|--------------------------|-----------|
  | 0  | schema             | schema               | Text                     | 0x06 str symref / 0x07 inline |
  | 1  | id                 | id                   | Text                     | 0x06 / 0x07 |
  | 2  | streams            | streams              | Table(MediaStream)       | 0x14 table-soa |
  | 3  | assets             | assets               | Map(RemodelingAssetChild)| 0x10 map |
  | 4  | durable-artifacts  | durable_artifacts    | Map(RemodelingDurableArtifact) | 0x10 map |
  | 5  | calibration        | calibration          | Block(Record)            | 0x0E block -> 0x0D record |
  | 6  | params             | params               | Block(Record)            | 0x0E -> 0x0D |
  | 7  | gcps               | gcps                 | Table(GroundControlPoint)| 0x14 table-soa |
  | 8  | job                | job                  | Block(Record)            | 0x0E -> 0x0D |
  | 9  | results            | results              | Block(Record)            | 0x0E -> 0x0D |

  A `0x0D` record value is itself a nested `record_fields` body (same 0-based per-struct field ids),
  so the whole document is this one production applied recursively.
seq:
  - id: magic
    contents: [0x89, 0x53, 0x50, 0x4b, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: header
    type: spk_header
    size: 24
  - id: segments
    type: segment
    repeat: until
    repeat-until: _.kind == 0
  - id: footer
    type: spk_footer
    size: 84
types:
  spk_header:
    seq:
      - id: version_major
        type: u2
      - id: version_minor
        type: u2
      - id: required_flags
        type: u4
      - id: optional_flags
        type: u4
      - id: header_crc32
        type: u4
      - id: reserved
        size: 8
  segment:
    seq:
      - id: kind
        type: u1
        enum: segment_kind
      - id: flags
        type: u1
      - id: stored_len
        type: vlq_base128_le
      - id: raw_len
        type: vlq_base128_le
        if: (flags & 1) != 0
      - id: payload
        size: stored_len.value
      - id: crc32
        type: u4
  spk_footer:
    seq:
      - id: magic
        contents: "SPKFOOT1"
      - id: version_major
        type: u2
      - id: version_minor
        type: u2
      - id: required_flags
        type: u4
      - id: manifest_offset
        type: u8
      - id: manifest_len
        type: u8
      - id: file_len
        type: u8
      - id: content_hash
        size: 32
      - id: prev_footer_offset
        type: u8
      - id: footer_crc32
        type: u4
  symbols_segment:
    doc: Decompressed payload of the symbols(3) segment (`os_pack::format::encode_symbols`).
    seq:
      - id: count
        type: vlq_base128_le
      - id: symbols
        type: symbol
        repeat: expr
        repeat-expr: count.value
  symbol:
    seq:
      - id: len
        type: vlq_base128_le
      - id: text
        type: str
        size: len.value
        encoding: UTF-8
  record_fields:
    doc: |
      Decompressed document(4) payloads concatenated: `os_pack::encode_record_fields`. Entries are
      sorted by ascending `field_id` and `Absent` fields are omitted, which is what makes the
      encoding byte-identical for equal `(spec, record)` regardless of map iteration order.
    seq:
      - id: field_count
        type: vlq_base128_le
      - id: fields
        type: field_entry
        repeat: expr
        repeat-expr: field_count.value
  field_entry:
    seq:
      - id: field_id
        type: vlq_base128_le
      - id: tag
        type: u1
        enum: value_tag
      - id: value
        size-eos: true
        doc: Tag-driven payload; a `record`(0x0D) value is a nested `record_fields` body.
enums:
  value_tag:
    0x00: absent
    0x01: false_
    0x02: true_
    0x03: int
    0x04: uint
    0x05: f64
    0x06: str_symref
    0x07: str_inline
    0x08: bytes
    0x09: bytes_chunked
    0x0a: enum_ordinal
    0x0b: tuple
    0x0c: list
    0x0d: record
    0x0e: block
    0x0f: statements
    0x10: map
    0x11: value
    0x12: null_value
    0x13: wire
    0x14: table_soa
    0x15: packed_f64
    0x16: packed_varint
    0x17: expr
  segment_kind:
    0x00: end
    0x01: manifest
    0x02: schema
    0x03: symbols
    0x04: document
    0x05: chunk
    0x06: chunk_table
    0x07: snapshot
    0x08: field_index
    0x7f: padding
