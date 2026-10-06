meta:
  id: stdio_gltf_diff_wire
  title: stdio.gltf diff binary encoding
doc: |
  Structured 21-slot GltfDiff wire. The Semio protocol leaf is authoritative for conditional
  unsigned-LEB128 lengths and tri-state fields.
seq:
  - id: ordered_diff_slots
    size-eos: true
