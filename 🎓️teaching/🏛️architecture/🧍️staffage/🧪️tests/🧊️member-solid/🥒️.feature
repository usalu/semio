@capability-staffage-evaluation
Feature: A member is a bar drawn in its true foreshortened outline
  A `member` shape is a prismatic bar between two bone-local end points (each carried by the frame of its bone) with a rectangular or circular section. Its outline in the side, front and plan view is the
  projection of the prism, so a diagonal at any 3D orientation is one foreshortened bar, never a bounding rectangle, and it follows its bones in every pose.

  Scenario: A diagonal bar at an arbitrary orientation
    Given a rectangular bar from (0, 0, 0) to (0.4, 0.6, 0.3) metres on a bone
    When the three views are evaluated
    Then the outline of each view equals the convex hull of the projected end sections recomputed with numpy and shapely

  Scenario: The bar follows its bone when the bone turns
    Given the same bar on a bone turned by 30 degrees and spread by 20 degrees
    Then the outline equals the oracle for the turned frame in all three views

  Scenario: A bar between two bones follows both
    Given a circular bar from the shoulder bone to the hand bone
    Then each end sits at its own bone and the outline equals the oracle

  Scenario: A member takes part in the depth ordering
    Given a member and a rect shape on other bones
    Then the depth of the member is the nearest point of its end sections
