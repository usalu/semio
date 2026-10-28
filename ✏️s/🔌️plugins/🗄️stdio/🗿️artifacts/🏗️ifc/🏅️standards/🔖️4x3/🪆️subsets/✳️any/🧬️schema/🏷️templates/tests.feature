Feature: Released IFC property and quantity templates retain exact authoring domains
  Scenario: Every released template is available in both implementations
    Given the pinned unchanged ADD2 property definition archive
    Then 645 property sets and 115 quantity sets retain 3988 named entries
    And both implementations retain all property forms and quantity measure domains
    And source names, hashes and applicability remain traceable

  Scenario: Predefined type applicability is explicit
    Given the electric actuator template
    When only an actuator entity type is supplied
    Then applicability requires its predefined type
    When ELECTRICACTUATOR is supplied
    Then the template is applicable
    And the pneumatic actuator type is inapplicable

  Scenario: Enumerated and quantity fields retain their actual domains
    Given Pset_WallCommon Status and Qto_WallBaseQuantities NetSideArea
    Then Status retains the named PEnum_ElementStatus and NEW literal
    And NetSideArea retains IfcAreaMeasure
    And kind labels require an explicit English or German locale
