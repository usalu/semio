Feature: Central Unpublished Projection Ownership
  Scenario: Applying to an unpublished owner preserves projection storage
    Given an unpublished projection with fields 1 and 2
    When the central applier changes them to 7 and 8
    Then the projection contains 7 and 8 without cloning the complete projection
    And both displaced fields retire through their typed owners

  Scenario: A later field refusal cannot publish an earlier field
    Given an unpublished projection with fields 1 and 2
    When the first field is prepared as 7 and the second field refuses
    Then the supplied owner remains exactly 1 and 2
    And the abandoned prepared field retires through its typed owner

  Scenario: Cold evaluation owns and retires its scratch projection
    Given an original projection with fields 1 and 2
    When a cold application prepares the first field and then refuses
    Then the original remains exactly 1 and 2
    And the single scratch clone retires through its projection owner
