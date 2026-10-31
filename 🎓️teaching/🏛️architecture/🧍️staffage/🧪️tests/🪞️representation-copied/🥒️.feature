@capability-staffage-representation
Feature: Every scale is redrawn, never copied
  A representation authored for one scale must not be another scale's drawing reused. Validation flags `representation-copied` when two authored
  representations have identical shapes, when every shape of the coarser one occurs identically in the finer one (a finer drawing with shapes deleted), or when one
  equals the other multiplied by a common factor. It also flags overlapping valid ranges and pens or hatches that are not plausible paper millimetres.

  Scenario: The fixture figure is clean
    Given a figure whose 1:200, 1:100 and 1:50 drawings are separately authored
    When it is validated
    Then no copy, overlap or pen-unit issue is reported

  Scenario: An identical copy
    Given the 1:100 drawing replaced by the shapes of the 1:50 drawing
    When it is validated
    Then representation-copied names 1:100

  Scenario: A drawing with shapes deleted
    Given the 1:100 drawing replaced by the first six shapes of the 1:50 drawing
    When it is validated
    Then representation-copied names 1:100

  Scenario: A scaled copy
    Given the 1:200 drawing replaced by the 1:100 drawing multiplied by 2
    When it is validated
    Then representation-copied names 1:200

  Scenario: Overlapping ranges and pens in model metres
    Given the valid range of 1:100 stretched to 60…200 and the body pen set to 0.0003
    When it is validated
    Then valid-range-overlap and pen-units are reported
