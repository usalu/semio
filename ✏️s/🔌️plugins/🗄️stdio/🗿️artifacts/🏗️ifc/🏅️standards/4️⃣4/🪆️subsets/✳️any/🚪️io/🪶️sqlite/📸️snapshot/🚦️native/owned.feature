Feature: Original IFC artifact Pack receiving
  Scenario: The original literal graph retains all authored syntax values
    Given an IFC Pack with complete ordered headers and complex instances
    And nested aggregates, typed values, binary content and exact binary64 payloads
    When the actual original decoder receives the Pack under its admitted grant
    Then the graph contains the identical ordered authored values
    And source, parser, bindings and graph temporaries close in the same original recipient

  Scenario Outline: A denied or cancelled physical frontier retains its actual owners
    Given original Pack receiving has advanced <prefix> turns
    When its next grant is denied or the original callback cancels
    Then no graph is published
    And every partial allocation drains through original retirement demands
    Examples:
      | prefix |
      | 0      |
      | 1      |
      | 8      |
      | 64     |
      | 512    |

  Scenario: An invalid flat value forest refuses atomically
    Given an IFC Pack reuses a child, references a preceding value or retains an orphan
    When its original decoder binds the flat records into the IFC graph
    Then it refuses the invalid forest without publishing a snapshot

  Scenario: Completed graph work cancels during a bounded binary scan
    Given the original IFC Pack decoder has entered actual graph binding
    And a canonical binary value requires more than 256 graph turns
    When its observer cancels at completed graph work 256
    Then the accepted binary turn returns its physical receipt with the cancellation
    And no graph is published
    And the partial graph drains through the same original decoder ledger and recipient
