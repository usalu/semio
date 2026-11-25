Feature: The air flow layer is styled without an inline style element
  Scenario: A browser with constructable style sheets
    Given a page whose Content-Security-Policy forbids inline style elements
    When the air flow layer is put into the page
    Then its style is one style sheet adopted by the document and no style element is created
    And putting a second layer into the page adopts nothing more

  Scenario: A browser without constructable style sheets
    When the air flow layer is put into the page
    Then its style is one style element in the head
