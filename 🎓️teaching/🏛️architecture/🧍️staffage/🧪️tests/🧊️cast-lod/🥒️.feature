@capability-staffage-lod
Feature: A cast is drawn at a level of detail chosen before anything is loaded
  A render profile names a level of detail. The level samples the same analytical state at canonical times and draws a subset of the same authored shapes; it never changes units, origin, anchors, scenario or the clock of the cast. The vectors live in `🧫️fixtures/🧊️cast-lod/🔣️.json`.

  THE REFERENCES are ajv (the levels file against its schema), the repository's own schedule and shape pipeline recomputed per cast, and a full-detail export compared frame by frame at coinciding canonical times.

  Scenario: A level samples the canonical clock without losing its endpoints
    Given the schedule vectors of looping clips, non-looping transients, a pose and the poster
    When each level is resolved for the output rate of the job
    Then a preview samples at most half the output rate and at most 90 frames, a non-looping clip keeps both end samples and a loop wraps in whole periods

  Scenario: Every course cast resolves at every level
    Given every cast of the course manifest
    When all three levels are resolved
    Then every schedule is monotone and inside the canonical clock, the full level is the canonical schedule and every thermal region stays painted

  Scenario: A sampled frame carries exactly the full frame values
    Given a thermal walk cycle exported at the full, preview and poster levels
    When the preview frames are compared with the full frames at the same canonical times
    Then every shape carries the same fill and the logical descriptor is identical for every level
