@capability-staffage-representation
Feature: The drawings of an asset cover every scale they promise
  Each preset scale stands for a band of denominators (1:50 for 1:20…1:75, 1:100 for 1:75…1:150, 1:200 for 1:150…1:350, 1:500 for 1:350…1:1000), so the product
  promises 1:20…1:1000 for an asset drawn at every preset. The valid ranges of the authored representations must cover the union of the bands of the authored presets
  without a hole (rule `valid-range-gap`): the boundary between two authored neighbours may move, the outer ends may not shrink, and an explicit `unavailable` decision
  promises nothing. A band that fails print suitability at its end is redrawn, never narrowed into a hole.

  Scenario: Promised bands, moved boundaries, overlaps and unavailable presets
    Given the vectors of every case
    When the holes are computed
    Then they equal the listed holes, filed under the listed border preset

  Scenario: The validator files one finding per hole
    Given the sample figure with the ranges of every case
    When it is validated
    Then valid-range-gap is reported once per hole at the validRange of the border preset

  Scenario: Independent oracle
    Given the same vectors
    When shapely subtracts the union of the valid ranges from the union of the promised bands on the denominator axis
    Then it finds the same holes

  Scenario: Schema and implementation agree on the bands
    Given the ScaleBands of the JSON Schema
    Then they equal the bands of the implementation and of the vectors, and tile 1:20…1:1000 without a hole

  Scenario: The catalog has no hole
    Given every catalog asset
    When its holes are computed
    Then there are none
