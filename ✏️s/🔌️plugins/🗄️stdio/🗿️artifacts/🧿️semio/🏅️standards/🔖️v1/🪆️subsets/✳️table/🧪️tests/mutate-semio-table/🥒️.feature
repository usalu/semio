@capability-semio-v1-table-mutate
@oracle-semio-table-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-table
Feature: Apply every typed semio TABLE mutation to a real 50-row survey table, against an independent Python implementation
  `s.stdio.semio.table` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio` or `.pack.semio`, so the second producer a differential comparison needs is a second
  IMPLEMENTATION. `🐍️component.py` beside this file is that implementation — the envelope, the DSL
  grammar, the `SemioValue` cell grammar, the pack frame and all eight verbs with their inverses,
  written in Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
  `…/🧬️mutations/🔣️.json` and the semio envelope region of
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing from and
  transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-table-python-independent` in `…/✳️table/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  🧫️ **The document under test is a real one.** The richest `s.stdio.semio.table` document committed
  anywhere in this artifact is the three-row demo sheet, which is a fixture, not a data set. So the
  table this case mutates was derived ONCE — by `derive_document_from_csv` in `🐍️component.py`, and
  re-derived on every run by the `payload-fidelity` scenario — from the real committed survey
  `../../../📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv`: 50 records over 12 columns of German
  building-material-reuse marketplace research, with commas, em dashes and umlauts inside quoted
  fields. Its source is committed beside this case as `local://🧪️reuse-marketplaces/📊️.csv` so the
  provenance is checkable in the tree, and the derivation is a faithful transcription — the header
  names the columns, every column is `str` because every source field is text, and every cell carries
  its field verbatim. The result is 600 cells, 24 399 bytes of DSL and 12 212 bytes of pack, against
  240 and 132 for the demo sheet the case used to rest on. `local://` rather than `asset://` because
  `asset://` resolves against the artifact root and the CSV lives in a sibling artifact.

  The `mutate-` and `inverse-` parameters are chosen against the survey's own shape, so a plausible
  wrong codec fails: `create-column` opens a `float` column in the MIDDLE of the twelve and has to
  null-pad all fifty rows there, `delete-column` drops a middle column and has to cascade into all
  fifty, `rename-column` retags the last column to a non-ASCII name, `reorder-columns` moves the
  fourth column to the very end and every row's cells have to follow it, `insert-row` splices in a
  record whose twelve cells between them carry every `SemioValue` scalar variant — `str`, `null`,
  `int`, `float`, `bool` and `bytes` — into columns all declared `str`, `remove-row` deletes a middle
  record, `reorder-rows` moves the last record to the front, and `edit-cell` overwrites a `str` cell
  with `bytes`, addressed by row index and column NAME.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each kind, now applied by BOTH
  implementations and checked against the committed after-snapshot by each of them in role. Nothing
  was removed to make room for the oracle.

  `payload-fidelity` is the second half of the provenance claim and the one place a genuinely
  third-party parser does the work: each side re-tokenizes the committed CSV with its own RFC 4180
  implementation — Python's `csv` module on the oracle side, this repository's `stdio.csv` codec on
  the subject side — and requires the derived table document to still carry exactly those 600 fields.
  A drifted fixture, or a disagreement between two RFC 4180 readers about a quoted field, is a red
  scenario rather than a silent one.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions. `.dsl.semio`
  is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact re-emission is
  the CORRECT answer here and the wave's must-differ tripwire would be backwards, which is why the
  Rust side asserts `law::carrier_is_exact`. What stops that being a codec agreeing with itself is
  that the two committed encodings of the demo sheet were written by the RUST codec and the Python
  side reproduces them byte for byte from the grammar alone, while the survey table's two encodings
  were written by the PYTHON implementation and the Rust codec has to reproduce THOSE — so each
  implementation is measured against bytes the other one emitted, and the digests of what each side
  re-emitted are compared across the two languages.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real 50-row survey table
    Given the real survey table local://📊️reuse-marketplaces.dsl.semio
    When the <id> mutation is applied to the table parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting table
    Examples:
      | id              | mutation |
      | create-column   | {"CreateColumn":{"name":"Bewertung","kind":"float","index":4}} |
      | delete-column   | {"DeleteColumn":{"name":"PhysischerZugang"}} |
      | rename-column   | {"RenameColumn":{"name":"Uebergabe","new_name":"Übergabe"}} |
      | reorder-columns | {"ReorderColumns":{"name":"Land","to_index":11}} |
      | insert-row      | {"InsertRow":{"index":25,"row":{"cells":[{"kind":"str","value":"BB-00"},{"kind":"str","value":"Marktplätze"},{"kind":"str","value":"Bau- und Rohstoffbörse"},{"kind":"str","value":"Österreich"},{"kind":"null"},{"kind":"str","value":"öffentlich"},{"kind":"str","value":"Website"},{"kind":"str","value":"—"},{"kind":"int","lexeme":"12"},{"kind":"float","lexeme":"0.750"},{"kind":"bool","value":true},{"kind":"bytes","value":[0,1,2,255]}]}}} |
      | remove-row      | {"RemoveRow":{"index":37}} |
      | reorder-rows    | {"ReorderRows":{"from":49,"to":0}} |
      | edit-cell       | {"EditCell":{"row_index":17,"column_name":"Zugang","new_value":{"kind":"bytes","value":[0,1,2,255]}}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real 50-row survey table
    Given the real survey table local://📊️reuse-marketplaces.dsl.semio
    When the <id> mutation is applied to the table parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the survey table and agree on the mutated and the restored table
    Examples:
      | id              | mutation |
      | create-column   | {"CreateColumn":{"name":"Bewertung","kind":"float","index":4}} |
      | delete-column   | {"DeleteColumn":{"name":"PhysischerZugang"}} |
      | rename-column   | {"RenameColumn":{"name":"Uebergabe","new_name":"Übergabe"}} |
      | reorder-columns | {"ReorderColumns":{"name":"Land","to_index":11}} |
      | insert-row      | {"InsertRow":{"index":25,"row":{"cells":[{"kind":"str","value":"BB-00"},{"kind":"str","value":"Marktplätze"},{"kind":"str","value":"Bau- und Rohstoffbörse"},{"kind":"str","value":"Österreich"},{"kind":"null"},{"kind":"str","value":"öffentlich"},{"kind":"str","value":"Website"},{"kind":"str","value":"—"},{"kind":"int","lexeme":"12"},{"kind":"float","lexeme":"0.750"},{"kind":"bool","value":true},{"kind":"bytes","value":[0,1,2,255]}]}}} |
      | remove-row      | {"RemoveRow":{"index":37}} |
      | reorder-rows    | {"ReorderRows":{"from":49,"to":0}} |
      | edit-cell       | {"EditCell":{"row_index":17,"column_name":"Zugang","new_value":{"kind":"bytes","value":[0,1,2,255]}}} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/➡️after/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id              | dir              | slug                                                      |
      | create-column   | 🏗️create-column   | appends-a-float-column-and-null-pads-every-row            |
      | delete-column   | 🗑️delete-column   | drops-the-middle-column-and-cascades-into-every-row       |
      | rename-column   | 🏷️rename-column   | renames-city-to-town-without-touching-any-row             |
      | reorder-columns | 🔀reorder-columns | moves-the-area-column-to-the-front-and-realigns-every-row |
      | insert-row      | 📥insert-row      | inserts-a-row-between-the-two-existing-rows               |
      | remove-row      | ➖remove-row      | removes-the-leading-row                                   |
      | reorder-rows    | 🔃reorder-rows    | moves-the-last-row-to-the-front                           |
      | edit-cell       | ✏️edit-cell       | rewrites-the-population-cell-of-the-second-row            |

  @id-payload-fidelity
  @level-exhaustive
  @mode-differential
  Scenario: The derived survey table still carries exactly what the real CSV carries
    Given the real committed survey source local://🧪️reuse-marketplaces/📊️.csv
    And the table document derived from it local://📊️reuse-marketplaces.dsl.semio
    When each implementation re-tokenizes the source with its own RFC 4180 reader and rebuilds the table from it
    Then the rebuilt table equals the committed derived document and the two implementations agree on all 600 fields

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both encodings of the demo sheet and of the real survey table from the parsed documents
    Given the committed demo sheet asset://📚️examples/📃️sheet/🖼️assets/🗣️.dsl.semio
    And its committed binary twin asset://📚️examples/📃️sheet/🖼️assets/🎒️.pack.semio
    And the real survey table local://📊️reuse-marketplaces.dsl.semio
    And its binary twin local://📊️reuse-marketplaces.pack.semio
    When each implementation parses all four files, prints the two documents back and re-encodes both packs
    Then all four files are reproduced byte for byte and the two implementations agree on the documents and on the digests of what they emitted
