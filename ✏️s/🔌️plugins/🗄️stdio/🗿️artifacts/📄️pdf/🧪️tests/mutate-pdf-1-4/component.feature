@capability-pdf-1-4-mutate
@oracle-lopdf-pdf-1-4-mutate
@comparison-semantic-pdf-v1
@mutations-pdf-1-4-any
Feature: Apply every typed PDF 1.4 mutation to a real-world document
  The input is the real ~6.3MB, 65-page bachelor-thesis PDF committed under this standard's own
  examples directory, not a synthetic fixture. Every scenario copies it into the case work
  directory before touching it; the committed document is never written to.

  This subset's own `PdfSnapshot` is `{schema, page: {width, height, text}}` — one page, no object
  graph — and its `decode_pdf` never reads a real page's width/height (hardcoded to 612/792 for
  every input; confirmed against this fixture's true `MediaBox [0 0 595.276 841.89]`). Both
  mutation kinds it declares (`NoMutation`, `SetSnapshot`) are real end to end regardless: applying
  either one, undoing it, and decoding/re-encoding the real document all genuinely exercise the
  subset's whole codec on genuinely large, genuinely real input. The oracle and subject projections
  below compare `width`/`height`/`text` — everything `PdfSnapshot` itself carries — through an
  independent `lopdf` reader on both sides, never against each other's own writing.

  THE LAWS THE ORACLE ASSERTS IN-ROLE, so a scenario cannot pass merely because `lopdf` did not
  error. `inverse-<kind>` applies the mutation, applies its algebraic inverse, and fails with the
  first diverging field unless the result projects onto exactly what the un-mutated document
  projects onto. `identity-round-trip` fails unless the re-encoded bytes differ from the input AND
  the independent reader recovers exactly the text the real input carries. Both laws are asserted on
  `text` and on the pinned `612×792` geometry, and NOT on this fixture's real page size: the
  subset's whole model hardcodes Letter-size on both sides (`decode_pdf` never reads a MediaBox),
  so `595.276 × 841.89` is unreachable by construction and demanding it would be a contrived check
  rather than a true one. That carve-out is a documented property of the subset, not a softened law.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id           | params                                                                                                     |
      | no-mutation  | {}                                                                                                         |
      | set-snapshot | {"snapshot": {"schema": "s.stdio.pdf", "page": {"width": 612, "height": 792, "text": "Wave seven replaced this page."}}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id           | params                                                                                                     |
      | no-mutation  | {}                                                                                                         |
      | set-snapshot | {"snapshot": {"schema": "s.stdio.pdf", "page": {"width": 612, "height": 792, "text": "Wave seven replaced this page."}}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the document is fully decoded into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
