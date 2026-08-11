meta:
  id: semio_workflow_snapshot
  endian: le
doc: |
  `s.stdio.semio.workflow` snapshot binary wire format — a `store::semio_format` binary envelope
  (magic + component + version header, see `SemioEnvelope`) wrapping `serde_json::to_vec` of
  `SemioWorkflowSnapshot` verbatim (`ArtifactPack::encode_pack_with`/`decode_pack_with`). Honest
  opaque-payload boundary: this subset's snapshot is a NEUTRAL semio type, not an on-disk file
  format with its own byte layout.
seq:
  - id: envelope_id
    type: str
    size: 20
    encoding: ASCII
    doc: "stdio.semio.workflow"
  - id: component_tag
    type: u1
    doc: "store::semio_format::Component::Pack"
  - id: version
    type: u1
  - id: json_payload
    size-eos: true
    doc: "serde_json::to_vec(SemioWorkflowSnapshot) — {schema, nodes, edges}"
