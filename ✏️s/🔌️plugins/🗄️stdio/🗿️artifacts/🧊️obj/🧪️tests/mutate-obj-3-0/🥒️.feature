@capability-obj-3-0-mutate
@oracle-tobj-obj-3-0-mutate
@comparison-semantic-obj-3-0-v1
@mutations-obj-3-0-any
Feature: Apply every typed OBJ 3.0 mutation to a real-world mesh
  The input is a real 16,128-triangle mesh derived once from the real committed art asset
  `🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb`, not a synthetic fixture. The GLB
  container was hand-parsed (12-byte header, JSON chunk, BIN chunk; POSITION/NORMAL/TEXCOORD_0 and
  the index accessor read directly with plain struct decoding, no gltf crate) and re-emitted as real
  Wavefront OBJ text; the committed fixture's own header comments record the same derivation. It
  declares 8,449 `v`/`vt`/`vn` rows each, one `mtllib`-less `usemtl pattern` run covering every
  face, exactly one object — `o pattern-sphere`, all 16,128 faces — and exactly three groups,
  `g band-0`, `g band-1` and `g band-2`, holding faces 0–5375, 5376–10751 and 10752–16127. Those
  four names are the only ones the `set-group`/`remove-group`/`set-object`/`remove-object` rows
  address; the adapter reads each one's membership back out of the file rather than restating it,
  so a row naming a band the mesh does not carry fails outright instead of inverting into a
  fabrication. One real vertex/texcoord/normal (a duplicate of index 0's real value) is appended
  once more, unreferenced by any face, so the `remove-vertex`/`remove-texcoord`/`remove-normal`
  scenarios have a real, exactly-known target that needs no cascading face-index repair. Every
  scenario copies the fixture into the case work directory before touching it; the committed mesh
  is never written to.

  Both roles' results are read back by INDEPENDENT readers before `semantic-obj-3-0-v1` compares
  them. `tobj` supplies the mesh half — and on its own it is not enough to judge this vocabulary:
  it triangulates, re-indexes per `o`/`g` model and drops every declared row no face references, so
  14 of the 22 kinds move nothing in it at all. (It also splits a model at every `o`/`g` transition,
  which is why the projected vertex count is 8,576 rather than 8,449 and why a face that ends up in
  no band is immediately visible.) The other half is the document surface a mesh reader cannot see —
  declared `v`/`vt`/`vn` row counts with their per-component extent and totals, the `mtllib`
  reference, the `g`/`o` membership spans, the `usemtl`/`s` run starts and the retained comment
  lines — read by the oracle module's own grammar parser, which never touches `decode_obj`. Extent
  and totals rather than a text digest of the rows, because the two producers format decimals
  differently and only numbers survive that under the profile's 1e-5 tolerance. With both halves in
  the projection every declared kind is observable, and each `mutate-<kind>` scenario asserts in
  role that it really did move it — a row whose parameters leave the document where it was fails
  instead of passing.

  ↩️ `inverse-remove-face` was this case's one reproduced FAILURE and is now fixed at the cause, so
  the shape of the defect is recorded here rather than lost. Both roles apply the forward mutation,
  serialize, re-read and then apply the inverse — the wire is deliberately in the middle, because
  that is what this ticket tests. Face 16127 belongs to `g band-2` and to `o pattern-sphere`.
  Removing it necessarily drops it from both (a membership list keyed by face index cannot keep a
  member the document no longer has), and `InsertFace` carries a face but no membership, so a lone
  `InsertFace` restored the row into no band and no object: `tobj` read that as a fourth model and
  the projection reported 8,577 vertices where the real mesh has 8,576.

  `Mutation::inverse` returns `Vec<Self>`, and that is where the repair went.
  `../../🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` now inverts
  `RemoveFace` as `[InsertFace, SetGroup…, SetObject…]`, re-declaring every membership list that
  names a face at or after the removed index — the removed face's own bands included — with the
  exact list the pre-mutation document carries; this adapter computes the same sequence
  independently from the real file. The same "restore the prior index space, do not re-insert by
  value" reading fixed two sibling kinds that were passing only by luck of this fixture's shape:
  `RemoveGroup`/`RemoveObject` used to invert through a single `SetGroup`/`SetObject`, which
  APPENDS, so the band came back at the end of the list rather than at its own position — invisible
  while the three bands are disjoint, visible the moment two bands share a face and the `g a b`
  token order is decided by that list. Their inverses now lift the tail off and re-declare it in
  order. `InsertVertex`/`InsertTexcoord`/`InsertNormal`/`InsertFace` likewise invert at the CLAMPED
  landing index rather than the requested one.

  🔴 The Rust SUBJECT phase ran for this case for the first time and reproduced a SECOND real
  defect the oracle phase could not see, because it lives in our own writer rather than in the
  reference: `mutate-set-object` left the compared projection bit-for-bit identical to the
  untouched input. `o` is a sticky statement and `encode_obj` had no way to END an object run — it
  emitted `o <name>` on a transition into an object and nothing at all on a transition out — so
  `SetObject {name: "pattern-sphere", faces: [0,1,2]}` moved the snapshot and then re-rendered a
  document in which `o pattern-sphere` still ran to end-of-file. Re-reading it handed back the
  original membership over all 16,128 faces: the mutation was unobservable and the row measured
  nothing. `decode_obj` already read an argument-less `o` as "no object from here on", and this
  case's reference writes exactly that (`../../🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧪️oracle/
  🦀️component.rs` renders `o\n` when the object run ends), so only our encoder was short of the
  grammar. It now emits the bare `o` terminator, mirroring the bare `g` it already wrote when a
  group run ends, pinned by `🚪️io/🦀️component.rs`'s own
  `an_object_run_that_ends_is_closed_with_a_bare_o`. Nothing in this case was relaxed to reach it:
  the `mutate-<kind>` observability assertion, the profile and the fixture are untouched.

  🧭️ Still open and deliberately NOT patched here, because no law in this case measures it and
  changing it is a vocabulary decision rather than a repair: neither producer renumbers the `v`/
  `vt`/`vn` index space when a row is removed, so a `remove-vertex` leaves every later `f` reference
  pointing one row off, and our own `RemoveFace`/`InsertFace` do not renumber the `g`/`o`/`usemtl`/
  `s` face-index space the way this case's reference implementation does. Both sides agree today —
  which is exactly why the differential comparison cannot see it — and the `remove-vertex`/
  `remove-texcoord`/`remove-normal` rows target the fixture's deliberate unreferenced orphan row, so
  no scenario walks into it.

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
      | set-group             | {"name":"band-0","faces":[0,1,2]}                                                            |
      | remove-group          | {"name":"band-0"}                                                                            |
      | set-object            | {"name":"pattern-sphere","faces":[0,1,2]}                                                    |
      | remove-object         | {"name":"pattern-sphere"}                                                                    |
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
      | set-group             | {"name":"band-0","faces":[0,1,2]}                                                            |
      | remove-group          | {"name":"band-0"}                                                                            |
      | set-object            | {"name":"pattern-sphere","faces":[0,1,2]}                                                    |
      | remove-object         | {"name":"pattern-sphere"}                                                                    |
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
