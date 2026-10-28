Feature: Effective derived SELF attributes and local bindings
  Scenario: Inherited WHERE reads effective derived attributes by unqualified name
    Given a structurally valid SubContext inheriting its parent's TrueNorth
    When the parent direction has three components
    Then North2D fails for both the parent and the SubContext
    And a two-component parent direction passes both rules
    And full independent IfcOpenShell validation yields the same failed source identities

  Scenario: SELF resolution preserves explicit qualification and local shadows
    Given an admitted entity with an effective derived attribute
    Then its unqualified explicit SELF and qualified group reads agree
    And local parameter and query bindings shadow that unqualified name
    And explicit SELF reads still resolve the entity attribute

  Scenario: Foreign SELF is refused before inspecting an arena
    Given a SELF entity handle from another prepared native context
    When its arena index is absent or coincides with a local entity
    Then identifier evaluation returns an owned entity-domain error
    And the receiving context remains valid for its own handles
