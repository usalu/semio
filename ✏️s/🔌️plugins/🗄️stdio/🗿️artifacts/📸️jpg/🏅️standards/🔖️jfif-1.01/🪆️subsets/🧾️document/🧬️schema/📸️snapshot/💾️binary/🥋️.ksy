meta:
  id: stdio_jpg_snapshot
  endian: be
doc: |
  Real pack encoding: a semio binary envelope (magic/schema-id/version header, see the repo's
  `store::semio_format` module) wrapping the ACTUAL JFIF 1.01 byte stream `engine::encode_jpg`
  produces (ITU-T T.81 / ISO 10918-1 marker-segment structure) — not a raw octet blob.
seq:
  - id: envelope_magic
    contents: [0x53, 0x45, 0x4d, 0x49] # "SEMI" — semio pack envelope magic (see semio_format)
  - id: envelope_header
    size: 28 # schema id hash + component + version + payload length, format-internal
  - id: jpeg
    type: jpeg_stream
types:
  jpeg_stream:
    seq:
      - id: soi
        contents: [0xff, 0xd8]
      - id: segments
        type: segment
        repeat: until
        repeat-until: _.marker == 0xda # stop at SOS, entropy-coded data follows non-marker-aligned
      - id: sos
        type: sos_segment
      - id: entropy_and_eoi
        size-eos: true # byte-stuffed entropy-coded scan data + trailing EOI (0xffd9); a real
                        # entropy decoder (this codec's `decode_scan`) must byte-destuff and
                        # watch for restart/EOI markers inline — not expressible as a flat ksy
                        # field, documented here rather than faked as further marker structure.
  segment:
    seq:
      - id: marker_prefix
        contents: [0xff]
      - id: marker
        type: u1
      - id: length
        type: u2
      - id: body
        size: length - 2
  sos_segment:
    seq:
      - id: marker_prefix
        contents: [0xff, 0xda]
      - id: length
        type: u2
      - id: num_components
        type: u1
      - id: components
        type: scan_component
        repeat: expr
        repeat-expr: num_components
      - id: spectral_start
        type: u1
      - id: spectral_end
        type: u1
      - id: approx
        type: u1
  scan_component:
    seq:
      - id: component_id
        type: u1
      - id: dc_ac_table
        type: u1
