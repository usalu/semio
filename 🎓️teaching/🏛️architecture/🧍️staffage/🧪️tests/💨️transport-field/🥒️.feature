Feature: Transport field
  Air parcels live in world space on the absolute timeline; the field at a time is a pure function of that time.

  Scenario: The head turns after parcels were emitted
    Given a head turning by 90 degrees between 3 s and 4 s
    When the field is solved before and after the turn
    Then the pathlines of earlier parcels are unchanged
    And only parcels born after the turn leave along the turned direction

  Scenario: A walking source leaves a trail
    Given a figure walking at 1.3 m/s through several clip loops
    When the field is solved at 9 s
    Then parcels start at the world position of their birth instant on a straight line
    And the oldest parcel stays near its birth position

  Scenario: The wind changes in the timeline
    Given a wind that rises from 0 to 3 m/s at 6 s
    When the field is solved at 9 s
    Then the pathlines agree with the calm solution before 6 s and are pushed downwind afterwards

  Scenario: Play equals scrub
    Given a walking figure with a head turn
    When the field is played at 24, 25, 30 and 60 frames per second, scrubbed in reverse and evaluated cold
    Then every field at a common time is bit-identical

  Scenario: Parcels never teleport
    When consecutive 1/120 s fields are compared
    Then no parcel moves more than 1.5 times its speed times the step

  Scenario: Exhaust follows its operating level
    Given an engine that ignites at 2 s to 3 s and shuts down at 8 s to 9 s
    Then no parcel is born before ignition or after shutdown and flow and temperature rise scale with the level

  Scenario: Plume heat
    Given a body whose thermal solver reports 60 W sensible heat for its plume region
    Then the plume stream releases 60 W as buoyant air

  Scenario: Transpiration
    Given a crown with six transpiration anchors and 150 W latent heat
    Then parcels carry vapour only at ambient temperature, start at the six anchors and release 150 W divided by the latent heat of vaporisation

  Scenario: Condensation
    Given exhaled air at 34 degrees and 95 percent humidity
    Then supersaturated parcels exist in 5 degree air and none in 30 degree air

  Scenario: Streamlines are not pathlines
    Given steady wind and no jets
    Then streamlines are straight lines at the wind speed
    And after a wind change a pathline is bent while the streamline is straight
