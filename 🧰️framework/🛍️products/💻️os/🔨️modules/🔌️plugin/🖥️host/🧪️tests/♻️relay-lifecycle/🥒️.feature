@capability-plugin-host-relay-lifecycle
@no-oracle-plugin-host-relay-lifecycle-state-machine
@comparison-ordered-json-v1
Feature: Retire replay and mounted relay owners through bounded lifecycle transitions
  The neutral fixture fixes capacity, first-reason, cancellation, generation, caller ownership, and
  balanced-accounting traces independently of either language implementation.

  @id-production-traces
  @level-fundamental
  @mode-conformance
  Scenario Outline: Interpret the <id> lifecycle trace
    Given the schema-valid neutral relay lifecycle fixture
    When the named lifecycle trace is interpreted
      """
      {"id":"<id>"}
      """
    Then its final state, first reason, cancellation count, bounded releases, caller output, and accounting equal the committed expectation
    Examples:
      | id                                |
      | replay-first-fault-wins           |
      | relay-abandoned-blocked-wake      |
      | relay-live-terminal-caller-output |
      | relay-stale-generation-refused    |
      | relay-max-plus-one-refused        |
