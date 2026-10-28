Feature: Declared Positive-Axis CSG Block Geometry
  Scenario: Original And Neutral Physical Blocks
    Given the pinned IFC4x3 ADD2 official csg-primitive file and eleven explicit neutral records
    When a required block Position, represented frames and selected SI units are applied
    Then each supported source emits eight vertices and twelve outward triangles
    And canonical dimension edits, both inverses, STEP and actual GLB exchange retain geometry

  Scenario: Finite Original Output Grant
    Given the source graph is admitted independently of its geometry output grant
    When the item grant is 59 scalars
    Then no partial block is returned
    When the item grant is 60 scalars
    Then the complete block is returned
    And public grants 179 and 180 over three item references publish two and three complete blocks respectively

  Scenario Outline: Original Caller Authority
    Given the actual triangles or composed GLB route
    When the original observer errors, throws undefined or aborts at initial, interior or completion CSG work
    Then its first failure propagates and replacement controls are never adopted
    And the original source remains unchanged

  Scenario: Numeric Realization Refusals
    Given a subnormal SI dimension, an overflowing SI dimension or distinct corners collapsed by world-coordinate rounding
    When geometry is requested
    Then the geometry conversion reports an explicit error without publishing the item
