Feature: Original prepared candidate local WHERE rules
  Scenario: Every selected issued rule must be true for local approval
    Given the exact original caller source and strict prepared SetEntityArg witness
    When the issued retained machine evaluates the exact received candidate row
    Then inherited entity and defined alias WHERE rules use received SELF
    And false fails while unknown or unsupported remains incomplete
    And no global or full schema conformance is asserted
  Scenario: Denied and canceled prefixes retain original owners for paid close
