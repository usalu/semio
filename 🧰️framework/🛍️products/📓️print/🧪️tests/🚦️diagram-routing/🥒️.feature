@capability-diagram-edge-routing
@no-oracle-diagram-edge-routing
@comparison-viz-probe-v1
Feature: The diagram kernel routes an edge between two node rectangles
  `semio-viz-diagram-flowchart.sty`'s `🔖️Routing` region turns a source/target pair into an ordered
  waypoint list, and every diagram family draws that list. Two routings are specified here.

  `routing=orthogonal` leaves the node through the face that faces the target. When the two centres
  differ vertically the edge leaves the bottom (or top) face, runs to the vertical midpoint between
  the two faces, steps sideways and enters the opposite face — four waypoints, collapsed to two when
  the two centres share an x. When the centres share a y the edge leaves the left or right face and
  enters the opposite one, two waypoints.

  `routing=straight` intersects the centre-to-centre ray with each rectangle: with a half-extent
  (hw, hh) and a direction (dx, dy) the port is the centre plus `min(hw/|dx|, hh/|dy|)·(dx, dy)`,
  so the endpoint slides along whichever face the ray actually crosses.

  **Why there is no third-party oracle.** Waypoint routing is a drawing decision, not a computed
  layout: dagre and elkjs emit control points from their own splines and edge-separation model and
  would disagree with any other implementation by construction, so neither adjudicates this. The
  rules above are complete, so each scenario writes out the waypoints they produce. The probe emits
  `geometry/diagram-route` records of `count, x1, y1, …` in edge-table order.

  @id-orthogonal-vertical
  @level-quick
  @mode-conformance
  Scenario: A vertical edge leaves and enters through the horizontal faces
    Given the diagram edges
      | from | to | count | waypoints                                    |
      | a    | b  | 2     | 38,39.3333,42,39.3333                        |
      | b    | c  | 2     | 59.5,33.6666,59.5,29.6666                    |
      | c    | d  | 4     | 59.5,18.3334,59.5,16.3334,20.5,16.3334,20.5,14.3334 |
      | a    | d  | 2     | 20.5,33.6666,20.5,14.3334                    |
    Then every routed edge matches the specified waypoints

  @id-straight-ports
  @level-quick
  @mode-conformance
  Scenario: A straight edge meets each rectangle where the centre ray crosses it
    Given the diagram edges
      | from | to | count | waypoints                        |
      | a    | b  | 2     | 38,39.3333,42,39.3333            |
      | b    | c  | 2     | 59.5,33.6666,59.5,29.6666        |
      | c    | d  | 2     | 45.087,18.3334,34.913,14.3334    |
      | a    | d  | 2     | 20.5,33.6666,20.5,14.3334        |
    Then every routed edge matches the specified waypoints
