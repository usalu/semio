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

  ALL THREE LAWS ARE ASSERTED IN ROLE, through the shared ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law module,
  so no scenario can pass merely because `lopdf` declined to error. `mutate-<kind>` fails unless the
  mutation MOVES the compared projection — with two declared kinds that is a small claim, but it is
  the difference between measuring `set-snapshot` and merely running it, and until this wave neither
  row was measured at all. `inverse-<kind>` applies the mutation, applies its algebraic inverse, and
  fails with the first diverging field unless the result projects onto exactly what the un-mutated
  document projects onto. `identity-round-trip` fails unless the re-encoded bytes differ from the
  input AND the independent reader recovers exactly the text the real input carries.

  WHAT THE LAWS ARE MEASURED AGAINST, AND WHY IT IS THE REBUILD RATHER THAN THE COMMITTED BYTES. The
  oracle is a rebuild-from-text writer that pins `MediaBox [0 0 612 792]` for every document,
  mirroring `decode_pdf`, which hardcodes the same constant and never reads a real page's geometry
  (confirmed against this fixture's true `[0 0 595.276 841.89]`). Measured against the committed
  input, `set-snapshot` would be credited with a `595.276 → 612` move the REBUILD made and the
  mutation did not — a green for something never observed. Measured against the reference's own
  `no-mutation` output, the only field that can move is `text`, the one this subset genuinely reads
  out of a document, and it has to. Geometry therefore carries no round-trip information on either
  side; that is a documented property of the subset, not a softened law, and demanding the real page
  size would be a contrived check rather than a true one. Both laws are proven again at unit level
  against the same real document by
  `every_declared_kind_is_observable_and_its_inverse_restores_the_document` in
  ../../🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs.

  WHAT THE DIFFERENTIAL RUN ACTUALLY REPORTS. The oracle-against-subject comparison ran for the
  first time in ticket 26/08/23/END-TO-END-TESTING-REFACTOR and scored parity 5 of 5: both mutate
  rows, both inverse rows and the identity round trip agree on the whole projection, with nothing
  ignored beyond `semantic-pdf-v1`'s own declared writer freedom. The sibling `mutate-pdf-1-7` case
  scored 24 of 37 on its first run over the very same document, and the ten-failure cluster there
  was a defect in the 1.7 writer's handling of a retained COS graph — a graph this subset's own
  `PdfSnapshot` does not carry, which is why the same run came back clean here. Its subject-side
  handlers do NOT yet assert the three laws in role the way the oracle-side ones do; parity carries
  those two rows today, and closing that gap is open work rather than a claim already made.

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
