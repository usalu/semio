Feature: IFC4X3_ADD2 structural declarations and validation
  The retained official EXPRESS schema determines entity attributes and value domains.
  WHERE, UNIQUE and inverse rule evaluation are outside structural validation.

  Scenario: Resolve every declaration using an independent IFC implementation
    Given the official IFC4X3_ADD2 catalog
    When every entity and named type is compared with IfcOpenShell
    Then entity inheritance, abstractness, inherited attribute order and optionality agree
    And enumeration and select members agree

  Scenario Outline: Validate a structural snapshot
    Given the IFC4X3_ADD2 snapshot named <case> in schema-cases.json
    When structural validation runs
    Then diagnostic codes equal <codes>
    Examples:
      | case             | codes              |
      | point            |                    |
      | coordinate-count | aggregate-bounds   |
      | coordinate-type  | attribute-type     |
      | reference        |                    |
      | dangling         | dangling-reference |
      | reference-type   | attribute-type     |
      | abstract         | abstract-entity    |
      | unknown          | unknown-entity     |
      | duplicate        | duplicate-id       |

  Scenario: Derived redeclarations retain inherited serialization positions
    Given the declaration IfcSIUnit
    Then the inherited Dimensions attribute is derived
    And its serialized value must be a derived marker

  Scenario: Preserve exact official schema provenance
    Given IFC4X3_ADD2.exp
    Then its SHA-256 equals f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860
