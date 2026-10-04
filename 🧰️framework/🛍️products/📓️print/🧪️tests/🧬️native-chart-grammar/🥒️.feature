Feature: Native inferred chart grammar
  Scenario: All canonical marks consume authored scales and row channels
    Given the language-neutral native chart grammar vectors
    When native chart inference emits its TikZ and pinned Tectonic compiles the source
    Then every declared mark emits nonempty geometry
    And authored positions match independent D3 scales
    And ordinal fill, row opacity, stroke width and rotation reach painted geometry
  Scenario: Detail and order define independent paths
    Given two interleaved detail groups with descending source order
    When the native plot is compiled
    Then each ordered group emits its own path
  Scenario: Continuous color scales interpolate named colors
    Given a continuous named-color scale
    When a midpoint is encoded as fill
    Then the painted color equals the independent D3 RGB midpoint
  Scenario: Every coordinate system consumes the same authored encodings
    Given the neutral coordinate frame, scale ranges, options and row
    When all seven coordinate systems are compiled with pinned Tectonic
    Then their geometry matches independent D3 scales, radial shapes and geographic projections
