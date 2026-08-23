@capability-obj-3-0-mutate
@oracle-tobj-obj-3-0-mutate
@comparison-semantic-mesh-v1
@mutations-obj-3-0-any
Feature: Apply every typed OBJ 3.0 mutation to a real-world mesh
  The input is a real 16,128-triangle mesh (8,449 real vertices/texcoords/normals each, one real
  material, three real named `o`/`g` bands plus a small real `apex` object/group carved out of
  band-0's own first 3 real faces) derived once from the real committed art asset
  🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb, not a synthetic fixture. The GLB
  container was hand-parsed (12-byte header, JSON chunk, BIN chunk; POSITION/NORMAL/TEXCOORD_0 and
  the index accessor read directly with plain struct decoding, no gltf crate) and re-emitted as real
  Wavefront OBJ text; the committed fixture's own header comments record the same derivation. One
  real vertex/texcoord/normal (a duplicate of index 0's real value) is appended once more,
  unreferenced by any face, so the `remove-vertex`/`remove-texcoord`/`remove-normal` scenarios have a
  real, exactly-known target that needs no cascading face-index repair. Every scenario copies the
  fixture into the case work directory before touching it; the committed mesh is never written to.
  Both the oracle's and the subject's results are read back by the INDEPENDENT `tobj` reader before
  the `semantic-mesh-v1` profile compares them: the vertex set, face topology and counts are
  normative; `o`/`g`/`usemtl`/`s`/`mtllib`/comment mutations are exercised for real (real band/
  object names, real face-index membership) but sit outside that profile's normative surface, same
  as generator strings and precision are writer freedom for mesh creation.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real mesh
    Given the real input mesh shared://🧊️pattern-sphere.obj
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                                                       |
      | no-mutation           | {}                                                                                            |
      | set-snapshot          | {"snapshot":{"vertices":[{"x":0,"y":0,"z":0},{"x":1,"y":0,"z":0},{"x":0,"y":1,"z":0}],"texcoords":[],"normals":[],"faces":[{"vertices":[{"vertex":0},{"vertex":1},{"vertex":2}]}],"groups":[],"objects":[],"mtllib":null,"usemtlRanges":[],"smoothingGroups":[],"unknownStatements":[]}} |
      | insert-vertex         | {"index":8449,"vertex":{"x":0.5,"y":0.5,"z":0.5}}                                            |
      | remove-vertex         | {"index":8448}                                                                               |
      | set-vertex            | {"index":0,"vertex":{"x":1,"y":2,"z":3}}                                                     |
      | insert-texcoord       | {"index":8449,"texcoord":{"u":0.25,"v":0.75}}                                                |
      | remove-texcoord       | {"index":8448}                                                                               |
      | set-texcoord          | {"index":0,"texcoord":{"u":0.1,"v":0.9}}                                                     |
      | insert-normal         | {"index":8449,"normal":{"x":0,"y":1,"z":0}}                                                  |
      | remove-normal         | {"index":8448}                                                                               |
      | set-normal            | {"index":0,"normal":{"x":1,"y":0,"z":0}}                                                     |
      | insert-face           | {"index":16128,"face":{"vertices":[{"vertex":0},{"vertex":1},{"vertex":2}]}}                 |
      | remove-face           | {"index":16127}                                                                              |
      | set-face              | {"index":16127,"face":{"vertices":[{"vertex":2},{"vertex":1},{"vertex":0}]}}                 |
      | set-group             | {"name":"equator","faces":[0,1,2]}                                                           |
      | remove-group          | {"name":"apex-band"}                                                                         |
      | set-object            | {"name":"north-cap","faces":[0,1,2]}                                                         |
      | remove-object         | {"name":"apex"}                                                                              |
      | set-mtllib            | {"mtllib":"pattern-sphere.mtl"}                                                              |
      | set-usemtl            | {"usemtl":[{"faceIndexFrom":0,"material":"clay"}]}                                           |
      | set-smoothing-groups  | {"smoothingGroups":[{"faceIndexFrom":0,"group":1}]}                                          |
      | set-unknown-statements | {"unknownStatements":[{"lineIndex":0,"raw":"# replaced by mutation"}]}                       |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real mesh
    Given the real input mesh shared://🧊️pattern-sphere.obj
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the mesh matches its pre-mutation semantic projection
    Examples:
      | id                    | params                                                                                       |
      | no-mutation           | {}                                                                                            |
      | set-snapshot          | {"snapshot":{"vertices":[{"x":0,"y":0,"z":0},{"x":1,"y":0,"z":0},{"x":0,"y":1,"z":0}],"texcoords":[],"normals":[],"faces":[{"vertices":[{"vertex":0},{"vertex":1},{"vertex":2}]}],"groups":[],"objects":[],"mtllib":null,"usemtlRanges":[],"smoothingGroups":[],"unknownStatements":[]}} |
      | insert-vertex         | {"index":8449,"vertex":{"x":0.5,"y":0.5,"z":0.5}}                                            |
      | remove-vertex         | {"index":8448}                                                                               |
      | set-vertex            | {"index":0,"vertex":{"x":1,"y":2,"z":3}}                                                     |
      | insert-texcoord       | {"index":8449,"texcoord":{"u":0.25,"v":0.75}}                                                |
      | remove-texcoord       | {"index":8448}                                                                               |
      | set-texcoord          | {"index":0,"texcoord":{"u":0.1,"v":0.9}}                                                     |
      | insert-normal         | {"index":8449,"normal":{"x":0,"y":1,"z":0}}                                                  |
      | remove-normal         | {"index":8448}                                                                               |
      | set-normal            | {"index":0,"normal":{"x":1,"y":0,"z":0}}                                                     |
      | insert-face           | {"index":16128,"face":{"vertices":[{"vertex":0},{"vertex":1},{"vertex":2}]}}                 |
      | remove-face           | {"index":16127}                                                                              |
      | set-face              | {"index":16127,"face":{"vertices":[{"vertex":2},{"vertex":1},{"vertex":0}]}}                 |
      | set-group             | {"name":"equator","faces":[0,1,2]}                                                           |
      | remove-group          | {"name":"apex-band"}                                                                         |
      | set-object            | {"name":"north-cap","faces":[0,1,2]}                                                         |
      | remove-object         | {"name":"apex"}                                                                              |
      | set-mtllib            | {"mtllib":"pattern-sphere.mtl"}                                                              |
      | set-usemtl            | {"usemtl":[{"faceIndexFrom":0,"material":"clay"}]}                                           |
      | set-smoothing-groups  | {"smoothingGroups":[{"faceIndexFrom":0,"group":1}]}                                          |
      | set-unknown-statements | {"unknownStatements":[{"lineIndex":0,"raw":"# replaced by mutation"}]}                       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real mesh without passing bytes through
    Given the real input mesh shared://🧊️pattern-sphere.obj
    When the mesh is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the oracle and the subject agree on the semantic projection
