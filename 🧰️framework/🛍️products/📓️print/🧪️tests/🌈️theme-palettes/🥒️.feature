@capability-viz-theme-palettes
@oracle-d3-scale
@oracle-d3-interpolate
@oracle-d3-color
@comparison-viz-probe-exact-v1
Feature: The theme kernel answers every colour question the library is allowed to ask
  `semio-viz-theme` is the only place a renderer may name a colour. Every family reaches it for the
  hue of a series, the hatch that replaces that hue in a grayscale figure, and the ramp a value maps
  onto, so a defect here is a defect in every figure at once — and none of the family cases would
  see it, because they all read the same wrong answer.

  Three of the four questions have a real third-party reference. The **wrap** is `d3-scale`'s
  `scaleOrdinal`: a series index beyond the end of the palette must come back to its first colour,
  and it must do so on the same modulus, which is what separates a palette of twelve presence slots
  from one of seven brand tokens and from the seven grays the grayscale theme substitutes. The
  **ramp** is `d3-interpolate`'s `piecewise` over the same stop list, evaluated in `d3-color`'s CIE
  Lab and HCL: `semio-viz-scale` converts between sRGB, Lab on the D50 white point and HCL with the
  shortest hue path exactly as `d3-color` does, so the comparison is on the rendered hexadecimal,
  character by character, and not on a tolerance. The **stops** are adjudicated by `d3-color`
  parsing each declared stop and formatting it back, which is what proves the tables in
  `semio-tokens.sty` are sRGB and not typos.

  The fourth question has no third-party reference at all, and it is the theme's own default:
  `interpolator=oklab`. d3 has no OKLab, so nothing can adjudicate the colours between the stops.
  What can be adjudicated is the law every interpolation space owes its stop list — that a ramp of
  five stops returns those five colours unchanged at 0, ¼, ½, ¾ and 1 — and `d3-color` is the
  reference for what "unchanged" means, because it is what normalises the declared stop into the
  hexadecimal the probe must produce. A space that drifts at its own knots is broken whatever it
  does between them; a space that does not is measured on the Lab and HCL scenarios instead.

  Colours and colour names travel through the probe as strings prefixed with a vertical bar, so no
  hexadecimal digit is reclassified as a number, and the comparison profile is the exact one.

  @id-categorical-wrap
  @level-quick
  @mode-differential
  Scenario: A series index wraps onto the palette exactly as d3-scale's ordinal scale does
    Given the committed probe document shared://🌈️theme-palettes/theme-palettes.tex and the categorical palettes
      | palette  | colours                                                                                                                                                                                                                        |
      | presence | semio-presence-light-0;semio-presence-light-1;semio-presence-light-2;semio-presence-light-3;semio-presence-light-4;semio-presence-light-5;semio-presence-light-6;semio-presence-light-7;semio-presence-light-8;semio-presence-light-9;semio-presence-light-10;semio-presence-light-11 |
      | brand    | semio-primary;semio-secondary;semio-tertiary;semio-info;semio-success;semio-warning;semio-danger                                                                                                                                |
      | gray     | semio-dark;semio-gray-300;semio-gray;semio-gray-600;semio-gray-800;semio-gray-200;semio-light-gray                                                                                                                              |
    Then the compiled probe and d3-scale's ordinal scale name the same colour for every slot
    And the theme reports the same number of distinct slots as the palette has colours

  @id-pattern-wrap
  @level-quick
  @mode-differential
  Scenario: A series index wraps onto the hatch set exactly as d3-scale's ordinal scale does
    Given the committed probe document shared://🌈️theme-palettes/theme-palettes.tex and the hatch set
      | patterns                                                                                              |
      | north east lines;horizontal lines;vertical lines;north west lines;crosshatch;dots;grid;crosshatch dots |
    Then the compiled probe and d3-scale's ordinal scale name the same hatch for every slot

  @id-scheme-stops
  @level-quick
  @mode-differential
  Scenario: Every declared scheme stop is an sRGB colour d3-color reads back unchanged
    Given the committed probe document shared://🌈️theme-palettes/theme-palettes.tex and the scheme stop lists
      | scheme          | light                                | dark                                 |
      | primary         | f7f3e3;ff344f;6b2c32                 | 001117;ff344f;ffa9a0                 |
      | grays           | f7f3e3;7b827d;001117                 | 001117;7b827d;f7f3e3                 |
      | viridis-like    | 051a47;1a1a89;1a8989;1a891a;89891a   | a8b1ea;5858e4;58e4e4;58e458;e4e458   |
      | danger-success  | a60009;f7f3e3;7eb77f                 | a60009;001117;7eb77f                 |
    Then the compiled probe and d3-color agree on every stop of both appearances

  @id-scheme-ramp-lab
  @level-quick
  @mode-differential
  Scenario: The CIE Lab ramp of a scheme agrees with d3-interpolate's piecewise interpolation
    Given the committed probe document shared://🌈️theme-palettes/theme-palettes.tex and the sampled ramps
      | scheme         | stops                              | positions              |
      | viridis-like   | 051a47;1a1a89;1a8989;1a891a;89891a | 0;0.125;0.5;0.8;1      |
      | danger-success | a60009;f7f3e3;7eb77f               | 0.25;0.75              |
    Then the compiled probe and d3-interpolate agree on every sample

  @id-scheme-ramp-hcl
  @level-quick
  @mode-differential
  Scenario: The HCL ramp of a scheme agrees with d3-interpolate's piecewise interpolation
    Given the committed probe document shared://🌈️theme-palettes/theme-palettes.tex and the sampled ramps
      | scheme  | stops                | positions          |
      | primary | f7f3e3;ff344f;6b2c32 | 0;0.25;0.5;0.75;1  |
      | grays   | f7f3e3;7b827d;001117 | 0.375;0.625        |
    Then the compiled probe and d3-interpolate agree on every sample

  @id-scheme-ramp-oklab
  @level-quick
  @mode-differential
  Scenario: The default OKLab ramp returns its own stops unchanged at the stop positions
    Given the committed probe document shared://🌈️theme-palettes/theme-palettes.tex and the sampled ramps
      | scheme       | stops                              | positions             |
      | viridis-like | 051a47;1a1a89;1a8989;1a891a;89891a | 0;0.25;0.5;0.75;1     |
    Then the compiled probe and d3-color agree on every stop the ramp lands on
