@capability-s-home-1-any-editor-transient-mutate
@no-oracle-s-home-1-any-editor-transient-state-lane-semantics
@comparison-ordered-json-v1
@mutations-s-home-1-any-editor-transient
Feature: Fold every committed directory page vector of s.space.home's ✏️editor/🫧️transient lane
  `HomeTransientMutation` is the transient lane of the Home launcher's editor: it folds authenticated, receipt-sealed hub
  directory pages into the local directory projection both Home surfaces list. The projection is derived hub state — never
  persisted, shared or undone — so its one verb `apply-directory-page` is non-invertible by declaration, and each page is one
  bounded item priced by the page, never by the projection. It is this repository's own state record, so the case records the
  no-oracle decision `s-home-1-any-editor-transient-state-lane-semantics` in `✏️editor/🫧️transient/🔮️oracles/🔣️.json` and
  asserts every law inside the subject handlers through the shared `semio_s_plugin_stdio_test_oracle::law::vector` module: the
  applied snapshot is the committed after-snapshot, the produced delta is the committed `🔺️diff`, the diagnostics are the ones
  the committed `🎯️outcome` declares, and only an applied vector moves the snapshot. Production dispatch is reached through
  `home_transient_mutation_report_json`, which runs `Mutation::diff(..).apply_to` exactly as the transient store does.

  The kind is non-invertible, so its inverse law is asserted where it is meaningful: a page that folds nothing (a held
  frontier, a refused race) leaves nothing to undo — the kind computes no inverse step and the projection stays the
  committed before-snapshot. A folded page is never undone; the host re-reads the directory from its origin instead.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Fold <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> vector under ✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory
    When its page is folded into that vector's before-snapshot through home_transient_mutation_report_json
    Then the folded snapshot, the produced diff and the diagnostics are exactly what the vector commits
    Examples:
      | id |
      | apply-directory-page-applied |
      | apply-directory-page-no-op |
      | apply-directory-page-rejected |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> leaves the committed before-snapshot
    Given the committed <id> vector under ✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory
    When its page is folded into that vector's before-snapshot through home_transient_mutation_report_json
    Then the kind computes no inverse step and the projection is exactly the committed before-snapshot
    Examples:
      | id |
      | apply-directory-page-no-op |
      | apply-directory-page-rejected |
