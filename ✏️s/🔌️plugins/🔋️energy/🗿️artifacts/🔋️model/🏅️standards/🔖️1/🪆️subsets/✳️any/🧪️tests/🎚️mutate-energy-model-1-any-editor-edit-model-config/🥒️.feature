@capability-energy-model-1-any-editor-edit-model-config-mutate
@no-oracle-energy-model-1-any-editor-edit-model-config-state-lane-semantics
@comparison-ordered-json-v1
@mutations-energy-model-1-any-editor-edit-model-config
Feature: Apply every config state-lane mutation of s.energy.model's ✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🎚️config to its committed vector
  `EnergyModelWindowConfigMutation` is the config lane of this subset's editor: it holds the 3d model window's camera pose (position, target, zoom). It is this
  repository's own state record, so the case records the no-oracle decision `energy-model-1-any-editor-edit-model-config-state-lane-semantics` in
  `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🎚️config/🔮️oracles/🔣️.json` and asserts every law inside the subject handlers through the shared
  `semio_s_plugin_stdio_test_oracle::law::vector` module: the applied snapshot is the committed after-snapshot, the
  produced delta is the committed `🔺️diff`, the diagnostics are the ones the committed `🎯️outcome` declares, an
  applied vector really moves the snapshot, and the mutation's own computed inverse restores the committed
  before-snapshot. Production dispatch is reached through `energy_model_window_config_mutation_report_json`, which runs
  `Mutation::diff(..).apply_to` exactly as the store does.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> vector under ✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🎚️config/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through energy_model_window_config_mutation_report_json
    Then the applied snapshot, the produced diff and the diagnostics are exactly what the vector commits, and the snapshot moved
    Examples:
      | id |
      | set-camera |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> vector under ✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🎚️config/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through energy_model_window_config_mutation_report_json
    Then the mutation's own inverse steps apply without refusal and restore the before-snapshot exactly
    Examples:
      | id |
      | set-camera |

  @id-keep
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Re-applying <id> to a snapshot that already holds its value is a warned no-op
    Given the committed <id> vector under ✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🎚️config/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through energy_model_window_config_mutation_report_json
    Then the snapshot is unchanged and the only diagnostic is mutation.no-op
    Examples:
      | id |
      | set-camera |
