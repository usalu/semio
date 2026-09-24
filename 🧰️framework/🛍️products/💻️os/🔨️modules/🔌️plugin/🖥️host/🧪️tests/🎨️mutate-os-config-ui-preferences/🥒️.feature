@capability-os-config-ui-preferences-1-mutate
@no-oracle-os-config-ui-preferences-mutation-semantics
@comparison-ordered-json-v1
@mutations-os-config-ui-preferences-1-any
Feature: Apply every typed UI-preferences mutation to its committed specification vectors
  `os.config.ui-preferences` is this operating system's own persisted local-only UI preference
  record: appearance, chrome layout, driver and custom drivers, locale, terminology, theme and
  custom themes, and keybinding overrides. No third party implements it and none could adjudicate
  it, so there is no reference implementation to register (recorded as the
  `os-config-ui-preferences-mutation-semantics` no-oracle decision in
  `../../../../../🎚️config/🔮️oracles/🔣️.json`).

  ⚠️ This case lives beside its opening-preferences, merge-policy and identity siblings under
  `🔌️plugin/🖥️host`, whose package re-exports `semio_framework_os_config::opening_config` — the
  production mount through which the host applies every os.config vocabulary — so all four
  vocabularies are claimed side by side through the one subject path the host itself uses.

  Nine kinds close this vocabulary because `UiPreferences` holds nine preferences and each kind
  owns exactly one of them: six optional scalars replaced whole, and three keyed maps (custom
  drivers, custom themes, keybinding overrides) where a value upserts its key and `null` removes it.
  Every kind carries two committed vectors. The `sets-*` vector runs against an untouched record,
  so it proves the kind writes its own preference and nothing else. The `keeps-*` vector runs
  against a record that already holds every preference, and re-applies the value that is already
  there, so it proves a re-set is a warned `mutation.no-op` that hands the record back unchanged
  rather than a silent write. The `field` column names the one preference a `sets-*` row must move,
  which is what lets each row be held to the claim that it touched nothing else.

  The inverse law is load-bearing on the keyed kinds: undoing an upsert into an empty map must
  REMOVE the key again rather than leave it holding the value it held after the mutation, so each
  inverse is read off the pre-mutation record, never off the mutation's own payload.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler, which compares the applied record against the
  committed after-snapshot, checks the `field` claim, and checks the reported diagnostics against
  the committed outcome. A handler that merely ran the mutation and returned would report a pass
  having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> vector
    When <id> is applied through apply_ui_preferences_config_mutation_reporting
      """
      {"vector": "<id>", "kind": "<kind>", "field": "<field>", "status": "<status>"}
      """
    Then the resulting record matches the committed after-snapshot, moves only <field> when the status is applied, and reports the committed <status> outcome
    Examples:
      | id                        | kind                    | field               | status  |
      | sets-appearance           | set-appearance          | appearance          | applied |
      | keeps-appearance          | set-appearance          | appearance          | no-op   |
      | sets-layout               | set-layout              | layout              | applied |
      | keeps-layout              | set-layout              | layout              | no-op   |
      | sets-driver               | set-driver              | driverId            | applied |
      | keeps-driver              | set-driver              | driverId            | no-op   |
      | sets-custom-driver        | set-custom-driver       | customDrivers       | applied |
      | keeps-custom-driver       | set-custom-driver       | customDrivers       | no-op   |
      | sets-locale               | set-locale              | locale              | applied |
      | keeps-locale              | set-locale              | locale              | no-op   |
      | sets-terminology          | set-terminology         | terminology         | applied |
      | keeps-terminology         | set-terminology         | terminology         | no-op   |
      | sets-theme                | set-theme               | themeId             | applied |
      | keeps-theme               | set-theme               | themeId             | no-op   |
      | sets-custom-theme         | set-custom-theme        | customThemes        | applied |
      | keeps-custom-theme        | set-custom-theme        | customThemes        | no-op   |
      | sets-keybinding           | set-keybinding-override | keybindingOverrides | applied |
      | keeps-keybinding          | set-keybinding-override | keybindingOverrides | no-op   |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> vector
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"vector": "<id>", "kind": "<kind>", "field": "<field>", "status": "applied"}
      """
    Then the record equals the committed before-snapshot again, and <field> holds its original value
    Examples:
      | id                 | kind                    | field               |
      | sets-appearance    | set-appearance          | appearance          |
      | sets-layout        | set-layout              | layout              |
      | sets-driver        | set-driver              | driverId            |
      | sets-custom-driver | set-custom-driver       | customDrivers       |
      | sets-locale        | set-locale              | locale              |
      | sets-terminology   | set-terminology         | terminology         |
      | sets-theme         | set-theme               | themeId             |
      | sets-custom-theme  | set-custom-theme        | customThemes        |
      | sets-keybinding    | set-keybinding-override | keybindingOverrides |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the committed fully populated preference record without passing bytes through
    Given the committed before-snapshot of the keeps-keybinding vector, which holds every preference
    When the record is decoded into UiPreferences and re-encoded from the typed value alone
    Then the re-encoded projection is the committed one, member for member, and the decode is proven real by reading the locale, the custom driver scale and the keybinding back off the typed value rather than off the text
