meta:
  id: stdio_gltf_mutation_wire
  title: stdio.gltf mutation binary encoding
doc: |
  Structured OpBinary frame. Tags 0..23 retain their original meanings; tags 24..27 are
  TransformNode, ReparentNode, BindNodeMesh, and BindPrimitiveMaterial.
seq:
  - id: format
    type: u1
    valid: 1
  - id: tag
    type: u1
    valid:
      max: 27
  - id: variant_payload
    size-eos: true
