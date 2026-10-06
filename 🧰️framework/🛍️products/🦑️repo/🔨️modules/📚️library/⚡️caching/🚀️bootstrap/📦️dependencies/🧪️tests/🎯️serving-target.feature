Feature: Watcher reconciliation waits for the exact serving target
  Scenario: Another target shares the serving target prefix
    Given the initial graph is serving application:dev
    When application:dev-tools starts
    Then initial preparation remains pending
    When the complete colored application:dev marker arrives in several chunks
    Then preparation completes and its observers are removed
  Scenario: Initial preparation fails before the serving target
    When the initial graph exits before the serving marker
    Then preparation fails and reconciliation does not run
  Scenario: Source changes arrive while the initial graph is preparing
    Given automatic watcher callbacks are not registered yet
    When the initial prerequisite graph completes
    Then source watching registers
    And its first reconciliation includes edits made during preparation
    And cancellation or initial failure prevents registration
