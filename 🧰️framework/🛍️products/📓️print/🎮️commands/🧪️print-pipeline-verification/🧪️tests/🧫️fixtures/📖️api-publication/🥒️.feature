Feature: Complete authored API publication layout
  Scenario Outline: Consumer PDF resolves authored scientific defaults and localized headings
    Given the complete authored API and 91 independent default samples for 51 scientific tables
    When the publication owner produces the <language> <theme> PDF
    Then all headings and contents entries use the selected language
    And every affected table publishes the scoped boolean false defaults
    And every caption badge remains at least 2 points beyond its visible title
    And PDF.js reads every page with valid glyphs and page bounds
    And affected table pages are rendered for retained visual review
    Examples:
      | language | theme |
      | en       | light |
      | en       | dark  |
      | de       | light |
      | de       | dark  |