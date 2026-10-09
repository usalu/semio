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

  Scenario: The launcher gives text editors a visible cursor
    Given a command with a required text parameter
    When its configuration form opens
    Then the parameter editor owns keyboard focus and displays the terminal cursor
    When the developer selects a non-text field
    Then the list owns keyboard focus and the editor cursor is hidden

  Scenario: Launcher text editing follows the shared input caret
    Given a text parameter containing wide characters and combining marks
    When the developer moves the caret and edits with keys, paste and the mouse
    Then edits occur at grapheme boundaries and remain in the resolved command
    And repainting preserves the caret within that text field

  Scenario: Confirmation is operable with the mouse
    Given a mutating command awaiting confirmation
    When the developer clicks a preview row
    Then no command starts
    When the developer clicks the visible start action
    Then the confirmed command starts once

  Scenario: The configured start action needs one click
    Given a configured command and keyboard focus in a text field
    When the developer clicks the visible start action once
    Then that command starts once with the entered value

  Scenario: Glyph capabilities preserve the framework width policy
    Given the framework scalar width policy or an explicitly chosen cluster policy
    When the dashboard adapts its glyph repertoire to terminal capabilities
    Then the selected width policy is preserved

  Scenario: A pending action is visible before routine footer status
    Given an English or German dashboard in an eighty-column terminal
    When the developer arms the prefix
    Then the waiting notice precedes task counts, connection and discovery status
    And the notice appears on the actual footer row
  Scenario: Restore requested before the first snapshot raises the replayed task
    Given a newly connected view has not received its task snapshot
    When the developer requests restore tasks
    And the daemon sends the task snapshot
    Then the newest task output holds keyboard focus without another action
