@capability-semio-v1-document-mutate
@oracle-semio-document-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-document
Feature: Apply every typed semio DOCUMENT mutation to the real committed memo, against an independent Python implementation
  `s.stdio.semio.document` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`/`.pack.semio`, so the second producer a differential comparison needs is a second
  IMPLEMENTATION. `🐍️component.py` beside this file is that implementation - the envelope, the DSL
  grammar, the binary pack frame and all eighteen verbs together with their inverses, written in
  Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `.../📸️snapshot/💾️binary/📡️component.protocol.semio`,
  `.../🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`, the committed schema mirror
  `.../🧬️mutations/🟦️component.ts` for the three `DocBlockPath` segment tags, and the semio
  envelope in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing
  from and transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-document-python-independent` in `.../✳️document/🧪️oracle/🔣️component.json`; the recorded
  no-oracle decision it replaces is gone, because there is now a reference to compare against.

  The CommonMark route the replaced decision surveyed stays rejected and nothing here revives it:
  the oracle role may never link the subject crate, so handing `comrak` a document would mean
  routing the snapshot through THIS repository's own Markdown exporter first, and CommonMark has no
  named style table, no id-keyed embedded image store and no run-level character formatting that
  survives a parse-render cycle - ten of the eighteen kinds would have had nothing to compare
  against. A from-specification second implementation judges all eighteen.

  The document under test is the REAL committed memo, read where the domain keeps it through
  `asset://` and never written to: one named style that is itself `basedOn` another, one embedded
  PNG payload, and eight top-level blocks covering ALL eight `DocBlock` variants - a level-1
  heading whose run is bold, a plain paragraph, an ordered list, a one-cell table, a `rust` code
  block, a quote, an image block carrying both optional dimensions, and a page break. It is the
  richest `s.stdio.semio.document` document committed anywhere in this artifact; `asset://` resolves
  against the artifact root, so no other plugin's larger `.dsl.semio` is reachable from here, and
  that limit is stated rather than papered over.

  The `mutate-` and `inverse-` parameters are chosen against the memo's own shape, so a plausible
  wrong codec fails, and between them they exercise all three `DocBlockPath` segment kinds:
  `insert-block` reaches INSIDE the table's only cell, `set-block-content` replaces the list item's
  paragraph with a code block through a `listItem` segment, `set-run-text` reaches into the quote,
  `remove-block` deletes the MIDDLE code block so a tail-only implementation fails, `set-run-style`
  overwrites an all-default style with one carrying every optional member at once so an
  implementation honouring only the booleans fails, `set-image-block` sets `width` to ABSENT while
  setting `height` present so an implementation that cannot write `None` fails, `set-style-name`
  writes a non-ASCII name so a byte-length assumption fails, and `set-style-based-on` clears a
  present `basedOn` to absent.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each kind in this case's own `🧫️fixtures/`, now applied by
  BOTH implementations and checked against the committed after-snapshot by each of them in role.
  Nothing was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law on BOTH committed encodings.
  `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, and both files
  were produced by the Rust codecs, so an exact re-emission is the CORRECT answer here and the
  wave's must-differ tripwire would be backwards, which is why the Rust side asserts
  `law::carrier_is_exact` twice. What stops that being a codec agreeing with itself is that the
  Python side reproduces the same 610 text bytes and 271 binary bytes - the text from the grammar,
  including its rule that an `f64` leaf prints as the DECIMAL OF ITS BIT PATTERN rather than as a
  float literal, and the binary from the committed protocol plus a record layout derived from the
  committed bytes because the protocol document declares the three collections one opaque `payload`
  chain by its own admission - and the two sides' digests of what each emitted are compared. The two
  encodings also cross-check each other: the binary twin has to decode to the same memo the text
  does.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real committed memo
    Given the real committed memo asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio
    When the <id> mutation is applied to the memo parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id                  | mutation                                                                                                                                                                                                                                                                                              |
      | no-mutation         | {"mutation":"noMutation"}                                                                                                                                                                                                                                                                             |
      | set-snapshot        | {"mutation":"setSnapshot","snapshot":{"schema":"s.stdio.semio.document","styles":[],"images":[],"blocks":[{"kind":"pageBreak"}]}}                                                                                                                                                                     |
      | insert-block        | {"mutation":"insertBlock","path":{"segments":[{"kind":"tableCell","block_index":3,"row":0,"cell":0}],"index":0},"block":{"kind":"paragraph","style_id":null,"runs":[{"text":"header cell","style":{"bold":true,"italic":false,"underline":false,"size":null,"font":null,"color":null,"link":null}}]}} |
      | remove-block        | {"mutation":"removeBlock","path":{"segments":[],"index":4}}                                                                                                                                                                                                                                           |
      | set-block-content   | {"mutation":"setBlockContent","path":{"segments":[{"kind":"listItem","block_index":2,"item":0}],"index":0},"block":{"kind":"code","language":"python","text":"print(1)"}}                                                                                                                             |
      | set-paragraph-style | {"mutation":"setParagraphStyle","path":{"segments":[],"index":1},"style_id":"heading1"}                                                                                                                                                                                                               |
      | set-heading-level   | {"mutation":"setHeadingLevel","path":{"segments":[],"index":0},"level":3}                                                                                                                                                                                                                             |
      | set-list-ordered    | {"mutation":"setListOrdered","path":{"segments":[],"index":2},"ordered":false}                                                                                                                                                                                                                        |
      | set-run-text        | {"mutation":"setRunText","path":{"segments":[{"kind":"quote","block_index":5}],"index":0},"run_index":0,"text":"zitiert"}                                                                                                                                                                             |
      | set-run-style       | {"mutation":"setRunStyle","path":{"segments":[],"index":0},"run_index":0,"style":{"bold":false,"italic":true,"underline":true,"size":11.5,"font":"Inter","color":"#202020","link":"https://semio.tech"}}                                                                                              |
      | set-image-block     | {"mutation":"setImageBlock","path":{"segments":[],"index":6},"image_id":"img1","alt":"Grundriss","width":null,"height":240.5}                                                                                                                                                                         |
      | insert-style        | {"mutation":"insertStyle","style":{"id":"caption","name":"Caption","basedOn":"normal"}}                                                                                                                                                                                                               |
      | remove-style        | {"mutation":"removeStyle","id":"heading1"}                                                                                                                                                                                                                                                            |
      | set-style-name      | {"mutation":"setStyleName","id":"heading1","name":"Überschrift 1"}                                                                                                                                                                                                                                    |
      | set-style-based-on  | {"mutation":"setStyleBasedOn","id":"heading1","based_on":null}                                                                                                                                                                                                                                        |
      | insert-image        | {"mutation":"insertImage","image":{"id":"img2","mime":"image/jpeg","bytes":[255,216,255]}}                                                                                                                                                                                                            |
      | remove-image        | {"mutation":"removeImage","id":"img1"}                                                                                                                                                                                                                                                                |
      | set-image-bytes     | {"mutation":"setImageBytes","id":"img1","mime":"image/gif","bytes":[71,73,70]}                                                                                                                                                                                                                        |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real committed memo
    Given the real committed memo asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio
    When the <id> mutation is applied to the memo parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the memo and agree on the mutated and the restored snapshot
    Examples:
      | id                  | mutation                                                                                                                                                                                                                                                                                              |
      | no-mutation         | {"mutation":"noMutation"}                                                                                                                                                                                                                                                                             |
      | set-snapshot        | {"mutation":"setSnapshot","snapshot":{"schema":"s.stdio.semio.document","styles":[],"images":[],"blocks":[{"kind":"pageBreak"}]}}                                                                                                                                                                     |
      | insert-block        | {"mutation":"insertBlock","path":{"segments":[{"kind":"tableCell","block_index":3,"row":0,"cell":0}],"index":0},"block":{"kind":"paragraph","style_id":null,"runs":[{"text":"header cell","style":{"bold":true,"italic":false,"underline":false,"size":null,"font":null,"color":null,"link":null}}]}} |
      | remove-block        | {"mutation":"removeBlock","path":{"segments":[],"index":4}}                                                                                                                                                                                                                                           |
      | set-block-content   | {"mutation":"setBlockContent","path":{"segments":[{"kind":"listItem","block_index":2,"item":0}],"index":0},"block":{"kind":"code","language":"python","text":"print(1)"}}                                                                                                                             |
      | set-paragraph-style | {"mutation":"setParagraphStyle","path":{"segments":[],"index":1},"style_id":"heading1"}                                                                                                                                                                                                               |
      | set-heading-level   | {"mutation":"setHeadingLevel","path":{"segments":[],"index":0},"level":3}                                                                                                                                                                                                                             |
      | set-list-ordered    | {"mutation":"setListOrdered","path":{"segments":[],"index":2},"ordered":false}                                                                                                                                                                                                                        |
      | set-run-text        | {"mutation":"setRunText","path":{"segments":[{"kind":"quote","block_index":5}],"index":0},"run_index":0,"text":"zitiert"}                                                                                                                                                                             |
      | set-run-style       | {"mutation":"setRunStyle","path":{"segments":[],"index":0},"run_index":0,"style":{"bold":false,"italic":true,"underline":true,"size":11.5,"font":"Inter","color":"#202020","link":"https://semio.tech"}}                                                                                              |
      | set-image-block     | {"mutation":"setImageBlock","path":{"segments":[],"index":6},"image_id":"img1","alt":"Grundriss","width":null,"height":240.5}                                                                                                                                                                         |
      | insert-style        | {"mutation":"insertStyle","style":{"id":"caption","name":"Caption","basedOn":"normal"}}                                                                                                                                                                                                               |
      | remove-style        | {"mutation":"removeStyle","id":"heading1"}                                                                                                                                                                                                                                                            |
      | set-style-name      | {"mutation":"setStyleName","id":"heading1","name":"Überschrift 1"}                                                                                                                                                                                                                                    |
      | set-style-based-on  | {"mutation":"setStyleBasedOn","id":"heading1","based_on":null}                                                                                                                                                                                                                                        |
      | insert-image        | {"mutation":"insertImage","image":{"id":"img2","mime":"image/jpeg","bytes":[255,216,255]}}                                                                                                                                                                                                            |
      | remove-image        | {"mutation":"removeImage","id":"img1"}                                                                                                                                                                                                                                                                |
      | set-image-bytes     | {"mutation":"setImageBytes","id":"img1","mime":"image/gif","bytes":[71,73,70]}                                                                                                                                                                                                                        |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When both implementations apply the vector's mutation to its before-snapshot
    Then each reaches the committed after-snapshot and the two agree
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
  Scenario: Re-emit both committed encodings of the real memo from the parsed snapshot
    Given the real committed memo asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🎒️example.pack.semio
    And the committed specification vector local://🦠️no-mutation.json whose before-snapshot is that artifact decoded
    When each implementation parses the text artifact, prints it back, decodes the binary twin and re-encodes it
    Then both reproduce the two committed files byte for byte and agree on the memo and on the digests of what they emitted
