@capability-semio-v1-text-mutate
@oracle-semio-text-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-text
Feature: Apply every typed semio TEXT mutation to a real published article, against an independent Python implementation
  `s.stdio.semio.text` is a semio-NATIVE format: no third party reads or writes `.dsl.semio` or
  `.pack.semio`, so the second producer a differential comparison needs is a second IMPLEMENTATION.
  `🐍️component.py` beside this file is that implementation — the carrier, the DSL grammar, the pack
  frame and all seven verbs, written in Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`
  and the semio envelope in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing
  nothing from and transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-text-python-independent` in `…/✳️text/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  📰️ **The document under test is a real published article.** The richest `s.stdio.semio.text`
  document committed anywhere in this artifact is the three-run demo note — 203 bytes, which is a
  fixture, not a document. So the document every mutation row below runs on was derived ONCE — by
  `🐍️derive-text-fixture.py` in the ticket folder — from
  `../../../🌐️html/🧫️fixtures/🌐️zukunft-bau-entwerfen-mit-bestand.html`, the real 150 KB
  TYPO3-published German page "Zukunft Bau: Entwerfen mit Bestand" already committed as this
  repository's own HTML 5 fixture, read with Python's own stdlib `html.parser`. Every one of the 384
  runs is a real text node of that page, every one of the 344 marks is a real `<strong>` or a real
  `<a href>` with the real URL the page links to, and the language is the page's own
  `<html lang="de">`. The result is 70 816 bytes of DSL and 35 241 bytes of pack, against 203 and 118
  for the note the case used to rest on. `html.parser` reads HTML, not a semio envelope, and cannot
  express a single one of the seven verbs, which is why it is the source of the ARTIFACT and never
  the oracle.

  A limit of the source, stated rather than papered over: the real page uses `<strong>` and
  `<a href>` and nothing else inline — no `<em>`, no `<code>` — so the artifact itself carries the
  `bold` and `link` arms only, exactly as the three-run note did. The `italic` arm is reached by
  `insert-run`'s parameter and the `code` arm by neither, which is the same coverage this case had
  before and is not hidden here.

  The `mutate-` and `inverse-` parameters are chosen against the article's own shape, so a plausible
  wrong codec fails: `insert-run` puts a marked French run at index 331, immediately after the bold
  run 330 "Entwerfen mit Bestand" and deep inside the document rather than at either end;
  `remove-run` deletes the MARKED run 336 "Projektnummer"; `edit-run` rewrites the content of run 60
  "Baustellenblog", which carries a link with a real non-empty href, so a rewrite dropping marks
  fails; `change-run-language` retags run 330 as `en` while all 383 siblings stay `de`;
  `reorder-runs` moves the unmarked run 329 past every bold run to position 356; `add-mark` inserts a
  `link` AHEAD of run 330's existing `bold` so an implementation that merely appended fails; and
  `remove-mark` detaches run 60's only mark, the one carrying the non-empty `href`.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each kind, now applied by BOTH
  implementations and checked against the committed after-snapshot by each of them in role. Nothing
  was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions, over FOUR
  files. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an
  exact re-emission is the CORRECT answer here and the wave's must-differ tripwire would be
  backwards, which is why the Rust side asserts `law::carrier_is_exact`. The three-run note's two
  encodings were written by the RUST codec and the Python side reproduces them byte for byte from
  the grammar alone — it is kept for exactly that reason, and nothing it proved was given up — while
  the article's two encodings were written by the PYTHON implementation and the Rust codec has to
  reproduce THOSE, 384 runs and 344 marks among them.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real published article
    Given the real article local://🧪️zukunft-bau-entwerfen-mit-bestand/🗣️.dsl.semio
    When the <id> mutation is applied to the article parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id                  | mutation |
      | insert-run          | {"InsertRun":{"index":331,"run":{"language":"fr","content":"concevoir avec l'existant","marks":[{"kind":"italic","href":""}]}}} |
      | remove-run          | {"RemoveRun":{"index":336}} |
      | edit-run            | {"EditRun":{"index":60,"new_content":"Baustellenblog Variowohnungen"}} |
      | change-run-language | {"ChangeRunLanguage":{"index":330,"new_language":"en"}} |
      | reorder-runs        | {"ReorderRuns":{"from":329,"to":356}} |
      | add-mark            | {"AddMark":{"run_index":330,"index":0,"mark":{"kind":"link","href":"https://www.zukunftbau.de/projekte/forschungsfoerderung"}}} |
      | remove-mark         | {"RemoveMark":{"run_index":60,"index":0}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real published article
    Given the real article local://🧪️zukunft-bau-entwerfen-mit-bestand/🗣️.dsl.semio
    When the <id> mutation is applied to the article parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the article and agree on the mutated and the restored snapshot
    Examples:
      | id                  | mutation |
      | insert-run          | {"InsertRun":{"index":331,"run":{"language":"fr","content":"concevoir avec l'existant","marks":[{"kind":"italic","href":""}]}}} |
      | remove-run          | {"RemoveRun":{"index":336}} |
      | edit-run            | {"EditRun":{"index":60,"new_content":"Baustellenblog Variowohnungen"}} |
      | change-run-language | {"ChangeRunLanguage":{"index":330,"new_language":"en"}} |
      | reorder-runs        | {"ReorderRuns":{"from":329,"to":356}} |
      | add-mark            | {"AddMark":{"run_index":330,"index":0,"mark":{"kind":"link","href":"https://www.zukunftbau.de/projekte/forschungsfoerderung"}}} |
      | remove-mark         | {"RemoveMark":{"run_index":60,"index":0}} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                  | dir                  | fixture                                       |
      | insert-run          | 📥insert-run          | inserts-a-german-run-between-two-english-runs |
      | remove-run          | 🗑️remove-run         | removes-the-middle-run                        |
      | edit-run            | ✏️edit-run           | rewrites-the-marked-runs-content              |
      | change-run-language | 🌐change-run-language | retags-the-second-run-as-german               |
      | reorder-runs        | 🔀reorder-runs        | moves-the-first-run-to-the-end                |
      | add-mark            | ➕add-mark            | adds-a-link-mark-ahead-of-the-bold-mark       |
      | remove-mark         | ➖remove-mark         | detaches-the-italic-mark-from-the-run         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both encodings of the committed note and of the real published article
    Given the real committed text artifact asset://📚️examples/📃️note/🖼️assets/🗣️.dsl.semio
    And its committed binary twin asset://📚️examples/📃️note/🖼️assets/🎒️.pack.semio
    And the real article local://🧪️zukunft-bau-entwerfen-mit-bestand/🗣️.dsl.semio
    And its binary twin local://🎒️.pack.semio
    When each implementation parses all four files, prints both documents back and re-encodes both packs
    Then all four files are reproduced byte for byte and the two implementations agree on both documents and on the digests of what they emitted
