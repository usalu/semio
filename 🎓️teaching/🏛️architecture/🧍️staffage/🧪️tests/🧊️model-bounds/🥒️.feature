@capability-staffage-model-bounds
Feature: The 3D model has the extent of the asset and of its 2D views
  The volume solids of an asset fill the physical envelope its `dimensions` state: at rest the extent ahead is `widthM`, up `heightM`, lateral `depthM`, and in every named pose the extent of the
  posed, skinned model equals the extent of the elevation, front and plan shapes of the same evaluated state. Tolerance is the catalog's D18: the larger of 2 % and 15 mm.

  THE REFERENCES are three.js (the precise skinned extent of the GLB loaded by GLTFLoader) and the 2D shapes of `evaluate`.

  Scenario: At rest the model fills the dimensions
    Given the sample figure
    When the solids are tessellated and carried by the rest placements
    Then the extent ahead, up and lateral equals widthM, heightM and depthM within the tolerance

  Scenario: In every pose the model has the extent of the 2D views
    Given each named pose of the sample figure exported as a GLB and loaded by three.js
    When its precise extent is compared with the elevation, front and plan shapes of the same state
    Then the ahead extent equals the elevation and plan, the up extent the elevation and the lateral extent the front and plan, within the tolerance

  Scenario: A standing pose stands on the ground
    Given the standing pose
    When its GLB is loaded
    Then its lowest point is the ground within 15 mm

  Scenario: Four rubber feet carry the laptop at every digital scale
    Given the catalog laptop on a level desk
    When the body, thermal surfaces and load-bearing solids meet the desk plane
    Then each contact is a disk 10.4 mm across on a 178 by 268 mm rectangular pattern
    And the underside of the chassis is 2 mm above the desk
    And each sole has a corresponding structural contact and contributes its disk area to thermal contact in each pose

  Scenario: The vacuum and trolley roll on separate left and right tyres
    Given the catalog canister vacuum at all four scales and the shopping trolley at 1:500
    When the body, thermal and structural meshes meet the floor
    Then each axle has distinct left and right tread contacts inside the physical wheel widths
    And no central drum or contact plate bridges either axle
    And a load-bearing rim connects each vacuum rear tread to its axle

  Scenario: The side-entry street lamp retains its arm bearing and service closures
    Given the catalog street lamp at all four digital scales
    When the arm, sleeve and closures are tessellated in body, temperature and structural views
    Then the 60.4 mm tube enters the 60.6 mm sleeve bore over 130 mm
    And the terminal cover has a 10 mm lock and a 12 mm wide latch crosses the luminaire gear-cover seam

  Scenario Outline: A heater keeps its carry bar and resolves its grip opening, louvres and dial as its band states
    Given the electric fan heater at scale <scale>
    Then the carry bar stands above the cover across an 80 mm span in body, temperature and forces
    And the 26 mm opening under the bar is <grip> in body, temperature and forces
    And <body louvres> distinct louvre bands cross the outlet in the body and <thermal louvres> in temperature
    And the rotary dial is <body dial> in the body and present in temperature
    Examples: no solid dimension below the smallest feature of the band (7.5, 15, 30, 75 mm)
      | scale | grip   | body louvres | thermal louvres | body dial |
      | 50    | open   | 8            | 8               | present   |
      | 100   | open   | 4            | 6               | present   |
      | 200   | closed | 1            | 1               | present   |
      | 500   | closed | 1            | 1               | left out  |

  Scenario Outline: A park bench has three separate load-bearing back slats
    Given the park bench at scale <scale>
    Then a vertical section through the back spans three boards with two clear spaces in body, temperature and forces
    Examples:
      | scale |
      | 50    |
      | 100   |
      | 200   |
      | 500   |

  Scenario Outline: The balcony guard and thermal equipment retain their stock at every scale
    Given a cantilever balcony, panel radiator, finned heat exchanger and outdoor heat pump at scale <scale>
    Then eight 12 mm balusters and four 110 mm anchor studs secure the open guard
    And both radiator pressure panels, a pierced convection outlet and two 15 mm tails on 50 mm centres are present
    And twelve tubes on each of two staggered rows pass through 134 foils separated by 2.8 mm gaps
    And three pitched fan blades run inside the heat pump guard beside its intake louvres
    Examples:
      | scale |
      | 50    |
      | 100   |
      | 200   |
      | 500   |

  Scenario: Generator air passages and compressor isolation mounts remain open
    Given the diesel generator and scroll compressor at 1:50, 1:100, 1:200 and 1:500
    Then air passes through all four framed generator louvres in body, temperature and forces
    And four separate rubber contacts surround the compressor mounting bolts
    And the mounting plate has a through bore on each 190.5 mm mounting centre

  Scenario: Wall build-ups and suspended controls survive the four digital scales
    Given 120 and 300 mm insulated wall samples, a Venetian blind and a solar collector
    Then 15 mm lime plaster and 20 mm render enclose the brickwork and correct mineral wool
    And four 60 mm anchor heads transfer the wind reaction into the blockwork
    And eighteen 1.13 m slats remain carried by two lift tapes and a head bearing
    And two diagonal 2 mm steel straps brace the rear of the collector stand

  Scenario: Glazing fasteners, sliding-door suspension and cut masonry remain physical
    Given the glass balustrade, sliding door and solid brick wall at all four scales
    Then two drilled 10 mm panes enclose 1.5 mm PVB above two five-stud anchor rows
    And seven clamp bolts pass through the laminate and clamping cheeks
    And the glazed door travels on eight wheels with a pull handle on either face
    And twelve mortar beds cross the masonry behind separate indoor and outdoor finishes

  Scenario: Exposed structural assemblies retain their fixings at every scale
    Given the tied canopy, sectional garage door and masonry lintel wall
    Then both canopy clevises and pins are supported by two four-stud wall plates
    And ten door rollers support four articulated panels connected by three paired hinge-pin rows
    And the thirty-three facing courses frame two lintels, cavity insulation and plastered reveals

  Scenario: Insulating glass includes a real inter-pane cavity
    Given the curtain-wall bay and tilt sash at the four authored scales
    Then the curtain wall has two 6 mm panes with a clear 16 mm space between them
    And the tilt sash has two 4 mm panes with the same clear cavity

  Scenario: The boiler flue has an open load-bearing spigot
    Given the boiler at 1:50, 1:100, 1:200 and 1:500
    Then its structural spigot has a 74 mm bore and 80 mm outside diameter
