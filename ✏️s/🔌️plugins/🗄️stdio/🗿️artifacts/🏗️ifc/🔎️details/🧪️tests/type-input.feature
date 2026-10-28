Feature: Caller-authored required reference values during retained type remapping
  Scenario: Profile is supplied through the source-bound guided control
    Given an original complete complex IfcMaterialConstituent and declared IfcProfileDef entity
    And a type request for IfcMaterialProfile under the original canonical revision
    Then the missing required Profile control has English and German labels and no default ID
    When the caller supplies the exact positive decimal reference through its original binding
    Then the real central type edit preserves compatible original values
    And the supplied value crosses the original decimal receiver
    And strict target DomainWork checks the complete target tuple
    And progressive central inverse restores the original exact source
  Scenario: Inputs cannot grant an alternative type policy
    Then compatible-slot overrides, optional slots, duplicate indices and extra rows refuse
    And absent, wrong-kind, missing or ambiguous targets never receive invented IDs
    And denied original axes and cancelled input prefixes retain every owner for paid close
    And partial input, candidate and scaffold states cannot publish a mutation batch
