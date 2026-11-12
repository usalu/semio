Feature: Hidden surfaces and hidden lines of the axonometric view are exact vector geometry
  The visible faces are convex polygons cut against every nearer triangle, the visible lines are the
  intervals of each silhouette, crease and interpenetration edge that no nearer triangle covers.

  @capability-axonometric-hidden-surface
  Scenario Outline: A box behind a box shows exactly the hand-computed area
    Given the scene <name> of 🧫️fixtures/🧊️axonometric-visibility
    When the visible fragments of the back face are summed
    Then the area equals the hand-computed value and polygon-clipping's difference of the two outlines, to 1e-9
    And every grid point is owned by the nearest front-facing hit of three's Raycaster, none by two triangles

    Examples:
      | name                |
      | box behind box      |
      | box fully hidden    |
      | boxes side by side  |
      | box behind a corner |

  @capability-axonometric-interpenetration
  Scenario: Interpenetrating bars and a cycle of overlaps need no ordering
    Given two bars that cross each other and three bars that lie over one another in a cycle
    When the owner of every grid point is compared with the ray caster
    Then there is no mismatch and no point is covered twice

  @capability-axonometric-coplanar
  Scenario: Coplanar faces are covered once
    Given two boxes that share a flush front plane
    When the fragments of both fronts are summed
    Then the area is the union of the fronts and no point is owned twice

  @capability-axonometric-hidden-line
  Scenario: A line is hidden exactly where something nearer covers it
    Given a segment behind a box, an edge in front of a box and an edge that passes through a box
    When the hidden intervals are computed
    Then they equal the shadow of the box, are empty, and end where the depth changes sign
    And the ray caster agrees along the whole line

  @capability-axonometric-rasteriser
  Scenario: The visible surfaces of the sample figure equal the kernel viewer's rasteriser
    Given the sample figure in the stand pose at 1:50 and the viewer's isometric camera fitted to it
    When every pixel centre is owned by a surface in the vector drawing and in the software rasteriser
    Then at least 99 % of the covered pixels agree
