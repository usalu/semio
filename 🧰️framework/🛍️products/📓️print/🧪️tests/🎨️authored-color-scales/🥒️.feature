@domain-print @case-authored-color-scales @oracle-d3-scale @oracle-d3-interpolate
Feature: Authored Continuous and Color Scale Inference

  @id-color-scale-contract @mode-differential @level-quick
  Scenario: Authored scale kinds and controls
    Given the neutral numeric and color range specifications
    When chart inference resolves their ranges
    Then the outputs equal independent D3 scales

  Scenario: Preset paints retain CSS color and precise alpha
    Given the language-neutral presetPaints vectors
    When chart inference emits each preset paint definition
    Then the resolved RGB and registered alpha equal independent d3-color

  Scenario: Authored UTC intervals control ticks
    Given the language-neutral controls vectors
    When each authored calendar interval generates ticks
    Then day, week, month, year and reversed automatic intervals equal d3-time and d3-scale

  @id-scale-unknown-contract @mode-differential @level-quick
  Scenario: Missing scale inputs honor authored fallbacks
    Given the language-neutral unknown input vectors for every supported scale family
    When null, missing, NaN, invalid strings, booleans and empty strings are mapped
    Then numeric and color fallbacks equal independent D3 scales
    And band and point reject unknown configuration through both owned and AJV admission

  @id-missing-render-contract @mode-differential @level-quick
  Scenario: Missing coordinates create gaps and nullable paints are omitted
    Given the language-neutral missing rows and nullable paint fallback cases
    When the canonical asynchronous worker resolves the chart
    Then points and line gaps equal independent D3 scales and shapes
    And absent and null fill or stroke are omitted from the admitted output

  @id-nice-scale-contract @mode-differential @level-quick
  Scenario: Derived nice scales retain authored tick controls
    Given the neutral nice vectors for numeric and color continuous scale kinds
    When the canonical scale is derived through nice twice
    Then domain and range equal independent D3 nice arithmetic
    And explicit tick values, authored counts, formats and unknown fallback survive

