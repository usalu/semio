Feature: The custom element needs no React
  Scenario: Rendering
    Given the element staffage-figure with asset, view, scale, lang and pose attributes
    When it is connected to a document
    Then it contains an inline SVG at paper size in a focusable role img with a summary label
    And it dispatches staffage-evaluated
  Scenario: Reactivity
    When the scale or lang attribute changes
    Then the drawing and its label are re-rendered in the new scale and language
  Scenario: No default language
    Given neither the element nor an ancestor names a language
    Then nothing is rendered
