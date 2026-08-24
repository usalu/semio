@capability-pdf-1-4-a-mutate
@oracle-lopdf-pdf-1-4-a-mutate
@comparison-semantic-pdf-1-4-conformance-a-v1
@mutations-pdf-1-4-a
Feature: Apply every typed ISO 19005-1 (PDF/A-1) conformance mutation to a real document
  The input is the real, committed 6.3 MB bachelor thesis produced by MiKTeX pdfTeX 1.40.21 — 65
  pages, a classic cross-reference table, page 1 typeset at A4 (/MediaBox [0 0 595.276 841.89]) and
  carrying real extractable text. It is read where the domain already keeps it; every scenario copies
  it into the case work directory before touching it, and the committed document is never written to.

  WHY THIS VOCABULARY HAS FOUR KINDS AND NOT FIFTEEN. PDF 1.4's retained snapshot is a bare
  PageDoc { width, height, text } — no object graph — and this subset's own check_pdf_a_conformance says so in
  as many words: it raises exactly two diagnostics, stdio.pdf.a.text-empty and stdio.pdf.a.schema-gap-unverifiable, and the second fires
  unconditionally on every document to record that full conformance cannot be checked from this
  schema at all. A vocabulary derived honestly from that checker therefore has exactly ONE movable
  axis, page.text — whether the document has any extractable text at all, and the schema-gap axis is not movable by anything, because no mutation can give
  PDF 1.4's snapshot an object graph it does not have. Inventing the kinds the 1.7 subsets
  legitimately declare — encryption dictionaries, JavaScript and launch actions, output intents, font
  embedding, per-page trim boxes — would be fabricating a vocabulary for a schema that cannot observe
  a single one of them.

  AND WHY IT SHARES NO KIND WITH ITS SIBLING. 1.4/✳️x, whose checker reads page geometry and never looks at the text. Two subsets of one standard over one
  snapshot type, sharing not a single kind, because their checkers read different fields of it. That
  is what makes a mutation a property of one subset of one standard rather than of a format.

  THE REFERENCE. `lopdf` 0.44 both reads and writes that axis on the real document — it parses the
  complete object graph and writes a fresh file from that graph alone, never a patch of the input
  bytes — so it is a genuine second producer and every mutate scenario is @mode-differential. It is
  test-only; this repository's own PDF codec is hand-written and links nothing.

  NOTHING IS ARRANGED. Every scenario runs on the committed bytes exactly as they are: the real
  document already carries both a positive page box and real page-1 text, so both the "set" and the
  "clear/collapse" direction of the single axis have something genuine to move. No pre-state is
  fabricated anywhere in this case.

  THE LAWS THE ORACLE ASSERTS IN-ROLE, so a scenario cannot pass merely because `lopdf` did not
  error. mutate-<id> fails unless the mutation actually MOVED the conformance projection — and the
  dispatcher itself refuses a parameter set that would make the mutation a no-op, so a row whose
  parameters are indistinguishable from its sibling kind's is an error rather than a silent pass.
  inverse-<id> applies the mutation, applies its own independently computed inverse, and fails unless
  the result projects onto exactly what the document started from. identity-round-trip fails unless
  the re-serialized bytes differ from the input AND their projection is identical to the input's.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the conformance projection
    Examples:
      | id              | params                                                |
      | no-mutation     | {}                                                    |
      | set-snapshot    | {"conformance": "stamped"}                            |
      | set-page-text   | {"text": "An abstract a reader can actually extract"} |
      | clear-page-text | {}                                                    |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the conformance projection is the one the document started from
    Examples:
      | id              | params                                                |
      | no-mutation     | {}                                                    |
      | set-snapshot    | {"conformance": "stamped"}                            |
      | set-page-text   | {"text": "An abstract a reader can actually extract"} |
      | clear-page-text | {}                                                    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the document is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance projection
    And the re-encoded bytes are not bit-identical to the input
