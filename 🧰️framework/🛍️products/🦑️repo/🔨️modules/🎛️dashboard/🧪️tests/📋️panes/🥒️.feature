Feature: Lists, help and usage are generated from the keymap and the catalogue
  The tasks list, the keyboard help, the footer and the usage text have no key or hint of their own:
  they are generated from the effective keymap and the catalogue.

  Scenario: The tasks list keeps its selection on the same task and shows status per row
    Given tasks that start, end and arrive while the list is open
    When the list is shown again
    Then the selection stays on the same task and each row shows its status

  Scenario: The keyboard help lists every binding of the effective keymap
    Given the effective keymap
    When the developer opens the keyboard help
    Then every action is listed with its keys

  Scenario: The usage text is generated from the keymap in both languages
    Given the preferences language en and language de
    When the usage text is generated
    Then it lists the keys of the keymap in that language

  Scenario: Hints fit whole at eighty columns in both languages
    Given a terminal of eighty columns and the languages en and de
    When the footer shows its hints, armed and idle
    Then each hint is shown whole and none is cut
