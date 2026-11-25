@capability-staffage-evaluation
Feature: The sweep of an extruded solid is required and never defaulted
  An extruded solid sweeps its profile along the bone-local axis `sweep`: `w` (lateral) puts the profile in the (u, v) plane of the bone as every elevation shape is authored, `v` (the bone normal) puts it in the (u, w) plane. A missing sweep used to be drawn like `v`, so the front and plan views of every part without the member were transposed. There is no default any more:
  `evaluate` and the multiview resolver throw `SolidSweepError`, the semantic validator reports `solid-sweep`, and the JSON Schema requires the member.

  THE REFERENCE is gl-matrix, which carries the swept profile through the hand-written basis of the thigh bone, and ajv for the schema.

  Scenario: Sweep w
    Given a 0.45 m by 0.05 m profile on a hanging thigh swept laterally from -0.2 m to 0.3 m
    When the front and plan views are evaluated
    Then the front is 0.5 m wide by 0.45 m high and the plan 0.05 m long by 0.5 m wide

  Scenario: Sweep v
    Given the same profile swept along the bone normal
    When the front and plan views are evaluated
    Then the front is 0.05 m wide by 0.45 m high and the plan 0.5 m long by 0.05 m wide

  Scenario: The lateral span of sweep w is a rest position of the asset
    Given the same profile on a thigh that the rig places 0.1 m left of the centre plane
    When it is swept laterally from -0.2 m to 0.3 m and the front and plan views are evaluated
    Then the part still spans -0.2 m to 0.3 m of the asset, and the same profile swept along the bone normal sits 0.1 m to the left with its bone

  Scenario: The paint depth of a swept solid follows its sweep
    Given a profile on a hanging thigh swept along u, along v and along w
    When the depth that ranks the shape in the front and in the plan is read
    Then it is the nearest point of the profile placed in the plane of its sweep at the middle of its span, as gl-matrix carries it

  Scenario: A missing or unknown sweep is refused
    Given an extruded solid without sweep, or with the axis "u"
    When it is evaluated, resolved to views, validated or checked against the schema
    Then evaluate and the resolver throw SolidSweepError naming the asset, the shape and the viewpoint, the validator reports solid-sweep and ajv rejects the document

  Scenario: A transverse channel profile is swept along the bone
    Given a transverse profile in the local v-w plane
    When the part is swept along u and viewed from front and top
    Then both silhouettes agree with the declared dimensions and gl-matrix frame transformations
