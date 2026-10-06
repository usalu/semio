Feature: Source Watcher Preparation Barrier
  Scenario: The watcher becomes ready during cold preparation
    Given the initial renderer prerequisite graph is still running
    When the source watcher becomes ready
    Then no second activation starts
    When Nx starts the initial serving target after its prerequisites
    Then source reconciliation starts exactly once
  Scenario: Cancellation precedes reconciliation
    Given a watcher is awaiting initial preparation
    When its owner is cancelled
    Then reconciliation never starts
  Scenario Outline: Explicit launch selections isolate source watchers
    Given the same renderer, example and port
    When its language is <language> and its terminology is <terminology>
    Then its graph store and socket belong to that exact selection
    And their identity matches the independent SHA-256 oracle
    Examples:
      | language | terminology |
      | en       | native      |
      | en       | reuse       |
      | de       | native      |
      | de       | reuse       |
