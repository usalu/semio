Feature: Admit a private IFC graph before artifact projection
  Scenario Outline: Semantic callbacks mutate the caller's original graph
    Given a valid source for <endpoint>
    When the source is captured through central IFC graph admission
    And its first semantic progress callback clears the caller's entities and schema header
    Then the projection equals the projection of the unchanged source
    And the private admitted source is preserved

    Examples:
      | endpoint      |
      | ifc-to-bim    |
      | ifc-to-fem    |
      | ifc-triangles |

  Scenario: Source admission fails or is cancelled
    Given duplicate identities, an accessor, exceeded graph limits or original cancellation
    When an IFC source enters a projection
    Then no projection artifact is published
    And input accessors are never invoked
    And cancellation preserves its original terminal kind
