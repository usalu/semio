@capability-staffage-thermal-multiview
Feature: Thermal and transport read the same physical state in the side, front and top view
  The thermal field and the air transport (parcels, pathlines, streamlines, condensation, emitters) are solved once per instant in world metres
  (x ahead, y up, z left, a left-handed frame) and only projected afterwards: elevation (x, y), front (z, y), plan (x, z) through the cameras of the views module.
  The wind is a horizontal bearing from +x towards +z, so a crosswind drifts parcels sideways in the plan and not at all in the side view.

  THE REFERENCE is numpy: the passive wind drift of a zero-momentum parcel, the orthographic projection of world points and the symmetric auto range.

  Scenario: One region has one value, one legend and one range in all three views
    Given the jogger on the run cycle and the diesel generator on the load cycle at the same time and environment
    When the thermal view is drawn in elevation, front and plan with the shared range and with the auto range
    Then every region carries the solved surface temperature wherever it is visible
    And the legends of the three views state the same quantity, unit, range, stops and conditions
    And a region that a viewpoint cannot see is named in its legend

  Scenario: Parcels, pathlines and emitters sit at the same world place in every view
    Given the breath of the jogger and the exhaust of the generator
    When the transport overlay is drawn in the three views
    Then each parcel is at the projection of its world position and the three views draw the same parcels and legends
    And an emitter pointing along the viewing axis is drawn end-on

  Scenario: A crosswind drifts parcels sideways
    Given a wind bearing of 0, 30, 90 and 180 degrees on a transpiring tree without turbulence
    Then the pathlines follow the numpy drift U (cos bearing, sin bearing) t in x and z

  Scenario: A moving emitter and its older parcels stay consistent in world space
    Given a figure that moves sideways and turns its head
    Then the emitter moves linearly in z, its direction turns with the head and earlier parcels keep their world positions

  Scenario: Scrubbing equals playing
    Given a figure that moves and turns
    Then playing frame by frame, scrubbing in any order and evaluating cold give bitwise identical fields

  Scenario: One environment, one clock, one weighting
    Then opposite winds blend through calm in thermal and in transport
    And the thermal history is continuous across a clip loop boundary
    And a fixed range states the clamped neutral grey
    And merged regions are weighted by area times exposed fraction
    And emitters sit at the 3D frame of a spread or twisted bone
