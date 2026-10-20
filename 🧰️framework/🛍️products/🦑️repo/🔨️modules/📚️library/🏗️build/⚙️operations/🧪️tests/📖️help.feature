Feature: Canonical Native Build Help
  Scenario Outline: Human help names the installed native frontend
    Given a declared help language <language>
    When the neutral catalogue renders root, command and subcommand help
    Then usage and example commands begin with repo
    And the catalogue schema refuses a Bun frontend prefix
    And SQLite independently extracts the same native example commands

    Examples:
      | language |
      | en       |
      | de       |
