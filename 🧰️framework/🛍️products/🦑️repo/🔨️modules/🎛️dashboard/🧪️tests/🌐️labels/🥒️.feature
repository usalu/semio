Feature: Every text of the dashboard is in the language of the developer
  The catalogue states each text in English and German, compile-checked. Identifiers such as command
  ids, parameter values and key names are data and stay verbatim.

  Scenario: Every label is complete in every language with the same placeholders
    Given the catalogue
    When each label is read in English and in German
    Then none is empty and both name the same placeholders

  Scenario: Every keymap action has a text in every language
    Given the shipped keymap
    When the text of each action id is looked up in English and in German
    Then every action id has a text in both languages

  Scenario: The catalogue follows the chosen language
    Given the preference language en and the preference language de
    When the catalogue is read for each of them
    Then the texts are those of that language

  Scenario: No visible text is hard-coded outside the catalogue
    Given the sources of the native view
    When they are scanned for texts that reach the screen
    Then every one of them comes from the catalogue

  Scenario: The messages of the command line follow the preference
    Given a workspace without a stored preference
    When the language is chosen by an explicit flag
    Then the usage, status and failure messages of the daemon command are those of that language
