Feature: Supported spatial occurrence property interchange
  Scenario: Exact own typed values survive public exchange and STEP transport
    Given a supported Site, Building or Storey with authored dimensionless properties
    When the public BIM export and IFC import routes execute
    Then exact names and Text, Real, Integer and Boolean values survive
    And ambiguous names, owners, units and unsupported values refuse explicitly
  Scenario: Existing IFC canonical edits remain authoritative
    Given an exported occurrence property nominal value
    When a canonical SetEntityArg diff edits it
    Then import observes only that change
    And the canonical inverse restores the original graph
