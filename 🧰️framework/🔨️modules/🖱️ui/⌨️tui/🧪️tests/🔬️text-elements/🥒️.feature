Feature: Text And List Elements Of The Terminal Target
  Lists, trees, tables, logs, inputs, selects, scrollables, progress bars and toggles share one interaction
  vocabulary: a press selects and a double press or Enter activates, Page Up, Page Down, Home and End work
  everywhere, the wheel scrolls without moving the selection, and the caret is the terminal's own cursor.

  Scenario: A press selects and a double press activates
    Given a list, a wizard, a table and a tree under the pointer
    When a row is pressed once and then twice
    Then the first press only reports the selection and the second reports the activation of the same item

  Scenario: Signals name items, not screen positions
    Given a wizard whose filter hides the first options
    When the selection moves and an option is activated
    Then the signals carry the index of the option in the unfiltered list

  Scenario: A held Backspace cannot leave the wizard
    Given a wizard with an empty filter
    When Backspace arrives
    Then nothing happens and only Alt with Backspace or Escape on an empty filter navigates back

  Scenario: The caret moves over whole grapheme clusters
    Given an input holding an emoji with a variation selector, a letter with a combining accent and an umlaut
    When Left, Backspace and Delete are pressed
    Then each key crosses one whole cluster, the cursor never sits inside a character, and nothing panics

  Scenario: A tree filter keeps the ancestors of every match
    Given a tree of folders and commands
    When the query names a command and one of its folders
    Then the command shows under all of its ancestors, the selection lands on the match and clearing the query keeps it selected

  Scenario: A table with a stale selection does not panic
    Given a table whose selected row index is past the last row or hidden under a collapsed parent
    When any navigation key is pressed
    Then the table continues from the first visible row

  Scenario: Progress animates only while its work is unknown
    Given an indeterminate and a determinate progress bar
    When time advances
    Then the indeterminate marker moves frame by frame and the determinate bar never asks for a repaint
