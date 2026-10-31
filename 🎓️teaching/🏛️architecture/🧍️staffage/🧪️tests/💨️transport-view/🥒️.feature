Feature: Transport view
  The overlay draws parcels, pathlines, instantaneous streamlines and modelled visible condensation as separate, labelled layers.

  Scenario: Layers are separate and labelled
    Given a transport field of a breathing figure in cold air
    When all four layers are drawn
    Then each layer has its own items
    And the picture says "instantaneous streamlines", "particle pathlines" and "modelled visible condensation"

  Scenario: Legends state units and the model note
    When the legends are built
    Then each has a unit, a sign convention, an interpretation, English and German texts and the note that the model is not CFD

  Scenario: Shared comparison range
    When the range is shared
    Then the temperature excess legend spans plus and minus 20 K for every figure and time
