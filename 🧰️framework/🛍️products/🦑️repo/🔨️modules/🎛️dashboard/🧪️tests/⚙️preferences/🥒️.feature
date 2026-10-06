Feature: Opinionated Optional Dashboard Preferences
  Scenario: Starting without preferences
    Given no workspace, local, environment or argument overrides
    When the native dashboard starts
    Then it immediately shows task controls in English with dark appearance
    And native terminology, React and tabbed output are selected without questions
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
