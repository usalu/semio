Feature: Readable Print semantic foregrounds
  Scenario: Normal text uses its matching semantic foreground
    Given an explicit Print theme
    When generated text and surface roles are resolved
    Then each body, cover, link, outline and filled heading pair has contrast at least 4.5
    And the contrast agrees with an independent color library
  Scenario: Former border and on-accent choices remain rejected
    Then the known gray-on-cream, cream-on-teal and red-on-cream pairs fail normal text contrast

  Scenario: Heading Number Segments Use Their Heading Foreground
    Given each print theme and primary, secondary, tertiary filled heading surface
    When a hierarchical heading number is painted
    Then both its prefix and leaf use the accent foreground
    And both satisfy normal text contrast against the filled surface
    And outlined heading numbers use the ordinary foreground on the canvas
