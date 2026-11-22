Feature: Thermodynamic flow view
  The flow field is drawn as stream lines with arrow heads, advected tracers, vapour clouds, fog and the paths of the parcels, over every view and at every scale. It is the only drawing of the air: no discs of parcels, no emitter arrows, no labels in the picture.

  Scenario Outline: Every layer in every viewpoint
    Given the developed flow of a breathing person in calm cold air
    When it is drawn in the <viewpoint> view
    Then stream lines, arrow heads, tracers, vapour clouds, fog and parcel paths are drawn and each layer can be drawn on its own
    And nothing but flow marks is drawn: no text and no emitter marker

    Examples:
      | viewpoint   |
      | elevation   |
      | front       |
      | plan        |
      | axonometric |

  Scenario: Legends with units in both languages
    Given the flow view with all layers
    When its legends are read in English and in German
    Then they name the temperature excess in K, the speed in m/s, the vapour excess in g/kg, the condensate in g/kg and the age of the parcel paths in s, each with the ambient air and its dew point
    And each states that the model is not CFD, with the decimal comma in German
    And the legends of invisible air and vapour declare an explanatory visualization, only the fog depicts visible water

  Scenario: Shared comparison range
    When the range is shared
    Then the temperature excess legend spans plus and minus 5 K for every figure and time
    And an automatic range says that it is normalised to the values present

  Scenario: A second reading besides colour
    When streams, tracers and moisture are drawn
    Then moist air has dotted lines and hollow tracer heads, stream lines are solid

  Scenario Outline: Tracers show direction and speed
    Given a uniform <stream> stream
    When its tracers are read at several times
    Then they are released at the hand-derived period, move with the air, leave a streak of the exposure time or of the shortest readable length, and fade in and out

    Examples:
      | stream |
      | fast   |
      | slow   |

  Scenario: Coarser scales generalise
    Given the same flow field
    When it is drawn at 1:50, 1:100, 1:200 and 1:500
    Then every coarser scale draws no more stream lines, tracers and clouds than the finer one and keeps their spacing on paper

  Scenario Outline: Parcel paths are the parcel histories
    Given the developed flow of a breathing person in calm cold air
    When the pathlines layer is drawn in the <viewpoint> view
    Then every drawn path is the history of its parcel projected into that view, a thin solid line in the colour of the temperature excess the parcel carries now
    And streams and paths are told apart in the legends: paths are where parcels were, streams where the air goes at this instant

    Examples:
      | viewpoint |
      | elevation |
      | front     |
      | plan      |

  Scenario: A trail keeps the recent seconds of a path
    Given one parcel path sampled at 20 Hz that is 1.02 s old
    When a trail of 0.33 s is asked for
    Then the path starts at the hand-derived point where the parcel was 0.33 s ago, four fifths between two samples
    And its fade falls linearly from 1 at the head to 0 at the tail and is drawn in four steps of opacity
    And without a trail the whole path is drawn at full strength

  Scenario: A path behind the body is not drawn
    Given the same parcel path and a solid ball of 5 cm radius between a part of it and the eye of the side elevation
    When the pathlines layer is drawn in the side elevation and in the plan
    Then the side elevation draws the path in two parts that leave out the samples behind the ball
    And the plan, from where the ball is beside the path, draws it whole

  Scenario: One stroke widens every line
    When the widest stream is asked to be 0.6 mm instead of 0.3 mm
    Then stream lines, tracers, vapour lines and parcel paths are twice as wide
