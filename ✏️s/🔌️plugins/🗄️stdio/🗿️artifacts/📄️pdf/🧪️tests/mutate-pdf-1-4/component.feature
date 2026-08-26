@capability-pdf-1-4-mutate
@oracle-lopdf-pdf-1-4-mutate
@comparison-semantic-pdf-v1
@mutations-pdf-1-4-any
Feature: Apply every typed PDF 1.4 mutation to a real-world document
  The input is the real ~6.3MB, 65-page bachelor-thesis PDF committed under this standard's own
  examples directory, not a synthetic fixture. Every scenario copies it into the case work
  directory before touching it; the committed document is never written to.

  WHAT THIS SUBSET CARRIES. PDF 1.4 has a real page TREE — ISO 32000-1 §7.7.3's catalog pointing at
  a `/Pages` node whose `/Kids` recursively resolve to `/Page` leaves, each with its own inheritable
  `/MediaBox` and content stream — and this subset's `PdfSnapshot` is that tree walked flat:
  `pages: Vec<PageDoc>`, one `{width, height, text}` entry per leaf, in reading order. The two
  mutation kinds it declares are `NoMutation` and `SetSnapshot`, because this standard's document
  vocabulary is "replace the page tree" and nothing finer; the per-page conformance axes belong to
  the `✳️a` and `✳️x` subsets, which own their own vocabularies over the same snapshot.

  WHAT CHANGED, STATED PLAINLY BECAUSE IT WAS REPORTED GREEN FOR SIX WAVES. Until the first full
  differential run of ticket 26/08/23/END-TO-END-TESTING-REFACTOR the snapshot was
  `{schema, page: PageDoc}` — ONE page — and `decode_pdf` hardcoded `612×792` for every input
  instead of reading a real `/MediaBox`. Fed this 65-page thesis it produced a 607-byte one-page
  skeleton whose only text was `SemIO`: 64 pages destroyed on write. This case was written to mirror
  that stub — the oracle rebuilt every document as one synthetic 612×792 page, and every law was
  measured against that rebuild rather than against the committed bytes, precisely so the geometry
  gap could not fail anything. Both halves are gone. The reader walks the real page tree with real
  `/MediaBox` inheritance, the writer emits every page, and the laws below are measured against the
  REAL DOCUMENT's own projection.

  WHAT THE PROJECTION IS. The page count, and per page the `/MediaBox` extent and the shown text —
  the operand bytes of ISO 32000-1 §9.4.3's four text-showing operators (`Tj`, `TJ`, `'`, `"`) in
  content-stream order, lossily decoded. Exactly what `PdfSnapshot` itself carries, nothing more.
  Both sides are read back by the SAME independent `lopdf` reader — its own page-tree walk and
  content-stream decoder, never this repository's `decode_pdf` — so neither producer is ever checked
  against its own writing. The document VERSION is deliberately not projected: this snapshot does
  not retain it, and the committed file in fact declares `%PDF-1.5` while this standard's writer
  emits `%PDF-1.4`, so recording it would report a divergence about a field neither producer was
  asked to carry.

  ALL THREE LAWS ARE ASSERTED IN ROLE, through the shared ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law module,
  so no scenario can pass merely because `lopdf` declined to error. `mutate-<kind>` fails unless the
  mutation MOVES the compared projection. `inverse-<kind>` applies the mutation, applies its
  algebraic inverse — for `set-snapshot` a `set-snapshot` carrying the base document's own 65-page
  tree, read back out of the input by the independent reader — and fails with the first diverging
  field unless the result projects onto exactly what the un-mutated document projects onto.
  `identity-round-trip` fails unless the re-encoded bytes differ from the input AND the whole page
  tree survives. Nothing is exempt; there is no longer any field the laws step around.

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
      | id           | params                                                                                                                                                                                                            |
      | no-mutation  | {}                                                                                                                                                                                                                |
      | set-snapshot | {"snapshot": {"schema": "s.stdio.pdf", "pages": [{"width": 595.276, "height": 841.89, "text": "A replacement page tree, written from the model alone."}, {"width": 419.528, "height": 595.276, "text": "Its second page is A5."}]}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id           | params                                                                                                                                                                                                            |
      | no-mutation  | {}                                                                                                                                                                                                                |
      | set-snapshot | {"snapshot": {"schema": "s.stdio.pdf", "pages": [{"width": 595.276, "height": 841.89, "text": "A replacement page tree, written from the model alone."}, {"width": 419.528, "height": 595.276, "text": "Its second page is A5."}]}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the document is fully decoded into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
