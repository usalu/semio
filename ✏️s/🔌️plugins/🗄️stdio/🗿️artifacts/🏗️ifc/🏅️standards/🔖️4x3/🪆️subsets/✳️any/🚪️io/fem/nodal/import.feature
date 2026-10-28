Feature: Original-owned native nodal component import
  Scenario: Exact declared optional component presence
    Given one original IFC source and its admitted source row
    When the six Force and Moment attributes are queried by their declared owner
    Then explicit finite binary64 values including signed zero produce components
    And only explicit Unset skips a component
    And missing serialized values never become Unset
  Scenario: Query status and lifetime
    Then missing ambiguous and late unknown statuses refuse
    And source values and declarations stay borrowed only inside the original query loan
    And cancelled partial owners close under original grants before publication
  Scenario: Fixed component identity copy admission
    Given the complete fixed stack-copy frontier is 116 bytes
    When the original copy allowance is only 115 bytes
    Then component identity formatting refuses before initializing either word
    And no copy or allocation progress is posted
