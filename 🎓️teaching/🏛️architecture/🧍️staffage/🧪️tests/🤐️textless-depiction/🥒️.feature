Feature: A depiction names a language only when it renders text
  Scenario: textless depiction
    Given depiction options for a cast of figures, which render no text
    Then the options carry no language and switch off every text annotation and the free-body inset
    When the figure is drawn in each of the seven views
    Then no scene contains a text item
    And forcing a text annotation on without a language fails instead of falling back on a language

  Scenario: textual depiction
    Given depiction options for a language
    When the figure is drawn
    Then its dimension text is formatted in that language
