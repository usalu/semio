Feature: STEP ISO 8859 Alphabet Directives
  Scenario Outline: Decode Every Shifted Character on Every Normative Page
    Given an original case from cases.json
    When its source is read inside a Part 21 string
    Then its single decoded scalar equals expected
    And an undefined code is refused without replacement text

  Scenario: Each String Resets the Alphabet to A
    Given a string selecting page B followed by a string without an alphabet selector
    When the physical file is read
    Then only the first string uses page B

  Scenario: A Shift Payload Is One Graphic ASCII Character
    Given a PAGE directive whose payload is an apostrophe or reverse solidus
    When the physical file is read
    Then the payload is consumed once without ordinary quote or slash escaping
