Feature: Owned Staged Body Positions
  Scenario Outline: Admit Only The Bounded Native Form Shape
    Given the closed language neutral body position vector
    When the issued node is checked at that position
    Then only a Form within the shared content and navigation body size is admitted
    And a Field contains exactly one ordinary control or read only text
    And cyclic positions cannot recurse into another body
    And sibling IDs are unique while field scoped DOM IDs disambiguate shared leaves
