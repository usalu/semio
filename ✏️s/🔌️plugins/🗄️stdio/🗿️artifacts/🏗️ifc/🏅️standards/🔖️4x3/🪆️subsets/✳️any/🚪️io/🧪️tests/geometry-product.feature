Feature: Extract only declared IFC product representations
  Scenario: A non-product has a shape-looking seventh argument
    Given the exact original official triangulated IFC fixture
    And its represented product is replaced in memory by a non-product declaration
    When represented triangles are extracted
    Then no mesh is attributed to the non-product
    And its unbound tessellation is reported

  Scenario: Every real product family remains eligible
    Given a represented entity with a released IfcProduct subtype
    When represented triangles are extracted
    Then the entity is eligible independently of MVD scope badges
