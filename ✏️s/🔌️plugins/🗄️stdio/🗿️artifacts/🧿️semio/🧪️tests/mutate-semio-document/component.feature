@capability-semio-v1-document-mutate
@no-oracle-semio-document-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-document
Feature: Apply every typed semio DOCUMENT mutation to the real committed memo artifact
  `s.stdio.semio.document` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an oracle. That
  is recorded as the `semio-document-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️document/🧪️oracle/🔣️component.json`, which also records why
  `comrak` 0.54 — already linked into this owner's oracle crate, reads AND writes CommonMark, and
  reachable through this subset's own Markdown export serializer — was surveyed and rejected rather
  than merely absent: the oracle role may never link the subject crate, and CommonMark has no named
  style table, no id-keyed image store and no run-level formatting that survives a parse-render
  cycle, which strands ten of these eighteen kinds with nothing to compare against.

  The input is not synthetic. Every one of the eighteen kinds is applied to the snapshot this
  standard's own committed real artifact decodes to: one named style (`heading1`, based on
  `normal`), one embedded PNG image (`img1`), and an eight-block body that covers every single
  `DocBlock` variant — a bold-run heading, a plain paragraph, an ordered list, a one-cell table, a
  fenced Rust code block, a blockquote, a sized image reference and a page break. Each kind's
  committed `(before, mutation, after)` specification vector lives in this case's own `🧫️fixtures/`
  and was derived by an INDEPENDENT Python implementation of both the committed DSL grammar and this
  vocabulary's specification, never by running this repository's own Rust. Both roles read the same
  committed bytes: the `oracle` role reads the vector literally (no recomputation, no
  reimplementation of mutation semantics) and the `subject` role decodes it into real
  `SemioDocumentSnapshot`/`SemioDocumentMutation` values and runs the production entry point
  `apply_semio_document_mutation`. The `ordered-json-v1` profile compares the two structurally.

  What genuinely distinguishes this vocabulary from its siblings is `DocBlockPath` — a block is
  addressed by a segment chain that descends through `Quote`, list-item and table-cell containers
  before an index picks a slot in the innermost `Vec<DocBlock>`. A case that only ever used
  `DocBlockPath::top(n)` would leave that whole mechanism unexercised, so two kinds deliberately go
  nested: `insert-block` appends a second paragraph INSIDE the ordered list's first item, and
  `set-run-text` rewrites the run inside the blockquote's own paragraph.

  The `identity-round-trip` scenario is what keeps the vectors honest, and it is the only scenario
  here that touches raw artifact bytes. It asserts that production's OWN `parse_dsl` of the same
  real artifact equals the `before` snapshot every vector starts from, so a mistake in the
  independent Python decoder surfaces as a red scenario instead of a quietly agreeable one. It also
  crosses the two committed encodings of that one memo against each other — the text
  `🗣️example.dsl.semio` and the binary `🎒️example.pack.semio` are separate committed files produced by
  two separate codecs, so agreeing on one snapshot cannot be achieved by smuggling bytes from either
  one. This is the scenario that caught the `Q[{}.await]` defect described below. Note that unlike a
  foreign-writer format, byte-identical re-emission IS the expected result here: the committed text
  is this codec's own output, so the wave's usual "output must not equal input" tripwire does not
  apply and the pack/DSL cross-check carries that evidence instead.

  Writing this case found a real, isolated defect in the subset's own text codec, fixed under this
  ticket: `enc_block`'s `Quote` arm in `../../🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🔺️diff/
  🦀️component.rs` emitted `Q[{}.await]` — a stray `.await` literal left inside a `format!` string by
  an automated async sweep — while `dec_block`'s `"Q"` arm expects `Q[<blocks>]`. Any document
  carrying a blockquote therefore could not round-trip through its own DSL text codec, and the
  committed real memo carries exactly one. The committed artifact predates the defect, which is why
  it still holds the correct form.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the decoded real memo snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_document_mutation
    Then the resulting snapshot matches the vector's after-snapshot
    Examples:
      | id                  |
      | no-mutation         |
      | set-snapshot        |
      | insert-block        |
      | remove-block        |
      | set-block-content   |
      | set-paragraph-style |
      | set-heading-level   |
      | set-list-ordered    |
      | set-run-text        |
      | set-run-style       |
      | set-image-block     |
      | insert-style        |
      | remove-style        |
      | set-style-name      |
      | set-style-based-on  |
      | insert-image        |
      | remove-image        |
      | set-image-bytes     |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the decoded real memo snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_document_mutation
    And the mutation's own computed inverse is applied through apply_semio_document_mutation
    Then the snapshot matches the vector's before-snapshot again
    Examples:
      | id                  |
      | no-mutation         |
      | set-snapshot        |
      | insert-block        |
      | remove-block        |
      | set-block-content   |
      | set-paragraph-style |
      | set-heading-level   |
      | set-list-ordered    |
      | set-run-text        |
      | set-run-style       |
      | set-image-block     |
      | insert-style        |
      | remove-style        |
      | set-style-name      |
      | set-style-based-on  |
      | insert-image        |
      | remove-image        |
      | set-image-bytes     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real memo artifact through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🎒️example.pack.semio
    And the committed specification vector local://🦠️no-mutation.json whose before-snapshot is that artifact decoded
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees with the committed before-snapshot
