@capability-semio-v1-table-mutate
@no-oracle-semio-table-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-table
Feature: Apply every typed semio TABLE mutation to its committed specification fixtures
  `s.stdio.semio.table` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-table-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️table/
  🧪️oracle/🔣️component.json`). Every one of this subset's 8 kinds already carries an independently
  handcrafted `(before, mutation, after, diff)` specification fixture under its own leaf's
  `🧪️tests/` directory, authored by hand and already unit-tested inside the production crate itself
  — this feature re-exercises those SAME committed fixtures end-to-end through
  `apply_semio_table_mutation`, the entry point this ticket added, instead of calling
  `Mutation::diff`/`inverse` directly the way the in-crate tests do. Every fixture file is declared
  here as an `asset://` reference into its own committed leaf directory (never copied, never
  duplicated) and read at run time through the host's `Context::fixture_json`, so BOTH the `oracle`
  role (no recomputation, no reimplementation) and the `subject` role (decoded once into real
  `SemioTableSnapshot`/`SemioTableMutation` values, then run through the real production entry
  point) read the exact same committed bytes rather than a hand-transcribed copy that could drift
  from them.

  ⚖️ Because this case records a no-oracle decision, the runner executes NO oracle role: it resolves
  an oracle implementation from an `@oracle-` tag this feature deliberately does not carry, so the
  comparison profile never receives two sides to compare and the `oracle` handlers below are the
  written statement of the reference answer rather than a second running party. Every law this
  feature claims is therefore asserted INSIDE the subject handler, which fails with both documents
  printed. A handler that merely ran the mutation and returned would report a pass having checked
  nothing. Here that means the applied table is checked against the committed
  after-snapshot with column ORDER and row ORDER significant — which is the only way `reorder-columns`
  and `reorder-rows` can be told apart from a rebuild that keeps the same set — and the undone table
  against the committed before-snapshot, cell values included.

  The `identity-round-trip` scenario carries the BYTE half of the identity law as well as the
  semantic half. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin,
  and both committed example files were produced by these very codecs — so re-printing the parsed
  snapshot and re-encoding it must reproduce those files BYTE FOR BYTE, and the scenario asserts
  exactly that through the shared `law::carrier_is_exact`. The must-differ tripwire the wave applies
  to third-party carriers would be backwards here: a re-emission that DIFFERED would be the defect,
  not the evidence. The two encodings also cross-check each other — the binary twin has to decode to
  the same document the text does, which no single codec can arrange on its own.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/⬅️before/🔣️component.json for the <id> kind
    And the committed mutation fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/🦠️mutation/🔣️component.json for the <id> kind
    And the committed after-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/➡️after/🔣️component.json for the <id> kind
    When <id> is applied through apply_semio_table_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id              | dir              | slug                                                       |
      | create-column   | 🏗️create-column   | appends-a-float-column-and-null-pads-every-row             |
      | delete-column   | 🗑️delete-column   | drops-the-middle-column-and-cascades-into-every-row        |
      | rename-column   | 🏷️rename-column   | renames-city-to-town-without-touching-any-row               |
      | reorder-columns | 🔀reorder-columns | moves-the-area-column-to-the-front-and-realigns-every-row  |
      | insert-row      | 📥insert-row      | inserts-a-row-between-the-two-existing-rows                |
      | remove-row      | ➖remove-row      | removes-the-leading-row                                    |
      | reorder-rows    | 🔃reorder-rows    | moves-the-last-row-to-the-front                            |
      | edit-cell       | ✏️edit-cell       | rewrites-the-population-cell-of-the-second-row              |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/⬅️before/🔣️component.json for the <id> kind
    And the committed mutation fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/🦠️mutation/🔣️component.json for the <id> kind
    When <id> is applied through apply_semio_table_mutation
    And the mutation's own computed inverse is applied through apply_semio_table_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id              | dir              | slug                                                       |
      | create-column   | 🏗️create-column   | appends-a-float-column-and-null-pads-every-row             |
      | delete-column   | 🗑️delete-column   | drops-the-middle-column-and-cascades-into-every-row        |
      | rename-column   | 🏷️rename-column   | renames-city-to-town-without-touching-any-row               |
      | reorder-columns | 🔀reorder-columns | moves-the-area-column-to-the-front-and-realigns-every-row  |
      | insert-row      | 📥insert-row      | inserts-a-row-between-the-two-existing-rows                |
      | remove-row      | ➖remove-row      | removes-the-leading-row                                    |
      | reorder-rows    | 🔃reorder-rows    | moves-the-last-row-to-the-front                            |
      | edit-cell       | ✏️edit-cell       | rewrites-the-population-cell-of-the-second-row              |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real sheet artifact through both of its committed encodings and reproduce each byte for byte
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️table/📚️examples/📃️sheet/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️table/📚️examples/📃️sheet/🖼️assets/🎒️example.pack.semio
    When the text artifact is parsed and printed back to DSL, and the binary twin is decoded and re-encoded
    Then both encodings decode to the same sheet and each re-encoding reproduces its committed file byte for byte
