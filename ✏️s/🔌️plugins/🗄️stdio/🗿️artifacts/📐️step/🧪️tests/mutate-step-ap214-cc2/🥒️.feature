@capability-step-ap214-cc2-mutate
@oracle-ruststep-step-ap214-cc2-mutate
@comparison-semantic-step-conformance-class-v1
@mutations-step-ap214-cc2
Feature: Apply every typed ISO 10303-214 CC2 (bounded wireframe/basic surfaces) mutation to a real AP214 exchange structure
  The input is `shared://🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp`, the same real committed
  export the `✳️any` case reads: a real Rhino 8.31 / ST-Developer v19.2 BIM export whose 1,396-entity
  DATA section is byte-for-byte untouched real data, with one edit recorded in its own header comment
  — the `FILE_SCHEMA` line, changed from the source's real AP242 declaration to `AUTOMOTIVE_DESIGN`,
  because no git-tracked native AP214 file exists anywhere in this repository. Every scenario copies
  it into the case work directory first; the committed fixture is never written to.

  🎯️ What makes this case NOT a copy of the `✳️any` case that reads the same bytes. `✳️any` declares
  the ISO 10303-21 GRAMMAR — insert an entity, set an argument, remove an argument — eleven verbs
  that would read identically for any Part-21 file on earth. A conformance class is not a grammar,
  it is a FILTER, and the 6 kinds here are one per axis `check_cc2_conformance` actually reads:
  the `AUTOMOTIVE_DESIGN` declaration, the `*_SHAPE_REPRESENTATION` ladder, and the
  PRODUCT/formation/definition identity chain. The projection reports those three axes and nothing
  else — a projection carrying the whole entity graph would drown every class-level difference in
  1,396 entities of unrelated geometry.

  🪜️ What this class is. CC2 is the first conformance class that admits geometry, and the only one
  that admits a single kind of it: its ceiling is `GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION`
  — curves and points carried directly by the representation, with no topology and no surfaces. In
  the real export that means `#836` (the bare `SHAPE_REPRESENTATION`, rung 2) already sits exactly on
  the ceiling while `#13` (a real `ADVANCED_BREP_SHAPE_REPRESENTATION`, rung 6) is four rungs above
  it. One demotion is therefore the whole repair, and it is the deepest demotion the ladder can
  perform.

  🏭️ A real-export finding this case is built on. `has_product_definition_chain` used to match the
  three chain types by EXACT name, and the real file carries
  `#822=PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE` — the ISO 10303-41 SUBTYPE every real
  AP214/AP242 exporter writes. Every `✳️ccN` analyzer therefore reported the soft
  `product-definition-chain` diagnostic against a file that genuinely carries the chain. The ladder
  half of that module already classified `*_SHAPE_REPRESENTATION` subtypes; the product half did not,
  and only a real export made it visible. Both halves now enumerate their ISO 10303-41 subtypes
  explicitly, because a name PREFIX is not an EXPRESS subtype relation.

  🧬️ Why this catalog is identical to `mutate-step-ap214-cc3`, `mutate-step-ap214-cc4`, `mutate-step-ap214-cc5`'s, and
  why that is a consequence rather than a copy. ISO 10303-214 §4.3 defines the six conformance
  classes as one monotone ladder over a single capability — which `*_SHAPE_REPRESENTATION` types
  the exchange structure may carry — and varies nothing else: the `AUTOMOTIVE_DESIGN` declaration
  and the PRODUCT/formation/definition chain are required identically by all six. A vocabulary
  derived one kind per axis therefore depends only on WHERE the ceiling sits, and there are three
  places. Strictly inside the ladder (CC2, CC3, CC4, CC5) there is a ceiling type to write and at
  least one rung above it to demote from, so both ladder verbs exist and the list is these six.
  Below the ladder (CC1) no representation is admitted at all, so deletion replaces both verbs.
  On top of it (CC6) nothing can be above the ceiling, so no demotion has a subject. The two edge
  classes declare five kinds each. This is asserted, not merely written down: the shared
  `🏅️standards/🔖️ap214/🧪️oracle/🦀️component.rs` carries
  `the_four_interior_classes_share_one_vocabulary_because_their_ceilings_share_one_place`, and the
  four classes share every line of classification and serialization through that one module —
  what this subset supplies is `MAX_RUNG = 2` and the ceiling type GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION
  it selects.

  🔬️ The reference. `ruststep` 0.4 is a real ISO 10303-21 reader (it parses this fixture's 1,396
  entities with zero errors) and has NO writer at all — no `Display`/`fmt::Formatter` impl exists on
  `Exchange`/`DataSection`/`Record`/`Parameter`, and `ast::ser::to_record` only builds an in-memory
  `Record` from an already-typed struct. Per the fleet brief's §6 that rules out
  `@mode-differential`: it cannot be a second PRODUCER. It is registered as the INDEPENDENT READER,
  and the re-serializer is this standard's own from-scratch Part-21 writer, shared by all seven
  `ap214` subsets and deliberately independent of this repository's production `engine::part21`. The
  ladder classification the oracle applies is likewise re-derived from ISO 10303-214 §4.3 rather than
  called out of the production `engine::ladder` — an oracle that asked the code under test how to
  classify would be comparing an implementation with itself.

  📌️ Every row below was chosen against the file's REAL content: `#13` is the real
  `ADVANCED_BREP_SHAPE_REPRESENTATION` named `brep_rep_0`, `#836` is the real `SHAPE_REPRESENTATION`
  named `Document` carrying items `(#837,#895)` in context `#835`, and `#827`/`#822`/`#821` are the
  real product chain. The adapter FAILS any row other than `no-mutation` whose projection did not
  move: a row whose parameters make the mutation a no-op is not a test.

  @id-mutate
  @level-exhaustive
  @mode-property
  Scenario Outline: Apply <id> and read the conformance projection back
    Given the real input document shared://🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the independently read conformance projection shows the mutation's real effect, asserted in role
    Examples:
      | id                          | params                                                                                                                                                                        |
      | no-mutation                 | {}                                                                                                                                                                            |
      | set-snapshot                | {"fileSchema": ["AUTOMOTIVE_DESIGN"], "productIdentity": {"product": 1, "productName": "Document", "formation": 2, "formationId": "A", "definition": 3, "definitionId": "A"}} |
      | set-file-schema             | {"schemas": ["CONFIG_CONTROL_DESIGN"]}                                                                                                                                        |
      | set-product-identity        | {"identity": null}                                                                                                                                                            |
      | set-shape-representation    | {"id": 836, "representation": {"typeName": "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION", "name": "Document", "items": [837, 895], "context": 835}}                  |
      | demote-shape-representation | {"id": 13}                                                                                                                                                                    |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real exchange structure
    Given the real input document shared://🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse computed against the untouched original is applied to that result
    Then the restored document's conformance projection equals the original's, asserted in role
    Examples:
      | id                          | params                                                                                                                                                                        |
      | no-mutation                 | {}                                                                                                                                                                            |
      | set-snapshot                | {"fileSchema": ["AUTOMOTIVE_DESIGN"], "productIdentity": {"product": 1, "productName": "Document", "formation": 2, "formationId": "A", "definition": 3, "definitionId": "A"}} |
      | set-file-schema             | {"schemas": ["CONFIG_CONTROL_DESIGN"]}                                                                                                                                        |
      | set-product-identity        | {"identity": null}                                                                                                                                                            |
      | set-shape-representation    | {"id": 836, "representation": {"typeName": "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION", "name": "Document", "items": [837, 895], "context": 835}}                  |
      | demote-shape-representation | {"id": 13}                                                                                                                                                                    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real exchange structure without passing bytes through
    Given the real input document shared://🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input, asserted in role
    And the independently read conformance projections of input and output agree, asserted in role
