Feature: Original resident page actions
  Scenario: Page source changes preserve command arguments
    Given the exact current resident fixed page button and full status token
    When its original activate intent arrives without raw input
    Then only the resident page destination is retained
    And command arguments retain their exact identity and values
    And generation advances exactly once and old layout authority retires
    And execution waits for the newest complete status

  Scenario: Preserved partial text does not authorize old page controls
    Given a raw partial text draft with a newer request generation
    When an older resident page button is pressed
    Then the page action is refused without consuming its sequence
    And a newly validated current publication may page even if its status is invalid

  Scenario: Page data and command args remain separate
    Given canonical u64 offsets above binary64 precision
    When a page action replaces layout
    Then offsets remain exact strings outside command args
    And reset and canonical source revision change clear page addresses
