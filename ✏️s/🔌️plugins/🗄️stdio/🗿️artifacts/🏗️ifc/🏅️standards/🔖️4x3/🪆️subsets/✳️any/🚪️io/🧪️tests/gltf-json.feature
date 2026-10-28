Feature: Natural glTF JSON interoperability
  Scenario Outline: Read the complete JSON text under glTF encoding rules
    Given the neutral UTF8 prefix and original JSON member spelling
    And one embedded BIN byte with zero padding
    When the natural GLB is read
    Then acceptance equals the neutral case
    And integer properties accept integral decimals or exponents
    And a later duplicate member determines the effective value
    And the supported UTF8 BOM is ignored
