@capability-semio-v1-text-mutate
@no-oracle-semio-text-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-text
Feature: Apply every typed semio TEXT mutation to its committed specification fixtures
  `s.stdio.semio.text` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-text-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️text/
  🧪️oracle/🔣️component.json`). Every one of this subset's 7 kinds carries an independently
  handcrafted `(before, mutation, after, diff)` specification fixture under its own leaf's
  `🧪️tests/` directory, and this feature re-exercises those SAME committed bytes end-to-end through
  `apply_semio_text_mutation` rather than calling `Mutation::diff`/`inverse` directly the way the
  in-crate fixture tests do.

  What distinguishes this subset is that its addressing is TWO levels deep. `runs` is a positional
  sequence, and each run carries its own positional `marks` collection, so `add-mark`/`remove-mark`
  address `(run_index, index)` while `insert-run`/`remove-run`/`reorder-runs` address the outer
  sequence only. The fixtures are chosen against that: `add-mark` inserts a `link` mark AHEAD of an
  existing `bold` one so a mark that merely appended would fail, `edit-run` rewrites the content of
  the run that carries a mark so a rewrite that dropped marks would fail, and `change-run-language`
  retags a run whose siblings stay `en` so a language change applied to the whole document would
  fail. `link` is the only mark kind carrying a non-empty `href`, and `SemioTextMarkKind` is
  `rename_all = "camelCase"`, so the wire spelling is `"link"`.

  Because this case records a no-oracle decision, the runner executes NO oracle role — every
  assertion below therefore lives inside the subject handler, which compares the applied snapshot
  against the committed after-snapshot and the undone snapshot against the committed
  before-snapshot, and fails with both JSON documents printed. A handler that merely ran the
  mutation and returned would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_text_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                  |
      | insert-run          |
      | remove-run          |
      | edit-run            |
      | change-run-language |
      | reorder-runs        |
      | add-mark            |
      | remove-mark         |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_text_mutation
    And the mutation's own computed inverse is applied through apply_semio_text_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id                  |
      | insert-run          |
      | remove-run          |
      | edit-run            |
      | change-run-language |
      | reorder-runs        |
      | add-mark            |
      | remove-mark         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed note through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🎒️example.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees on the same three-run note, an unmarked English run, a bold English run and a German run carrying a link href
