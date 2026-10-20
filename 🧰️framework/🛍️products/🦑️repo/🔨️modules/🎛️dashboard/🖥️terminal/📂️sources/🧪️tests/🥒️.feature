Feature: Bounded Document Progress
  Scenario: Yield before processing more than one message quantum
    Given a document operation has queued progress and its terminal outcome
    When the dashboard polls that operation
    Then each poll consumes at most 32 messages
    And the shared poll count agrees with an independent SQL oracle
    And remaining messages and cancellation settle in later polls

  Scenario: Yielded work keeps the native viewer awake
    Given a full progress quantum leaves a document operation active
    When its poll yields to keyboard and frame handling
    Then the viewer is woken for a subsequent poll
