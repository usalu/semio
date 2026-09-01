meta:
  id: stdio_xml_snapshot
  endian: le
doc: |
  Semio binary envelope wrapping the JSON serialization of XmlDocument (root/doctype/declaration).
  NOT raw XML text -- ArtifactPack::encode_pack_with serde_json-encodes `doc`, then wraps that in
  the semio envelope. The `json_payload` bytes below, once envelope-unwrapped, parse as JSON per
  the sibling ../🔣️component.json XmlDocument shape.
seq:
  - id: json_payload
    size-eos: true
