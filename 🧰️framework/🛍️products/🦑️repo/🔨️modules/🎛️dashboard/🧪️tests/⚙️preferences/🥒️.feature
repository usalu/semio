Feature: Opinionated Optional Dashboard Preferences
  Scenario: Starting without preferences
    Given no workspace, local, environment or argument overrides
    When the native dashboard starts
    Then it immediately shows task controls in the language the system names, or in the first language of the declared fallback chain, English, with dark appearance
    And native terminology, React, tabbed output and the Ctrl B prefix key are selected without questions
  Scenario: Resolving layered preferences
    Given the shared preference vectors
    When workspace events, local events, environment and arguments are applied in order
    Then the effective preferences equal the vector expectations
    And invalid, unknown and empty changes are rejected
  Scenario: Customization survives reopening
    Given a local preference journal
    When a preference change command commits an event
    Then a new view replays that event with the same preferences
    And concurrent writers commit consecutive revisions
  Scenario: The prefix key and single bindings are customized by events
    Given a workspace journal that sets the prefix key and rebinds one action
    And a local journal that rebinds the same action again and another one
    When the preferences are replayed workspace first and local second
    Then the later layer wins per binding and the other bindings stay
    And a prefix that only types, a binding without keys or an unknown binding is rejected before it is published
  Scenario: The language falls back from the stored choice to the system and then to the declared chain
    Given the language chain declared as data and no hard-coded default language
    When a stored choice, an environment override, a system locale and no locale at all are resolved in turn
    Then a stored choice beats the system and the override beats the stored choice
    And the system locale beats the first language of the declared fallback chain
    And a compacted journal never freezes a language the journal did not choose
  Scenario: The language follows the preference everywhere
    Given the preference language de
    When the native dashboard shows its windows, rows, hints, status line, forms and keyboard help
    Then every text comes from the catalogue in German and none from a hard-coded English default
  Scenario: Discovery does not block interaction
    Given a large workspace with no inventory cache
    When the native dashboard starts
    Then its first usable task controls appear in under 250 milliseconds
    And discovery and connection show progress and remain cancellable
    And typing a command filter selects one command without presentation questions
  Scenario: Keyboard and mouse use the same command selection
    Given the shared searchable launcher vectors
    When the developer searches with multiple words or Unicode labels
    Then Enter and primary-button clicks select the same visible command
    And the active row remains visible while scrolling
  Scenario: Preferences pre-select what a command offers
    Given the preferences language de and renderer wgpu-wasm
    When a playground command opens its parameter form
    Then language and renderer are pre-selected and a command without those parameters shows none
