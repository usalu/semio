@capability-staffage-lateral-rest
Feature: The interactive 3D model carries a bone that rests laterally
  A clavicle runs sideways at rest and the arm hangs from its tip. The 3D stage builds its model through the same pipeline as every other consumer
  of the bone frames, so the clavicle solid and the arm must sit where the hand computation of the shared vectors puts them, in a pose and through a clip.

  THE REFERENCE is the hand computation of `🧫️fixtures/🦴️lateral-rest` (`consumers`), which scipy, numpy and shapely confirm in the library test
  `🧪️tests/🦴️lateral-rest`.

  Scenario: The prepared model of the stage holds the lifted clavicle at its place
    Given the sample figure with the shoulder girdle and a capsule on the left clavicle
    When the stage prepares the model of the pose with both shoulders lifted by 30 degrees at 1:50
    Then the tessellated clavicle solid of the body scene lies in the hand-computed box and the arm node sits on the clavicle tip
    And the clip that lifts the shoulders passes the hand-computed tips at a quarter and at half a second

  Scenario: The stage shows it
    When the stage is mounted with that pose and then with that clip
    Then it becomes ready both times and reports no error
