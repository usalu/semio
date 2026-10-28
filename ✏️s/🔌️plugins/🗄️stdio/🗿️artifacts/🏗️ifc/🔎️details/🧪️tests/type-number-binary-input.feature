Feature: Caller-owned exact numeric and binary required inputs retain central authority
  Scenario: REAL words remain exact through the original typed input and central inverse
    Given a required IfcParameterValue without an implicit value
    When the caller enters a canonical finite binary64 word through the resident control
    Then signed zero, subnormal and maximum finite words retain every bit
    And NaN, infinity, malformed and overflowing words stay incomplete and cannot execute
  Scenario: NUMBER has an explicit caller-selected representation
    Given a required IfcNumericMeasure and no supplied row
    Then neither integer nor real is selected by default
    When the caller selects integer or real and fills the matching exact typed control
    Then the complete payload retains its signed integer or binary64 representation
    And changing representation is an explicit ephemeral edit preserving other caller rows
  Scenario: BINARY keeps the Part21 encoding and actual candidate rules
    Given a required IfcBlobTexture RasterCode and original PNG RasterFormat
    When the caller supplies an exact canonical byte-aligned encoded bit sequence
    Then the value remains binary and retains all padding and leading data zeros
    And canonical empty bits differ from unset or absent input
    And malformed or non-byte-aligned BlobTexture candidates cannot execute
  Scenario: Actual named numeric WHERE rules remain strict
    Given IfcColourRgb Red has the released IfcNormalisedRatioMeasure domain
    When the caller supplies one half or a value above one
    Then the complete candidate admits one half and refuses the out-of-range value
    And an unevaluated rule cannot be replaced with primitive-only readiness
