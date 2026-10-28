Feature: Preserve original cancellation for IFC artifact exchange
  Scenario Outline: Caller options change during the first progress callback
    Given a supported source for <endpoint>
    And original cancellation and progress controls
    When the first callback replaces its options with a fresh signal and callback
    And the original signal is aborted
    Then the exchange refuses with io.cancelled
    And the original callback ran exactly once
    And the replacement callback did not run
    And no partial artifact is published

    Examples:
      | endpoint      |
      | bim-to-ifc    |
      | ifc-to-bim    |
      | fem-to-ifc    |
      | ifc-to-fem    |
      | ifc-triangles |
