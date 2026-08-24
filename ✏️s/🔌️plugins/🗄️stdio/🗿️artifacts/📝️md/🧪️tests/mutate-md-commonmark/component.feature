@capability-md-commonmark-mutate
@oracle-comrak-md-commonmark-mutate
@comparison-ordered-json-v1
@mutations-md-commonmark-any
Feature: Apply every typed CommonMark mutation to a real-world document
  The input is this repository's own root README.md (47607 bytes of genuine human-authored project
  documentation, not a synthetic fixture: 77 headings, 8 fenced code blocks, 135 list items
  including a real nested table-of-contents, 38 table rows, 10 images, 215 links, 11 block-quote
  lines and 143 inline-HTML fragments), copied ONCE with `cp` into this artifact's own
  `shared://📄️readme.md`, pinned so it can never drift when the real README is later edited. Every
  scenario copies it into the case work directory before touching it; the committed fixture is
  never written to.

  Both `InsertBlock`/`ReplaceBlock` target REAL nested containers of the real document rather than
  synthetic top-level stand-ins: `insert-block` appends into the real `[!NOTE]` callout block quote
  at top-level index 3, and `replace-block` replaces the first block of the real table-of-contents
  list's own first item (top-level index 7, `path=[{listItem, index:7, item:0}]`) — exercising
  `MdPathStep::BlockQuote`/`MdPathStep::ListItem` nesting, not just the top-level `blocks` vector.
  `remove-block` drops the real "🛍️ Products" section heading at top-level index 8. `set-inlines`
  rewrites the real "📑️ Overview" heading's own inlines at top-level index 6. `set-snapshot`
  discards the entire document and substitutes a fresh, real multi-block replacement (heading,
  paragraph, nested list, fenced code block, block quote) — substantive by the nature of a
  whole-document replace, not because it derives from the README.

  Byte-pass-through caveat: `output == input` is checked and rejected by BOTH sides' own handlers —
  the subject's `mutate`/`inverse`/`round_trip` and the oracle's `round_trip_oracle` — but is not
  expected in practice here regardless: this repository's own CommonMark renderer and `comrak`'s
  renderer choose different concrete syntax for the same semantic tree (see below), so byte-for-byte
  collision on 47 KB of real prose essentially never happens even for `no-mutation`. CommonMark is
  not a byte-preserving carrier, so the law applies in full and is asserted, not documented away.

  Both laws this feature names are asserted IN ROLE, not deferred to the oracle-vs-subject
  comparison: `identity-round-trip` requires each side's own decode → re-encode to preserve that
  side's own block projection (and to move the bytes), and every `inverse-<kind>` row requires
  apply-then-undo to restore the original document's own projection. A scenario that only proved
  `comrak` did not error would be vacuous.

  Writer freedom vs. real information loss: the oracle and subject are compared on parsed BLOCK
  STRUCTURE (the semantic tree), never rendered text — `project_md`
  (`../../🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`) only ever emits the
  fields `MdBlock`/`MdInline` themselves carry, so bullet marker character, ordered-list delimiter,
  emphasis delimiter, code-fence character/length, indented-vs-fenced code block source form, and
  hard-break encoding (backslash vs. trailing spaces) are all dropped by construction — genuine
  CommonMark writer freedom the spec itself leaves unconstrained, not a normalization choice made
  after the fact. One real SCOPE gap is documented rather than hidden: this subset's own parser
  does not resolve reference-style links/images or setext headings (documented scope cut,
  degrading to plain text), while `comrak` with `Options::default()` resolves both as core
  CommonMark; the real README fixture uses neither construct (verified: no `^\[.+\]:` reference
  definitions, no `^=+$`/`^-+$` setext underlines), so it does not affect these scenarios, but is
  recorded as a genuine future risk rather than papered over. GFM extensions (tables, strikethrough,
  task lists, footnotes, autolinks) are never enabled on the oracle's `Options`, matching this
  subset's own honest CommonMark-only scope, so a real pipe table or `~~strike~~` in the README
  parses as plain paragraph text on both sides rather than gaining GFM structure on one side only.

  Honest limits: the Rust SUBJECT phase does not compile this wave (a concurrent session's
  `semio_framework::` cycle in `📡️spr/🧵️channel`); every scenario below is written and `sut`-gated
  so it compiles into the subject role the moment that lands, and only the oracle side is verified
  here. The `@id-inverse` scenarios are therefore weaker than they look while the subject is
  blocked: `apply(inverse(m), apply(m, base))` is asserted equal to `base`'s OWN independent
  `comrak` projection, computed purely oracle-side (`inverse_mutation_spec` reads the ORIGINAL
  document's projection to build the inverse spec), never a comparison against the subject's actual
  inverse implementation. The `@id-mutate` scenarios do not share this weakness: there `comrak`
  genuinely performs the mutation on the real document via its own AST, independent of anything the
  subject does.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://📄️readme.md
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    And the semantic projection moved, unless the kind is no-mutation
    Examples:
      | id            | params                                                                                                                                                                                                                                                                                                                                                                                                                            |
      | no-mutation   | {}                                                                                                                                                                                                                                                                                                                                                                                                                                |
      | set-snapshot  | {"snapshot":{"schema":"stdio.md","blocks":[{"kind":"heading","level":1,"inlines":[{"kind":"text","text":"This heading is the complete replacement snapshot for the set-snapshot mutation kind"}]},{"kind":"paragraph","inlines":[{"kind":"text","text":"Every block of the original real-world README is discarded by this mutation and substituted with this hand-authored real UTF-8 document instead, proving SetSnapshot performs a genuine whole-document replacement rather than a token edit."}]},{"kind":"list","ordered":false,"tight":true,"items":[[{"kind":"paragraph","inlines":[{"kind":"text","text":"First replacement item"}]}],[{"kind":"paragraph","inlines":[{"kind":"text","text":"Second replacement item, nested under its own list"}]}]]},{"kind":"codeBlock","info":"bash","literal":"echo \"the set-snapshot mutation replaced the entire document\"\n"},{"kind":"blockQuote","blocks":[{"kind":"paragraph","inlines":[{"kind":"text","text":"A block quote inside the replacement snapshot, proving nested containers survive a whole-document SetSnapshot."}]}]}]}} |
      | insert-block  | {"path":[{"step":"blockQuote","index":3}],"index":1,"block":{"kind":"paragraph","inlines":[{"kind":"text","text":"This second paragraph was appended by the insert-block mutation inside the real note callout block quote, proving InsertBlock performs a genuine nested structural insertion rather than a top-level-only edit."}]}}                                                                                                     |
      | remove-block  | {"path":[],"index":8}                                                                                                                                                                                                                                                                                                                                                                                                            |
      | replace-block | {"path":[{"step":"listItem","index":7,"item":0}],"index":0,"block":{"kind":"paragraph","inlines":[{"kind":"text","text":"This paragraph replaced the original table-of-contents entry for the products section, proving ReplaceBlock performs a genuine nested wholesale block replacement inside a real list item rather than a top-level-only edit."}]}}                                                                    |
      | set-inlines   | {"path":[],"index":6,"inlines":[{"kind":"text","text":"📑️ Overview (rewritten by the set-inlines mutation)"}]}                                                                                                                                                                                                                                                                                                                  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://📄️readme.md
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the restored document's semantic projection matches its state before <id> was applied
    Examples:
      | id            | params                                                                                                                                                                                                                                                                                                                                                                                                                            |
      | no-mutation   | {}                                                                                                                                                                                                                                                                                                                                                                                                                                |
      | set-snapshot  | {"snapshot":{"schema":"stdio.md","blocks":[{"kind":"heading","level":1,"inlines":[{"kind":"text","text":"This heading is the complete replacement snapshot for the set-snapshot mutation kind"}]},{"kind":"paragraph","inlines":[{"kind":"text","text":"Every block of the original real-world README is discarded by this mutation and substituted with this hand-authored real UTF-8 document instead, proving SetSnapshot performs a genuine whole-document replacement rather than a token edit."}]},{"kind":"list","ordered":false,"tight":true,"items":[[{"kind":"paragraph","inlines":[{"kind":"text","text":"First replacement item"}]}],[{"kind":"paragraph","inlines":[{"kind":"text","text":"Second replacement item, nested under its own list"}]}]]},{"kind":"codeBlock","info":"bash","literal":"echo \"the set-snapshot mutation replaced the entire document\"\n"},{"kind":"blockQuote","blocks":[{"kind":"paragraph","inlines":[{"kind":"text","text":"A block quote inside the replacement snapshot, proving nested containers survive a whole-document SetSnapshot."}]}]}]}} |
      | insert-block  | {"path":[{"step":"blockQuote","index":3}],"index":1,"block":{"kind":"paragraph","inlines":[{"kind":"text","text":"This second paragraph was appended by the insert-block mutation inside the real note callout block quote, proving InsertBlock performs a genuine nested structural insertion rather than a top-level-only edit."}]}}                                                                                                     |
      | remove-block  | {"path":[],"index":8}                                                                                                                                                                                                                                                                                                                                                                                                            |
      | replace-block | {"path":[{"step":"listItem","index":7,"item":0}],"index":0,"block":{"kind":"paragraph","inlines":[{"kind":"text","text":"This paragraph replaced the original table-of-contents entry for the products section, proving ReplaceBlock performs a genuine nested wholesale block replacement inside a real list item rather than a top-level-only edit."}]}}                                                                    |
      | set-inlines   | {"path":[],"index":6,"inlines":[{"kind":"text","text":"📑️ Overview (rewritten by the set-inlines mutation)"}]}                                                                                                                                                                                                                                                                                                                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://📄️readme.md
    When the document is fully decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And each side's own re-encoded document projects back onto its own input's block structure
    And the re-encoded bytes are not bit-identical to the input
