@capability-viz-render-scene-graph
@no-oracle-render-scene
@comparison-viz-probe-exact-v1
Feature: A chart specification renders into the scene graph and into TikZ
  The visualization library owns its TypeScript numerical inference under
  `🧬️schema/💡️inferences`, alongside the native inference contract.
  Its renderer turns a taxonomy §79 chart specification into the
  dependency-free 2D scene graph of `🧰️framework/🔨️modules/◻️2d` and, from the SAME resolved item
  list, into TikZ source text.

  Nothing in d3 adjudicates that stage. d3 has no scene graph, no TikZ emitter and no notion of a
  chart specification — its shape generators stop at a path string. The arithmetic underneath the
  renderer is separately adjudicated against the registered d3 oracles by the kernel's own
  differential harness (`bun ./📜️script.ts test` in `🧬️schema/💡️inferences/📦️packages/🟦️typescript`,
  registered numerical checks), so this case covers only what that harness cannot: that the
  specification is turned into the primitives it names, that both emitters agree on the same item
  list, and that changing an option changes the projection.

  Axis defaults are 1.4 mm ticks and caps, 0.8 mm padding, 5 mm title gap,
  6.6 TeX point labels and 7.2 TeX point titles. Titles follow the domain segment.
  Axis domain lines default to enabled; domainLine overrides domain when both are authored.
  Minor ticks subdivide mapped major positions, default to four subdivisions and 0.7 mm length,
  and never create labels. Explicit options override the typed minor setting.

  The vectors below are the specification. They are written out, not derived, because a derived
  expectation would restate the implementation rather than constrain it.

  @id-scene-graph-primitives
  @level-quick
  @mode-conformance
  Scenario: The scene graph carries exactly the primitives the specification names
    Given the demo bar chart specification and the expected scene
      | key             | values      |
      | size            | 160,100     |
      | count/line      | 18           |
      | count/text      | 12           |
      | count/rect      | 4           |
      | count/path      | 4           |
      | identity        | 1           |
      | unitColors      | 1           |
    Then the rendered scene graph matches those counts

  @id-bar-rectangles
  @level-quick
  @mode-conformance
  Scenario: Every bar rectangle stands where its scales put it
    Given the demo bar chart specification and the expected rectangles
      | key    | values                     |
      | rect/0 | 22.47619,54.8,25.904762,31.2   |
      | rect/1 | 54.857143,8,25.904762,78       |
      | rect/2 | 87.238095,67.8,25.904762,18.2  |
      | rect/3 | 119.619048,28.8,25.904762,57.2 |
    Then the scene's rectangles match those numbers to six decimals

  @id-tikz-mirrors-the-scene
  @level-quick
  @mode-conformance
  Scenario: The TikZ emitter draws the same item list as the scene graph
    Given the demo bar chart specification and the expected picture
      | key         | values |
      | statements  | 34     |
      | opens       | 1      |
      | closes      | 1      |
      | unitless    | 1      |
    Then the emitted TikZ carries one statement per resolved item, in millimetre coordinates

  @id-options-change-the-projection
  @level-quick
  @mode-conformance
  Scenario: Two specifications of one mark with different options render differently
    Given the demo bar chart specification and the expected distinctness
      | key       | values |
      | differs   | 1      |
    Then widening the band padding moves every bar

  @id-customizable-grammar
  @level-quick
  @mode-conformance
  Scenario Outline: The complete grammar resolves its authored shared fixture
    Given the language-neutral chart fixture <id> from customization.json
    When the chart resolves its table transformations and drawing primitives
    Then its coordinates, styles and emitted text match its specified or D3 oracle output
    Examples:
      | id |
      | row-style |
      | empty-area |
      | heatmap-rectangle |
      | vertical-rule |
      | polar-point |
      | top-axis |
      | right-axis |
      | grid-only |
      | localized-annotation |
      | area-boundary |
      | line-curve |
      | link-curve |
      | arc-center |
      | rect-dimensions |
      | horizontal-bar |
      | circle-radius |
      | symbol-shape |
      | band-boundary |
      | polygon-points |
      | path-curve |
      | trail-width |
      | ribbon-geometry |
      | pie-layout |
      | text-style |
      | encoded-color-scale |
      | line-detail-order |
      | row-line-styles |
      | point-gap |
      | line-gap |
      | normalized-cartesian |
      | transpose-cartesian |
      | left-axis |
      | bottom-band-axis |
      | legend-color |
      | annotation-shapes |
      | localized-title |
      | clip-and-dash |
      | normalized-rect |
      | normalized-rule |
      | polar-rect |
      | full-circle-path |
      | geographic-point |
      | axis-domain-default |
      | axis-domainline-off |
      | axis-domain-alias-off |
      | axis-domainline-overrides-alias |
      | axis-domainline-enabled |
      | axis-minor-default |
      | axis-minor-top-custom |
      | axis-minor-left-custom |
      | axis-minor-right-reversed |
      | axis-minor-options-off |
      | axis-minor-options-on |
      | axis-minor-single-major |
      | grid-minor-default |
      | axis-minor-log-position-interpolation |
      | axis-domain-scale-range |
      | axis-domain-reversed-scale-range |
      | grid-domainline-explicitly-enabled |
      | axis-labelrotate-alias |
      | axis-labelrotation-primary |
      | axis-labelrotation-precedence |
      | axis-label-align-start |
      | axis-label-align-end |
      | axis-label-align-middle |
      | axis-tick-padding-bottom |
      | axis-tick-padding-top-zero |
      | axis-tick-padding-left |
      | axis-title-start-gap |
      | axis-title-end-gap |
      | axis-title-middle-gap |
      | axis-ticksize-options-overrides-typed |
      | axis-options-tickvalues |
      | axis-options-tickvalues-order |
      | axis-options-ticks-count |
      | axis-options-tickvalues-empty |
      | axis-options-tickformat |
      | axis-options-tick-controls-combined |
      | axis-options-scale |
      | axis-options-orient |
      | axis-options-offset |
      | axis-options-title |
      | axis-at-bottom |
      | axis-at-left |
      | axis-outer-caps |
      | axis-grid-length |
      | axis-segment |
      | axis-segment-grid-minor |
      | axis-radial-horizontal |
      | axis-radial-vertical |
      | axis-angular |
      | axis-angular-minor |
      | axis-angular-domain |
      | axis-broken-domain |
      | axis-broken-gap-marks |
      | axis-broken-hidden-marks |
      | axis-native-style |
      | axis-native-defaults |
      | axis-native-left-defaults |
      | axis-angular-title-start |
      | axis-angular-title-middle |
      | axis-angular-title-end |
      | axis-empty-title |
      | text-alphabetic-start |
      | text-alphabetic-middle |
      | text-alphabetic-end |
      | text-middle-start |
      | text-middle-middle |
      | text-middle-end |
      | text-top-start |
      | text-top-middle |
      | text-top-end |
      | text-bottom-start |
      | text-bottom-middle |
      | text-bottom-end |
      | axis-explicit-alignment-baseline |
      | text-default-baseline |
      | axis-native-default-scale |
      | axis-native-default-orient |
      | axis-native-boolean-literals |
      | axis-native-domain-literal |

      | axis-native-at-bottom |
      | axis-native-at-top |

      | axis-native-polar-default |
      | axis-native-rotation-positive |
      | axis-native-rotation-negative |
      | axis-native-rotation-alias |

      | axis-native-theme-light |
      | axis-native-theme-dark |
      | axis-native-stroke-overrides |

      | axis-native-grid-stroke-fallback |

  Scenario: Native Legend Foreground Follows Authored Theme and Fill
    Given language-neutral light, dark and explicit fill legend controls
    When canonical inference emits the portable scene and TikZ
    Then title and label paints match native foreground roles and independent D3 color conversion
