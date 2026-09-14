@capability-viz-render-determinism
@no-oracle-viz-typeset-appearance
@comparison-viz-probe-exact-v1
Feature: Every catalogue kind renders, in both themes and both languages
  This is the exhaustive level. It fixes what the previous library never checked: that a chart kind
  is actually drawn, that it is drawn in each theme and each language the product ships, that its
  page is reproducible, and that two kinds of one family with different options are not the same
  picture with a different caption.

  No third-party library renders a semio LaTeX document, so nothing can adjudicate appearance; the
  recorded decision `viz-typeset-appearance` names what stands in its place. The evidence is
  committed as JSON under `local://🖼️gallery-render.json` and is REGENERATED, never hand-written:
  `bun ./📜️script.ts test viz fixtures` in `📦️packages/🟦️typescript` rebuilds the whole matrix,
  extracts each variant's per-kind page text with pdfjs-dist and records its rebuild-stable hash.

  The matrix is every generated gallery section x {light, dark} x {en, de}. A section's document is
  derived by rewriting its `\documentclass` options, so no default language exists anywhere in the
  code: each variant names its own.

  @id-theme-and-language-matrix
  @level-exhaustive
  @mode-conformance
  Scenario: Every variant matches its committed evidence
    Given the committed gallery evidence local://🖼️gallery-render.json
    Then every catalogue kind appears on a page of its section in both themes and both languages
    And every variant's page count, per-kind page text and stable hash equal the committed evidence

  @id-family-option-distinctness
  @level-fundamental
  @mode-property
  Scenario: No two kinds of one family carry identical options
    Given the chart-kind catalogue
    Then no family registers two kinds with the same option string
