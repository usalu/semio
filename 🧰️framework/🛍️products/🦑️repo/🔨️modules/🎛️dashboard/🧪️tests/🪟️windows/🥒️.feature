Feature: Windows hold the tasks of the dashboard
  Every task has a window in a stack of tabs. Tabs and titles come from the task label, the terminal of
  a window is exactly as large as its widget, and the engine's focus is the only owner of the keyboard.

  Scenario: Tabs and titles come from the task label
    Given tasks with labels, a status and duplicate texts
    When their windows are shown
    Then each tab carries the status as a glyph and the text of its label
    And duplicates are told apart

  Scenario: Terminals follow the rectangle of their widget
    Given a task window
    When the layout changes by a split, a zoom, a closed tab or a resized terminal
    Then the terminal is exactly as large as its widget
    And only sizes that changed are sent to the daemon

  Scenario: Clicking a window moves the keyboard focus there
    Given two windows
    When the developer clicks the second
    Then the keyboard goes to the second window

  Scenario: Prefix actions split, zoom, cycle and close windows
    Given the dashboard with one window
    When the developer presses the prefix key followed by the keys of the window actions
    Then the windows split, zoom, cycle and close

  Scenario: Closing a window focuses its neighbour
    Given three windows
    When the developer closes the middle one
    Then the previous window takes the focus
    And typing quit never quits

  Scenario: The right button opens a context menu whose rows act on the window
    Given a task window
    When the developer presses the right mouse button on it
    Then a menu lists the actions with the keys of the keymap
    And choosing a row performs the action on that window

  Scenario: Window controls are named in the language of the dashboard
    Given the preferences language en and language de
    When the window controls are shown
    Then their names are those of the language
    And an unchanged frame writes nothing

  Scenario: A compound opens one tab per member
    Given a compound of two members
    When it starts
    Then each member has its own tab with its own task label

  Scenario: An unchanged screen is neither dirty nor repainted
    Given a screen that was rendered
    When nothing changes and when the developer types
    Then nothing is repainted without a change and the repaint follows what is typed
