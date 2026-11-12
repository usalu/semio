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

  Scenario: A missing or unknown sweep is refused
    Given an extruded solid without sweep, or with the axis "u"
    When it is evaluated, resolved to views, validated or checked against the schema
    Then evaluate and the resolver throw SolidSweepError naming the asset, the shape and the viewpoint, the validator reports solid-sweep and ajv rejects the document
