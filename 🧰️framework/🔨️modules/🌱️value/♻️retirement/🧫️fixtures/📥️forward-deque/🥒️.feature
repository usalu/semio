Feature: Original Forward Deque Retirement
  Scenario Outline: The original integer allocation remains under exact retirement authority
    Given an original forward deque containing <rows> sequential unsigned integers
    When the owner is created without a copy or allocation
    Then its pointer and physical backing are unchanged
    And independent JSON serialization contains the same complete integer sequence
    And a release grant shorter than the next physical demand releases no allocation
    And each paid step fits the original item copy capacity release and depth grants
    And terminal retirement empties the owner and releases its physical backing
    Examples:
      | rows |
      | 1    |
      | 3    |
      | 257  |
