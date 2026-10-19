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
    Given three windows with the middle window focused
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

  Scenario: Every docking zone previews the committed geometry
    Given a tab group and a held tab
    When the pointer enters a body half or any corner tab strip
    Then body drops split along the dominant axis as in the regular UI
    And corner strip drops join tabs
    And each proposed split rectangle matches the committed layout including its divider

  Scenario: Docking preserves identities and collapses empty groups
    Given three tabs with stable identities and titles
    When their schema-defined docking commands create nested splits and rejoin groups
    Then every tab occurs once and all vacated groups collapse
    And invalid source or target identities leave the layout unchanged
    And nested splits retain the size of their existing sibling slot

  Scenario: Pointer docking preserves window bodies and focus
    Given the real dashboard with a task overview and a launcher
    When a tab is dragged to each edge and back into the other group
    Then a styled preview precedes each release and never mutates the layout
    And the held tab retains its body and receives keyboard focus
    And a live terminal retains its session and output while its child receives the new size

  Scenario: Interrupted docking leaves the layout unchanged
    Given a held tab with a visible split preview
    When Escape, focus loss, terminal resize or an outside release occurs
    Then the preview disappears and no docking command is committed
    And removing the held window cancels its preview even after changing the active tab

  Scenario: Keyboard users can dock, join and reorder tabs
    Given the localized customizable keymap
    When a keyboard user moves a tab to any split edge or joins another group
    Then the same layout commands preserve the tab and its body
    And tab order can be changed without a pointer
    And focus navigation follows the resulting visual order

  Scenario: Tabs dock into every corner with stable identities
    Given a group with occupied and empty corner strips
    When a tab is dragged into any of its four corners or another group's corner
    Then a corner slot preview precedes the release without changing the layout
    And the committed tab retains its body and names the chosen corner
    And a sole tab can change its corner without needing a sibling
    And a layout event during a held drag cannot change which tab is committed

  Scenario: Every tab owns its fullscreen and close controls
    Given an inactive tab beside the active tab
    When its fullscreen control is pressed
    Then that tab becomes active and focused in the fullscreen group
    And selecting another tab keeps the same group fullscreen and shows its body
    When its close control is pressed after restoring the group
    Then only that tab closes and the active tab retains focus
    And fullscreen ends when only one window remains

  Scenario: Keyboard users can place tabs in every corner
    Given an English or German dashboard with customizable corner actions
    When a corner action is invoked after the prefix
    Then the active tab occupies that corner and keeps keyboard focus

  Scenario: Divider dragging continues across layout frames
    Given two tiled windows with a draggable divider
    When the held pointer moves forward and back across several frames
    Then every movement resizes the adjacent groups through the same capture
    And release ends the resize gesture
