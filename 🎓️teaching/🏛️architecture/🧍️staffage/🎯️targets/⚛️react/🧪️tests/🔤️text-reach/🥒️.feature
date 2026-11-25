Feature: The window around a drawing takes in its texts as the page sets them
  Scenario: A face no wider than the reserved room
    Given a drawing of the reference walker with its caption
    When its texts are measured in a face that is narrower than the room the drawing reserved
    Then no text reaches past its room and the drawing needs no other window than its paper

  Scenario: A wider face
    When its texts are measured in a face in which every character is one em wide
    Then the overhang to the right is the widest start-anchored text minus its reserved room
    And the overhang to the left is half of what the widest centred text takes more

  Scenario: A running time
    Given a face whose digit 1 is narrower than its other digits
    When the caption states the time 1.11 s and then the time 0.80 s
    Then the overhang is the same for both

  Scenario: Nothing to measure with
    When there is no 2D canvas
    Then the extent of the scene is its paper
