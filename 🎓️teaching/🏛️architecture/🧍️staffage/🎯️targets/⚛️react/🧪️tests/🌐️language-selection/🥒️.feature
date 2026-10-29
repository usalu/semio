Feature: No default language

  Scenario: A stored choice wins
    Given the reader once chose German and the browser prefers English
    Then the archive speaks German

  Scenario: The browser's list preselects
    Given nothing is stored and the browser lists French, then German
    Then the archive speaks German without storing it

  Scenario: Nothing matches
    Given nothing is stored and the browser lists only French and Japanese
    Then the archive asks in English and German at once and the document has no language

  Scenario: Every text exists in both languages
    Then both bundles have the same keys, no text is empty and placeholders agree
