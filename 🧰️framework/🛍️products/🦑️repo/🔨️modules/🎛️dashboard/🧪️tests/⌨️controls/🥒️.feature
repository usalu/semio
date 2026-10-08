Feature: Native Dashboard Keyboard Controls
  Scenario: A portable control key opens optional settings
    Given the shared native keyboard input vectors
    When the terminal receives Ctrl B followed by p
    Then Ctrl B opens the controls and p opens optional settings
    And the input projection matches Node readline independently

  Scenario: Unicode input survives terminal decoding
    Given the same shared vectors contain UTF-8 Ü
    When Rust and Node independently decode the input
    Then both project the same character without a control modifier

  Scenario: Windows owns UTF-8 code pages only while attached
    Given the native terminal acquires UTF-8 input and output code pages
    When cleanup restores one page and temporarily fails to restore the other
    Then only the remaining owned page is retried
    And an actual native view renders box drawing and German labels correctly

  Scenario: A running terminal keeps every key except the prefix
    Given a focused task window that forwards keys to its program
    When the developer presses Ctrl W, Tab, Esc, q and Enter
    Then each key reaches the program as the bytes the program expects
    And Ctrl B twice sends Ctrl B itself while Ctrl B followed by x closes the window

  Scenario: Every action is reachable by keyboard and listed in the language of the developer
    Given English and German as preferences
    When the footer, the keyboard help and the usage text are generated
    Then they are generated from the keymap and the text catalogue without a duplicated hint string
    And every action id of the keymap has a text in both languages

  Scenario: Searching, configuring and starting a command is one keyboard journey
    Given a workspace registry with verbs, projects, parameters, a required service, a compound and a command that changes the repository
    When the developer types words, opens a command, chooses values, adds extra arguments and environment and starts it
    Then the view sends the resolved registry launch with its task label and the chosen values
    And a command that changes the repository asks for confirmation first
    And a required service and the members of a compound are shown before anything starts

  Scenario: Terminals follow their windows
    Given a task window and a split, a zoom, a closed tab and a resized terminal
    When the layout changes
    Then every terminal is exactly as large as the rectangle of its widget
    And the daemon is told only the sizes that changed

  Scenario: The right button offers the actions of the window under it
    Given a task window
    When the developer presses the right mouse button on its terminal
    Then a menu lists the actions with the keys the keymap states for them
    And choosing a row performs the action on that window
