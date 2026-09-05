meta:
  id: stdio_dxf_mutations
  endian: le
doc: |
  DxfMutation's wire binary IS its serde_json encoding (protocol::OpBinary); the shared
  `.semio` envelope wraps that JSON payload.
seq:
  - id: magic
    contents: [0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]
  - id: token_len
    type: u4
  - id: token
    type: str
    size: token_len
    encoding: UTF-8
    doc: "stdio.dxf.mutation v1"
  - id: payload
    type: str
    size-eos: true
    encoding: UTF-8
    doc: JSON bytes of DxfMutation.
