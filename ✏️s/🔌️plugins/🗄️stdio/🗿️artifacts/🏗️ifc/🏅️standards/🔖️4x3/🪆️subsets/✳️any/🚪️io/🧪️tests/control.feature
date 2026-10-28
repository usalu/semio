Feature: Owned STEP Exchange Has Bounded Work Frontiers
  Scenario: Export Uses Admitted Private Input
    Given A Caller Owned IFC Graph
    When Export Captures And Admits Its Private Graph
    And The Caller Changes Its Source During A Later Write Callback
    Then Export Contains Only The Admitted Private Input

  Scenario: Cancel Interior Work In One Large Record
    Given A Large STEP Trivia, Unicode Or Binary Token Or A Large IFC Argument
    When Cancellation Is Requested At An Interior Work Checkpoint
    Then The Original Cancellation Refusal Occurs Before Publication

  Scenario: Stream Bounded Output Chunks
    Given A Graph With One Large Aggregate
    When The STEP Chunk Encoder Runs
    Then Each Chunk Fits The Neutral Maximum
    And Concatenated Chunks Reimport To The Exact Graph

  Scenario: Admit Mandatory Header Shapes And Order
    Given Each Malformed Header From The Neutral Cases
    When Import Or Export Is Requested
    Then The Operation Refuses The Invalid Header
