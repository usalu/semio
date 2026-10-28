Feature: EnergyPlus 25.2 IDF interchange
  Scenario: Official EnergyPlus basic building file preserves object semantics
    Given the official EnergyPlus v25.2.0 testfiles/1ZoneUncontrolled.idf
    When positional IDF fields are decoded using the official IDD dictionary
    And the epJSON document is exported to IDF and imported again
    Then every object, named field, numeric value and extensible group is unchanged
    And the building terrain is Suburbs
    And the building north axis is 0 degrees

  Scenario: Positional empty fields survive comments and international names
    Given the IDF records from lexical-vectors.json
    When records are parsed, serialized and parsed again
    Then every positional field equals its expected language-neutral vector

  Scenario: Semantically unrepresentable input is refused
    Given a truncated record, duplicate case-insensitive name or unknown object type
    When IDF interchange is requested
    Then a descriptive refusal is returned
