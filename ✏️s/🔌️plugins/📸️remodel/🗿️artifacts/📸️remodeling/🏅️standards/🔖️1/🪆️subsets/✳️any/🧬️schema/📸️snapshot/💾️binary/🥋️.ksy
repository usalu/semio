meta:
  id: remodeling_snapshot
  title: semio remodeling snapshot — .spk document container (post SEMIO-envelope unwrap)
  endian: le
doc: |
  What `RemodelingSnapshot::encode_pack_with` produces once `semio_format::unwrap_binary` has
  peeled the framework SEMIO envelope: the framework-generic `.spk` document container written by
  `os_pack::encode_document` (🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs). Segment order for this
  artifact is symbols(3), one-or-more document(4), manifest(1), end(0). Segment payload bytes are
  `encode_record_fields` output against `RemodelingSnapshot::__dsl_spec()`; they stay opaque here
  because that encoding is a self-describing, recursive per-field-id tag stream, not a fixed layout.
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
enums:
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
