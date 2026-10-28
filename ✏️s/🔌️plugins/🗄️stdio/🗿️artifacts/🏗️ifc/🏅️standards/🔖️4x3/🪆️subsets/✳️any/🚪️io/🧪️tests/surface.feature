Feature: Current Declared Planar Surface Geometry
  Scenario: Distinct Source And Geometric Identities
    Given the pinned official IFC4x3 surface model and thirty explicit neutral source inputs
    When exact planar faces and their declared groups are evaluated
    Then per-group source SET uniqueness and exact private geometric identity are preserved
    And permitted common faces across distinct groups retain each source occurrence
    And canonical point edits, both inverses, STEP and actual GLB preserve represented geometry

  Scenario: Group Topology Remains Declared
    Given a concrete ConnectedFaceSet, an OpenShell or a ClosedShell
    Then each member is arcwise connected through common vertices or edges
    And OpenShell requires manifold vertex links and a boundary
    And ClosedShell retains outward normal and closed manifold requirements
    And crossing interiors and partial coplanar intersections are refused

  Scenario: Original Output And Caller Authority
    Given the source is independently admitted
    Then item grants 17 and 18 refuse and admit a complete one-face mesh respectively
    And public grants 575 and 576 retain 31 and 32 complete product occurrences respectively
    And actual triangle and GLB routes propagate original error, undefined and abort at initial, interior and completion
    And callbacks cannot replace original controls or alter the private source capture

  Scenario: Shared Represented Frames
    Given millimetre source coordinates and a reflected nonuniform representation mapping
    Then SI geometry agrees with the independent Three transformation
    And central publication changes reflection orientation exactly once
    And geometry without product placement gets an explicit unbound-item diagnostic
