Feature: Air flow on screen
  The thermal view of the archive shows the thermodynamic air flow around an asset, animated by the shared clock, in the drawing and in the 3D model.

  Scenario: The layers are part of the address
    Given a viewer that shows only some flow layers
    When its state is written to the address and read back
    Then the same layers are shown, and the default writes nothing

  Scenario: Time never runs backwards for the solver
    Given a pose, a looping clip and a clip that plays once
    When the clock plays, wraps and is scrubbed
    Then the pose flow runs on, the loop keeps its phase at the earliest later instant, and the single clip follows its own time

  Scenario: Developing the flow reports progress and can be cancelled
    Given a flow solver session for a pose
    When the developed field is asked for
    Then progress is reported slice by slice and the field equals a direct solve
    And a cancellation stops the solve and a new request solves again

  Scenario: The figure carries the flow layer
    Given a figure of the thermal view with flow layers
    When it is mounted
    Then it takes the paper the flow domain needs, shows a progress bar with a cancel button while developing, and states the developed field in the reader's language
