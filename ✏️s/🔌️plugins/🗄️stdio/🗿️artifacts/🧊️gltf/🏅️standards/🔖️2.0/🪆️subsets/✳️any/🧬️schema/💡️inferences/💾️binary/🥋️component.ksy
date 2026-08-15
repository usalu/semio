meta:
  id: gltf_inference
  endian: le
  title: Canonical glTF geometric inference envelope
seq:
  - id: magic
    contents: [0x89, 0x53, 0xf8, 0x3f, 0x7d, 0x34, 0x0d, 0x0b]
  - id: format_major
    type: u2
    valid: 1
  - id: format_minor
    type: u2
    valid: 0
  - id: schema_version
    type: u4
    valid: 2
  - id: flags
    type: u4
    valid: 1
  - id: schema_crc32
    type: u4
    valid: 0x6b257ae0
  - id: payload_length
    type: u8
  - id: payload_crc32
    type: u4
    doc: CRC-32/ISO-HDLC of payload
  - id: header_crc32
    type: u4
    doc: CRC-32/ISO-HDLC of bytes 0 through 35
  - id: payload
    size: payload_length
    encoding: UTF-8
    doc: RFC 8785 canonical JSON encoding of GltfInference with geometry root
