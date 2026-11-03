@capability-shell-hooks @level-unit
# language: en
Feature: A plugin takes part in the shell without the shell knowing its domain

  Scenario: The shell asks only when the device changed
    Given a plugin that chooses "mobile" below 600 pixels and "desktop" above
    When the shell observes 1200 pixels
    Then the cause is "start" and the plugin chooses "desktop"
    When the shell observes 1200 pixels again
    Then nothing is asked
    When the shell observes 390 pixels
    Then the cause is "resize" and the plugin chooses "mobile"
    When the last input becomes touch
    Then the cause is "inputModality"
    When something becomes selected
    Then the cause is "selection"
    When another app opens
    Then the cause is "start" again

  Scenario: The plugin sees the key before the shell
    Given a plugin that consumes "ArrowLeft" in the "viewport" scope and asks for "undo" on ctrl+z
    When the key "ArrowLeft" arrives in "viewport"
    Then the shell does not run its own default
    When ctrl+z arrives
    Then the shell dispatches the app action "undo"
    When an unknown key arrives or the plugin has no hooks
    Then the shell continues with its defaults

  Scenario: The bottom sheet is a container of the UI tree
    Given a bottom sheet labelled "Properties"
    When it is built around a window body
    Then the node is a container of role "sheet" announced as "complementary" by every renderer

  Scenario: Names a plugin gives reach the platform accessibility tree
    Given a plugin node with role "option", name "Beam 1", position 1 of 2 and selected
    When it is projected
    Then the projection node carries the role, the label, the position, the set size and the selection

  Scenario: The touch lane asks the plugin per device and hands it every recognised gesture
    Given a plugin that orbits with one finger below 600 pixels and answers a long press with the action "menu"
    When the shell observes 390 pixels
    Then the touch profile orbits with one finger and asks for a 44 pixel hit edge
    When the shell observes 1200 pixels
    Then the touch profile no longer orbits with one finger
    When a long press is recognised in the "viewport" scope
    Then the plugin answers the action "menu" and a plugin without hooks ignores the gesture
    When a tap lands on the surface of the "viewport" scope
    Then the plugin is asked too, so it can dismiss what a long press opened
