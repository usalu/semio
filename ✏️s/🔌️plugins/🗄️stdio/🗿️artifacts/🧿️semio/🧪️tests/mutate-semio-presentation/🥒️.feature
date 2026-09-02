@capability-semio-v1-presentation-mutate
@oracle-semio-presentation-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-presentation
Feature: Apply every typed semio PRESENTATION mutation to a real conference deck, against an independent Python implementation
  `s.stdio.semio.presentation` is a semio-NATIVE format: no third party reads or writes
  `.dsl.semio`/`.pack.semio`, and `python-pptx` — the obvious candidate — was surveyed and rejected,
  because it cannot create masters or layouts at all and because reaching a
  `SemioPresentationSnapshot` from pptx bytes through OUR importer would compare this repository with
  itself. The second producer THE STANDARD requires is therefore a second IMPLEMENTATION.
  `🐍️component.py` beside this file is that implementation — the carrier, the DSL grammar, the pack
  frame, document's own recursive `DocBlock` union and all fifteen verbs — written in Python from the
  committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio` and its Kaitai mirror,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`, and the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`). It imports nothing from and
  transliterates nothing of the Rust it judges, and it was pinned before use: it reproduces the
  committed `📽️deck` example artifact byte for byte in BOTH encodings and reaches all fifteen
  committed after-snapshots. It is registered as the oracle `semio-presentation-python-independent`;
  the recorded no-oracle decision it replaces is gone, because a reference now exists.

  **The deck under test is a real one, and its provenance is written down.**
  `local://🧪️talk/🗣️.dsl.semio` and its binary twin were derived ONCE from the real committed PowerPoint
  deck `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx` — a genuine 2020
  conference talk: one slide master, ELEVEN slide layouts, SEVEN slides, ninety-eight shapes, three
  embedded PNG parts and German text throughout, with real EMU geometry and real run styling. The
  reader that produced it is an independent Python OOXML reader built on `zipfile` and `xml.etree`,
  never this repository's own pptx bridge — using that bridge is precisely what the old no-oracle
  decision refused, and it stays out of the fixture's provenance for the same reason. `p:sp` carrying
  text becomes a `TextBox` so no content is dropped, a bare `p:ph` becomes a `Placeholder`, `p:pic`
  carries the media part's real bytes, and pptx geometry inheritance (slide → layout → master, by
  placeholder `idx` then `type`) is resolved rather than zeroed. That is 183 293 bytes of DSL and
  97 849 of pack, against the committed `📽️deck` example's 826 and 516; `asset://` cannot leave this
  artifact's root, which is why the derived deck is committed here as a case fixture rather than
  borrowed in place. The derivation script and its provenance note live in the ticket folder.

  The parameters are chosen against the deck's own shape, so a plausible wrong codec fails:
  `insert-slide` puts a new slide in the MIDDLE and that slide's single text box carries a heading, an
  ordered list, a quote, a code block, an image block and a page break — six of document's eight
  block kinds the real deck itself never uses, so the block grammar is exercised end to end;
  `remove-slide` drops a middle slide, so its POSITION and not merely its presence is what the
  inverse has to restore; `insert-shape` adds a real two-by-two table beside the title of slide 1;
  `set-textbox-blocks` rewrites the title with two differently styled runs, so a rewrite that drops
  run styling fails; `set-shape-frame` moves a shape whose frame was INHERITED from its layout; and
  `set-slide-layout` clears the layout reference entirely, exercising both arms of `option-hex`.

  Two parameters are deliberately conservative, and this is why. Masters and layouts are ID-KEYED and
  `insert-master`/`insert-layout` APPEND, so undoing the removal of a NON-terminal one restores it at
  the wrong position — a documented property of this vocabulary since wave 7, not a codec defect.
  `remove-layout` therefore targets the trailing layout and `remove-master` the deck's only master.
  That was checked rather than assumed: against this same real deck, removing `slideLayout1` really
  does fail the inverse law and removing the trailing layout really does restore it.

  🔴 **`mutate-set-snapshot` and `inverse-set-snapshot` are RED, and they are left red.** The
  `set-snapshot` payload replaces the deck with one whose slides are the same seven in REVERSE order.
  The independent implementation returns the reversed deck. The subject returns the reversed layouts,
  shapes and notes but the ORIGINAL seven slide ids, still at their original indices — so slide 0 ends
  up carrying slide 23's content under slide 1's identifier. The cause is in the production diff
  facet, not in either adapter: `SlideDiff`
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🔺️diff/🦀️component.rs`) declares
  `layout_id`, `shapes` and `notes` and NO `id`, and `set-snapshot`'s semantics are
  `SemioPresentationDiff::between`, so an index-keyed slide diff has no slot in which to carry a new
  identifier. The committed specification vector cannot see this — its replacement snapshot reuses
  the same single slide id — which is exactly why the real seven-slide deck was worth deriving. The
  scenario is NOT tuned away: no `ignoreKeys`, no relaxed profile and no substituted payload, because
  a whole-document replacement that keeps the old identity strings while taking the new content is a
  defect and not a convention. `spec-vector-set-snapshot` stays green, which localises the failure to
  the reordering case.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each of the fifteen kinds, whose before-state is the
  committed `📽️deck` example artifact, applied now by BOTH implementations and checked against the
  committed after-snapshot by each of them in role. Nothing was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law, for BOTH decks. `.dsl.semio` is a
  fixed-layout record grammar and `.pack.semio` is its binary twin, so reproducing all four committed
  files byte for byte is the CORRECT answer here and the wave's must-differ tripwire would be
  backwards — which is why the Rust side asserts `law::carrier_is_exact`. What stops that being a
  codec agreeing with itself is that the talk deck's two files were WRITTEN by the Python
  implementation from the grammar alone, in another language, and the two sides' digests of the
  re-emitted bytes are compared.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived talk deck
    Given the real derived presentation artifact local://🧪️talk/🗣️.dsl.semio
    And the committed mutation payload local://🧫️<id>/🦠️mutation/🔣️.json
    When the <id> mutation is applied to the deck parsed from it
    Then the independent implementation and the subject agree on the resulting deck
    Examples:
      | id                 |
      | no-mutation        |
      | set-snapshot       |
      | insert-slide       |
      | remove-slide       |
      | set-slide-layout   |
      | set-slide-notes    |
      | insert-shape       |
      | remove-shape       |
      | set-shape-frame    |
      | set-textbox-blocks |
      | insert-master      |
      | remove-master      |
      | insert-layout      |
      | remove-layout      |
      | set-layout-master  |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real derived talk deck
    Given the real derived presentation artifact local://🧪️talk/🗣️.dsl.semio
    And the committed mutation payload local://🧫️<id>/🦠️mutation/🔣️.json
    When the <id> mutation is applied to the deck parsed from it and each side undoes it with its own computed inverse
    Then both sides restore the deck and agree on the mutated and the restored snapshot, slide and shape order included
    Examples:
      | id                 |
      | no-mutation        |
      | set-snapshot       |
      | insert-slide       |
      | remove-slide       |
      | set-slide-layout   |
      | set-slide-notes    |
      | insert-shape       |
      | remove-shape       |
      | set-shape-frame    |
      | set-textbox-blocks |
      | insert-master      |
      | remove-master      |
      | insert-layout      |
      | remove-layout      |
      | set-layout-master  |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector over the committed deck artifact
    Given the committed specification fixtures for the <id> kind
      | role     | fixture                     |
      | before   | local://<id>/⬅️before/🔣️.json   |
      | mutation | local://<id>/🦠️mutation/🔣️.json |
      | after    | local://<id>/➡️after/🔣️.json    |
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                 |
      | no-mutation        |
      | set-snapshot       |
      | insert-slide       |
      | remove-slide       |
      | set-slide-layout   |
      | set-slide-notes    |
      | insert-shape       |
      | remove-shape       |
      | set-shape-frame    |
      | set-textbox-blocks |
      | insert-master      |
      | remove-master      |
      | insert-layout      |
      | remove-layout      |
      | set-layout-master  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both committed encodings of both decks from the parsed documents
    Given the real derived presentation artifact local://🧪️talk/🗣️.dsl.semio
    And its committed binary twin local://🎒️.pack.semio
    And the committed deck example asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🎒️.pack.semio
    When each implementation parses both text artifacts, prints them back, decodes both binary twins and re-encodes them
    Then both reproduce all four committed files byte for byte and agree on the two decks and on the digests of what they emitted
