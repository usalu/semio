@capability-staffage-depth-order
Feature: A part that moves behind another is hidden by it in every view
  The authored paint order (z, then authored position) is the truth at rest. In a pose or at a clip instant two shapes of different bones are painted the other
  way round only when their physical depth along the view direction reverses against the rest pose by more than 2 mm: nearer is higher in the plan, further
  forward in the front, and on the right of the asset (smaller lateral z) in the elevation. Shapes of one bone, ground shapes and pairs whose order did not
  reverse keep their authored order, so a static drawing is unchanged.

  THE REFERENCE is shapely: it places the rectangle profiles by the hand-computed world angle of their bones, reads the depth and rebuilds the paint order with
  its own topological sort.

  Scenario: At rest the authored order stands in every view
    Given a riser on the left thigh and a level part on the pelvis, the riser authored first
    When the rest pose is prepared in the elevation, the front and the plan
    Then the paint order is ground, riser, level in every view

  Scenario: A rising thigh paints over the level part
    Given the same shapes in the stride pose, where the riser top is 0.018730 m and its foremost point 0.214932 m
    When the front and the plan are prepared
    Then the level part is painted before the riser, and the elevation keeps its order

  Scenario: A spread thigh comes towards the viewer of the elevation
    Given the left thigh abducted by -30 degrees so that its tip is 0.225 m to the right
    When the elevation is prepared
    Then the riser is painted over the level part

  Scenario: A third-party library derives the same paint orders
    Given the hand-computed world angles of the bones
    When shapely places the profiles and sorts them
    Then its paint orders equal those of the preparation
