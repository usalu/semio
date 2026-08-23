@capability-stl-ascii-mutate
@oracle-stl-io-ascii-mutate
@comparison-semantic-mesh-v1
@mutations-stl-ascii-any
Feature: Apply every typed STL ascii mutation to a real-world mesh
  The input is shared://🧊️hexagonal-cut-concrete-forest-left.stl, a real 958-triangle ASCII STL
  derived ONCE (♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/🧊️hexagonal-cut-concrete-forest-left.glb, a
  real modelled hexagonal-cut concrete forest structure, 86 KB) by hand-parsing its GLB container
  (12-byte header, JSON chunk, BIN chunk — no glTF crate is linked) since it carries no vertex
  normals: the 57 triangle-mode primitives of its one mesh are flattened into one solid and each
  facet's normal is computed from vertex winding, then written as real ASCII STL text and committed
  here verbatim. Never a tetrahedron. Every scenario copies the fixture into the case work directory
  before touching it; the committed file is never written to.

  STL is triangle soup with no vertex index, so the projection reports resolved corner positions per
  triangle (the shared `🧊️mesh` family module's `project_stl`, built on the independent `stl_io`
  reader). `stl_io` 0.8 reads both ASCII and binary STL, but its own top-level doc comment states
  "Writing is limited to binary STL" — confirmed in its source, which hardcodes a zeroed 80-byte
  header with no name field — so it can express neither an ASCII body nor the solid name in either
  direction. This subset's OWN codec (`decode_stl_ascii`/`encode_stl_ascii`) is genuinely ASCII
  text, matching the `ascii` standard this subset is filed under, so the derived fixture above is
  committed in that form. Six of the seven declared kinds round-trip through `stl_io`'s
  `IndexedMesh`/`write_stl` regardless — comparison is projection-based, so the binary form the
  oracle emits there never has to match the subject's own ASCII output; `set-solid-name`, the one
  kind whose whole payload IS the field `stl_io` cannot touch, is instead applied by the oracle as a
  direct ASCII header/trailer substitution.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real mesh
    Given the real input mesh shared://🧊️hexagonal-cut-concrete-forest-left.stl
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params |
      | no-mutation            | {} |
      | set-snapshot           | {"triangles": [{"normal": [0.0, 0.0, 1.0], "vertices": [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]}]} |
      | set-solid-name         | {"name": "renamed-hexagonal-forest"} |
      | insert-triangle        | {"index": 500, "triangle": {"normal": [0.0, 0.0, 1.0], "vertices": [[100.0, 100.0, 50.0], [101.0, 100.0, 50.0], [100.0, 101.0, 50.0]]}} |
      | remove-triangle        | {"index": 500} |
      | set-triangle-normal    | {"index": 500, "normal": [0.0, 1.0, 0.0]} |
      | set-triangle-vertices  | {"index": 500, "vertices": [[1.0, 1.0, 1.0], [2.0, 1.0, 1.0], [1.0, 2.0, 1.0]]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real mesh
    Given the real input mesh shared://🧊️hexagonal-cut-concrete-forest-left.stl
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original mesh
    Examples:
      | id                    | params |
      | no-mutation            | {} |
      | set-snapshot           | {"triangles": [{"normal": [0.0, 0.0, 1.0], "vertices": [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]}]} |
      | set-solid-name         | {"name": "renamed-hexagonal-forest"} |
      | insert-triangle        | {"index": 500, "triangle": {"normal": [0.0, 0.0, 1.0], "vertices": [[100.0, 100.0, 50.0], [101.0, 100.0, 50.0], [100.0, 101.0, 50.0]]}} |
      | remove-triangle        | {"index": 500} |
      | set-triangle-normal    | {"index": 500, "normal": [0.0, 1.0, 0.0]} |
      | set-triangle-vertices  | {"index": 500, "vertices": [[1.0, 1.0, 1.0], [2.0, 1.0, 1.0], [1.0, 2.0, 1.0]]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real mesh without passing bytes through
    Given the real input mesh shared://🧊️hexagonal-cut-concrete-forest-left.stl
    When the mesh is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
