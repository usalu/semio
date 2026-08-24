@capability-stl-ascii-mutate
@oracle-stl-io-ascii-mutate
@comparison-semantic-stl-ascii-v1
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

  STL is triangle soup with no vertex index, so the mesh half of the projection reports resolved
  corner positions per triangle (the shared `🧊️mesh` family module's `project_stl`, built on the
  independent `stl_io` reader). That half alone is not enough to judge this vocabulary. STL STATES
  each facet's normal rather than deriving it from winding, and it names the solid in its header;
  `set-triangle-normal` and `set-solid-name` change exactly those two fields and nothing else, so
  under the shared `semantic-mesh-v1` profile — which lists `solidName` as writer freedom and
  reports no normals at all — 2 of the 7 declared kinds moved nothing and their rows measured
  nothing. `semantic-stl-ascii-v1` makes both normative, the projection carries them, and every
  `mutate-<kind>` scenario asserts in role that the kind really did move it.

  `stl_io` 0.8 reads both ASCII and binary STL, and that reader is what every result here is read
  back through. Its WRITER is the half that cannot serve: its own top-level doc comment states
  "Writing is limited to binary STL" — confirmed in its source, which hardcodes a zeroed 80-byte
  header with no name field — and binary STL has no solid-name field at all, so emitting through it
  would discard the name on every kind and leave this `ascii` subset's oracle never once producing
  the grammar it is filed under. The oracle therefore writes the ASCII document itself, from the
  triangle soup `stl_io` parsed, the same precedent the OBJ case follows for a format whose Rust
  reference is a reader; nothing in it touches this repository's `decode_stl_ascii`/
  `encode_stl_ascii`. `no-mutation` is a real parse and re-emission for the same reason — it used to
  hand the input bytes straight back. Byte-identical output stays impossible either way: `stl_io`
  resolves every coordinate through `f32` while the committed fixture carries the `f64` decimals its
  GLB derivation produced.

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
