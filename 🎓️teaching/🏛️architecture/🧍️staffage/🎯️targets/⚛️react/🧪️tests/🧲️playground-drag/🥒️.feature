Feature: Placements move by keyboard and pointer, and every edit is announced and undoable

  Scenario: An asset is placed from the list
    Given the playground is empty and the catalog has one ready asset
    Then the page says nothing is placed yet
    When the reader presses the asset's place button
    Then one placement exists and a polite status announces it

  Scenario Outline: A focused placement moves by the stated step
    Given a placement at x metres
    When the reader presses <keys>
    Then it moves by <delta> metres and stays between 0 and the ground length

    Examples:
      | keys        | delta |
      | Right       | 0.1   |
      | Shift+Right | 1     |
      | Alt+Left    | -0.01 |
      | Home        | to 0  |

  Scenario: Duplicate, remove and undo
    Given a placement
    When the reader presses D
    Then there are two placements
    When the reader presses Delete on the copy and then chooses Undo
    Then there are two placements again, and Redo makes it one

  Scenario: Dragging snaps to 0.05 m
    Given a placement at 3.00 m and one metre on screen is 40 pixels
    When the pointer moves 21 pixels to the right and is released
    Then the placement is at 3.50 m rounded to the 0.05 m grid, i.e. 3.50 m
    And the figure squashes and hops once unless motion is reduced or playful reactions are off

  Scenario: The composition is kept
    Given placements exist
    When the page is mounted again
    Then the same placements are shown, in the reader's language
