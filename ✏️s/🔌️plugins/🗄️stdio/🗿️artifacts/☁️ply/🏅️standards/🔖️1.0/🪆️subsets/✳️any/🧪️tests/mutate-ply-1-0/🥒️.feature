@capability-ply-1-0-mutate
@oracle-ply-rs-ply-1-0-mutate
@comparison-semantic-ply-v1
@mutations-ply-1-0-any
Feature: Apply every typed PLY 1.0 mutation to a real-world document
  The input is a real ASCII PLY derived once from the real committed art asset
  `🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb` (679 KB), not a synthetic fixture. The
  GLB container was hand-parsed (12-byte header, JSON chunk, BIN chunk; POSITION/NORMAL/TEXCOORD_0 and
  the index accessor read directly with plain struct decoding, no gltf crate) and re-emitted as real
  PLY text: a "vertex" element of 8,449 real rows (8,448 real positions/normals/texcoords plus one
  real trailing duplicate of row 0, unreferenced by any face) each carrying real `x y z nx ny nz s t`
  float properties, a "face" element of 16,128 real triangles (`property list uchar int
  vertex_indices`), and a small "edge" element of 50 real undirected edges extracted from the mesh's
  own first 24 real triangles. Every scenario copies the fixture into the case work directory before
  touching it; the committed document is never written to.

  Both the oracle's and the subject's results are read back by the INDEPENDENT `ply-rs` reader before
  the `semantic-ply-v1` profile compares them. Unlike a pure mesh profile, comments and element/
  property declarations are themselves normative here — `InsertComment`/`RemoveComment`/`AddElement`/
  `RemoveElement`/`SetFormat` mutate exactly that surface, so nothing about the header is producer
  freedom the way generator strings are for OBJ/STL.

  `ply-rs` 0.1.3 genuinely reads and writes both ascii and binary (little/big endian) PLY, with its own
  real write/read round-trip test — a normal differential oracle per the fleet brief's §6 first branch,
  registered under its own id/capability since no shared stdio entry exists for ply yet. One real,
  reproduced-standalone defect in its BINARY writer (the per-row list-length prefix is computed from
  the ELEMENT's total row count instead of that row's own list length, corrupting any binary output
  with a list property) is worked around in the oracle module rather than hidden — see that module's
  own doc comment — so `set-format`'s conversion to `binaryLittleEndian` genuinely exercises real
  binary list encoding rather than avoiding it.

  🔴 The Rust SUBJECT phase ran for this case for the first time and reproduced two defects on OUR
  side of the comparison, both in this adapter's wiring rather than in the codec or the reference.
  First, the subject encoded through `encode_ply`, which is the ascii-FORCING convenience
  (`encode_ply_with_format(…, PlyFormat::Ascii)`) — so `set-format` → `binaryLittleEndian` moved the
  typed `PlySnapshot` and then wrote plain ascii with a `format ascii 1.0` header, while the
  reference really wrote binary. The mutation was unobservable in our document and the whole point
  of the row — real binary list encoding on both sides — was never measured. The production pack
  path had already read the field the right way (`🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/
  📸️snapshot/🦀️component.rs` calls `encode_ply_with_format(self, self.format)` and names the
  ascii-forcing call as the hazard); the adapter now mirrors it. Second, the no-byte-pass-through
  tripwire was applied to the UNDO step as well as the forward one, so `inverse-no-mutation` failed
  for the codec behaving exactly as its own retention law requires: the undo re-encodes this codec's
  own first-generation output, and decode/encode is a documented fixed point from the second
  generation onward, so an undo that restores the same model MUST reproduce those bytes. The
  tripwire now guards only the step that reads the real committed fixture — a foreign writer's
  bytes, which this codec's normal form cannot reproduce — and the pristine-input half of the law
  still runs unweakened in `identity-round-trip` and in every forward step. No profile, fixture or
  assertion was changed.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🧪️pattern-sphere/🧊️.ply
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                | params                                                                                                                                                                                                                                                                                              |
      | set-snapshot       | {"snapshot":{"format":"ascii","comments":["synthetic replacement snapshot"],"elements":[{"name":"vertex","count":3,"properties":[{"name":"x","form":"scalar","kind":"float"},{"name":"y","form":"scalar","kind":"float"},{"name":"z","form":"scalar","kind":"float"}],"rows":[{"values":[0,0,0]},{"values":[1,0,0]},{"values":[0,1,0]}]},{"name":"face","count":1,"properties":[{"name":"vertex_indices","form":"list","countKind":"uChar","valueKind":"int"}],"rows":[{"values":[[0,1,2]]}]}]}} |
      | set-format         | {"format":"binaryLittleEndian"}                                                                                                                                                                                                                                                                    |
      | insert-comment     | {"index":0,"comment":"mutation-inserted comment"}                                                                                                                                                                                                                                                  |
      | remove-comment     | {"index":0}                                                                                                                                                                                                                                                                                         |
      | add-element        | {"index":3,"element":{"name":"material","count":1,"properties":[{"name":"diffuse_red","form":"scalar","kind":"uChar"}],"rows":[{"values":[200]}]}}                                                                                                                                               |
      | remove-element     | {"name":"edge"}                                                                                                                                                                                                                                                                                     |
      | insert-row         | {"elementName":"vertex","index":8449,"row":{"values":[0.5,0.5,0.5,0,0,1,0.5,0.5]}}                                                                                                                                                                                                                |
      | remove-row         | {"elementName":"vertex","index":8448}                                                                                                                                                                                                                                                              |
      | set-row-property   | {"elementName":"vertex","rowIndex":0,"propertyName":"x","value":42}                                                                                                                                                                                                                                |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real document
    Given the real input document shared://🧪️pattern-sphere/🧊️.ply
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the semantic projection

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://🧪️pattern-sphere/🧊️.ply
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the document matches its pre-mutation semantic projection
    Examples:
      | id                | params                                                                                                                                                                                                                                                                                              |
      | set-snapshot       | {"snapshot":{"format":"ascii","comments":["synthetic replacement snapshot"],"elements":[{"name":"vertex","count":3,"properties":[{"name":"x","form":"scalar","kind":"float"},{"name":"y","form":"scalar","kind":"float"},{"name":"z","form":"scalar","kind":"float"}],"rows":[{"values":[0,0,0]},{"values":[1,0,0]},{"values":[0,1,0]}]},{"name":"face","count":1,"properties":[{"name":"vertex_indices","form":"list","countKind":"uChar","valueKind":"int"}],"rows":[{"values":[[0,1,2]]}]}]}} |
      | set-format         | {"format":"binaryLittleEndian"}                                                                                                                                                                                                                                                                    |
      | insert-comment     | {"index":0,"comment":"mutation-inserted comment"}                                                                                                                                                                                                                                                  |
      | remove-comment     | {"index":0}                                                                                                                                                                                                                                                                                         |
      | add-element        | {"index":3,"element":{"name":"material","count":1,"properties":[{"name":"diffuse_red","form":"scalar","kind":"uChar"}],"rows":[{"values":[200]}]}}                                                                                                                                               |
      | remove-element     | {"name":"edge"}                                                                                                                                                                                                                                                                                     |
      | insert-row         | {"elementName":"vertex","index":8449,"row":{"values":[0.5,0.5,0.5,0,0,1,0.5,0.5]}}                                                                                                                                                                                                                |
      | remove-row         | {"elementName":"vertex","index":8448}                                                                                                                                                                                                                                                              |
      | set-row-property   | {"elementName":"vertex","rowIndex":0,"propertyName":"x","value":42}                                                                                                                                                                                                                                |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real document
    Given the real input document shared://🧪️pattern-sphere/🧊️.ply
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    And the mutation's own inverse is applied to the result
    Then the document matches its pre-mutation semantic projection

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🧪️pattern-sphere/🧊️.ply
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the oracle and the subject agree on the semantic projection
