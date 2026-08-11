meta:
  id: semio_presentation_snapshot
  endian: le
doc: |
  Binary envelope for `stdio.semio.semio.presentation` snapshots: `SemioEnvelope` header
  (component=pack, version) followed by the serde_json-serialized `SemioPresentationSnapshot`
  payload verbatim (opaque at this binary-envelope level -- see the sibling JSON Schema for the
  payload's own structured shape).
seq:
  - id: envelope_id_len
    type: u4
  - id: envelope_id
    type: str
    size: envelope_id_len
    encoding: UTF-8
  - id: component_tag
    type: u1
  - id: version
    type: u4
  - id: payload_len
    type: u8
  - id: payload
    size: payload_len
