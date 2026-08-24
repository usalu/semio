@capability-txt-utf-8-mutate
@no-oracle-txt-utf-8-line-structure
@comparison-exact-bytes-v1
@mutations-txt-utf-8-any
Feature: Apply every typed UTF-8 text-line mutation to a real document
  The input is shared://📄️interview-transkript.tex, a real 170-line, LF-terminated German
  interview transcript (♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/transkript-bunschoten.tex,
  copied here verbatim) — genuine LaTeX markup, genuine umlauts (ä/ö/ü/ß) and 80 real blank lines
  from real paragraph spacing, not synthetic prose. Its 170th line is empty: the file ends
  `…conversation.\n\n`, which matters below. Every scenario copies it into the case work
  directory before touching it; the committed file is never written to.

  There is no credible third-party crate that is authoritative over plain-text line structure —
  line splitting on LF/CRLF, one line-ending style per document and trailing-newline presence are
  exactly what THIS subset defines, not a fact an external library could confirm or refute. See the
  recorded no-oracle decision `txt-utf-8-line-structure`: the `csv` crate, already linked for the
  tabular subsets, genuinely cross-checks line boundaries on single-style, non-blank real content
  (its record reader silently drops blank lines — a real limitation found and documented in the
  oracle module's own tests, not hidden), but cannot referee LF vs CRLF vs trailing-newline at all.
  Confidence instead comes from specification vectors (below), the inverse law as a metamorphic
  property, and a hand-written reference implementation of the mutation semantics in this subset's
  own oracle module that never calls this subset's production `TxtSnapshot`/`TxtMutation` code.

  What the subset claims: exactly `Lf`/`CrLf`, one style for the whole document (CrLf iff at least
  one literal `\r\n` occurs anywhere), a trailing terminator tracked as a separate boolean whose
  only legal combinations are the ones the split can return (a terminated document has at least one
  line; an unterminated one's last line is never empty — see the 🔒️ note below), and UTF-8 content
  with NO normalization, NO BOM handling and NO NEL(U+0085)/LS(U+2028)/PS(U+2029) line-breaking — see the `@id-spec-vector` scenarios for exactly what that means byte-for-byte. A
  second real capture, shared://📓️hub-boot-log.txt (a genuine terminal log, mostly LF with two real
  embedded CRLF sequences from a subprocess's own convention), is exercised directly in the oracle
  module's own tests rather than here: it demonstrates the whole-document CrLf detection rule
  collapsing real mostly-LF content into very few split points, which is why it is not also used as
  this feature's main exhaustive-mutation fixture.



  A note on the `@id-identity-round-trip` scenario below, which in every OTHER case this wave
  asserts the re-encoded bytes are NOT bit-identical to the input (proof that real parsing, not a
  byte-copy shortcut, produced them): for `s.stdio.txt@utf-8/*` that assertion would be dishonest.
  This subset's native `Text` payload IS the raw carrier text verbatim (the carrier law proved by
  `carrier_native_is_raw` in `🚪️io/🦀️component.rs`), and splitting a string on a fixed separator
  then rejoining with that SAME separator is a mathematical identity regardless of what characters
  sit between the split points — confirmed independently in the oracle module's own
  `mixed_crlf_lf_is_still_a_lossless_round_trip` test, not merely assumed. So decode→encode
  reproducing the input exactly is the CORRECT outcome here, not evidence of smuggled bytes. What
  proves genuine parsing for this subset instead is the exhaustive `mutate-<kind>` scenarios below:
  you cannot insert, remove or replace line 100 of a 170-line real document without actually having
  parsed it into lines.

  ⚠️ TWO things about this case a reader must not take on trust. FIRST: the runner does not execute
  a `@no-oracle-` case's scenarios in the oracle phase at all — `[test] not-exercised … recorded
  no-oracle decision txt-utf-8-line-structure`. The SUBJECT phase does run (`bun ./📜️script.ts
  subject exhaustive --owner 🗄️stdio --case mutate-txt-utf-8` → `executed=24 passed=24`), and it is
  the only phase that ever will for this case, so it is the only thing standing between these 24
  scenarios and no evidence at all. That is exactly why every subject handler below asserts its law
  IN ROLE: until 2026-08-24 the `mutate-<kind>`, `inverse-<kind>` and `identity-round-trip` handlers
  computed a result and asserted NOTHING about it — 15 of the 24 scenarios were reporting green
  while proving only that the codec did not panic. `parity=0/0` is the correct and permanent reading
  for this case; it is not a number waiting to improve. The reference module's own `#[cfg(test)]`
  suite (`cargo test --features oracles --lib`) exercises the observability, inverse and carrier
  laws against THIS fixture with THESE exact Examples parameters, independently of the subject.

  🔒️ SECOND, and found by asserting the inverse law rather than describing it: `set-trailing-newline`
  did not invert on this document, and the defect was in this subset's own data model. The pair
  `(lines, trailingNewline)` is not an injective encoding of a body — `(["a"], true)` and
  `(["a", ""], false)` both render to `"a\n"`, as do `([], true)` and `([""], true)` — and the split
  resolves every such tie in favour of the terminator. This fixture's last line is empty, so
  `set-trailing-newline` to `false` rendered 170 lines with no terminator, bytes that read back as
  169 lines WITH one, and the inverse (correctly computed from the original as `true`) could no
  longer recover the lost blank line.

  The remedy landed in the VOCABULARY, on both sides at once, and it is a NARROWING rather than a
  wider snapshot type. `TxtSnapshot`'s reachable states are now exactly the canonical ones — the
  images the carrier's own split produces — because every `TxtMutation` is gated on that predicate
  and a mutation whose result would fall outside it is REFUSED with
  `stdio.txt.mutation-not-representable`, changing nothing, the same "refuse rather than silently
  lose" discipline the sibling `📰xml ✳️valid` vocabulary applies to §2.8. The oracle module states
  the same rule independently, from the join rule rather than from our code. On the canonical states
  that remain, `from_body`/`to_body` is a bijection: no reachable document loses a line to its own
  serialization.

  The cost is stated plainly instead of hidden. On THIS fixture — and on any text file that ends
  with a blank line — `set-trailing-newline false` now has no result at all, because the document it
  would name is already spelled with one line fewer and a terminator; reaching it is `remove-line`'s
  job. Its two `Examples` rows below therefore exercise a REFUSAL, and the oracle module asserts
  exactly that (`every_feature_row_moves_the_real_documents_projection` requires the refusal and
  requires it to name the loss; `every_feature_row_inverts_back_to_the_real_document` requires it to
  leave the bytes untouched, so the row cannot pass by quietly doing nothing). The kind's POSITIVE
  inverse is exercised on a document whose last line is not empty, in
  `set_trailing_newline_inverts_where_its_result_is_representable`, so no kind is left with an
  unexercised inverse. The fixture was NOT swapped for one that avoids the collision — that would
  have hidden a defect that was live in production code.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://📄️interview-transkript.tex
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"lines": ["Ersetztes Protokoll", "Zweite Zeile mit Umlaut: äöüß und Emoji 🎉"], "trailingNewline": true, "lineEnding": "lf"} |
      | set-trailing-newline | {"value": false} |
      | set-line-ending | {"value": "crLf"} |
      | insert-line | {"index": 20, "text": "Eingefügte Randnotiz zu Bauhütte 4.0"} |
      | remove-line | {"index": 100} |
      | set-line | {"index": 50, "text": "Ersetzte Zeile: Stakeholder-Interessen verbinden"} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://📄️interview-transkript.tex
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original document
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"lines": ["Ersetztes Protokoll", "Zweite Zeile mit Umlaut: äöüß und Emoji 🎉"], "trailingNewline": true, "lineEnding": "lf"} |
      | set-trailing-newline | {"value": false} |
      | set-line-ending | {"value": "crLf"} |
      | insert-line | {"index": 20, "text": "Eingefügte Randnotiz zu Bauhütte 4.0"} |
      | remove-line | {"index": 100} |
      | set-line | {"index": 50, "text": "Ersetzte Zeile: Stakeholder-Interessen verbinden"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://📄️interview-transkript.tex
    When the document is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are bit-identical to the input, which is the carrier law working correctly rather than a byte pass-through

  @id-spec-vector
  @level-exhaustive
  @mode-conformance
  Scenario Outline: <id> decodes and re-encodes byte-for-byte
    Given the literal UTF-8 text vector
      """
      <vector>
      """
    When the vector is decoded to the typed snapshot and re-encoded from it alone
    Then the re-encoded bytes are bit-identical to the vector
    Examples:
      | id | vector |
      | pure-lf | "a\nb\nc\n" |
      | pure-crlf | "a\r\nb\r\nc\r\n" |
      | lf-no-trailing-terminator | "a\nb\nc" |
      | mixed-crlf-and-bare-lf | "a\r\nb\nc\r\nd\n" |
      | bom-as-first-line-content | "﻿hello\nworld\n" |
      | astral-emoji-and-variation-selectors | "🎉\n📜️\n" |
      | combining-mark-distinct-from-precomposed | "é\né\n" |
      | nel-ls-ps-as-ordinary-content | "beforemiddle more end\n" |
      | empty-document | "" |
