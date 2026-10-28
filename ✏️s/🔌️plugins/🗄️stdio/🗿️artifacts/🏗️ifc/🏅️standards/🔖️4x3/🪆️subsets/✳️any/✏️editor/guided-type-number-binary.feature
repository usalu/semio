Feature: Source-bound numeric and binary guided fields
  Scenario Outline: Preserve original typed drafts without selecting required defaults
    Given the released source declaration and exact source-bound type request
    When the original guided producer builds numeric and binary fields
    Then the fields match the closed physical control and raw payload cases
    And NUMBER requires an explicit integer or real representation
    And every supplied raw payload stays editable without normalization
    And readiness requires actual private candidate admission and local rules
    And empty BINARY 0 remains distinct from a blank draft
    And original source, request identity, cancellation and paid close are preserved
