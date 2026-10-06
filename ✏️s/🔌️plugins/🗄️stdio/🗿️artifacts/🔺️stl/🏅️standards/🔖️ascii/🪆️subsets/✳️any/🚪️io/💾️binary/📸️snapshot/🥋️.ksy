meta:
  id: stdio_stl_snapshot
  endian: le
doc: |
  Binary STL (https://en.wikipedia.org/wiki/STL_(file_format)). The 80-byte header is opaque
  by spec but this codec uses it as `solid_name` (trailing NUL/whitespace trimmed).
seq:
  - id: header
    size: 80
  - id: num_triangles
    type: u4
  - id: triangles
    type: triangle
    repeat: expr
    repeat-expr: num_triangles
types:
  vec3:
    seq:
      - id: x
        type: f4
      - id: y
        type: f4
      - id: z
        type: f4
  triangle:
    seq:
      - id: normal
        type: vec3
      - id: v0
        type: vec3
      - id: v1
        type: vec3
      - id: v2
        type: vec3
      - id: attribute_byte_count
        type: u2
