Feature: Thermodynamic flow field
  The flow view shows a velocity, temperature and vapour field reconstructed from the parcels of jets and plumes, the streams traced through it and the tracers advected along them.

  Scenario Outline: The field matches its independent re-implementation
    Given the parcels of the <case> from its classical similarity solution in still air
    When the field is sampled inside, beside, above and far from the stream and on the ground
    Then velocity, induced speed, temperature excess and vapour excess match the numpy field

    Examples:
      | case  |
      | plume |
      | jet   |
      | twin  |

  Scenario: Overlapping streams mix
    Given two equal jets through one outlet
    When the field is sampled on their common axis, beside it and at their edge
    Then the temperature and vapour excess on the axis are those of one jet, the velocity is twice that of one jet everywhere, and only at the edge, where ambient air is left in the mixture, the excesses of the two add

  Scenario: A plume draws ambient air in
    Given the top-hat plume of Morton, Taylor and Turner
    When the radial inflow is read beside the plume
    Then it matches the continuous line sink of strength 2 pi b alpha w with its ground image, integrated by scipy quad, within 3 percent

  Scenario: The Gaussian profile carries the plume's volume flux
    Given the top-hat plume of Morton, Taylor and Turner
    When the vertical velocity is integrated over a horizontal plane
    Then the flux matches pi b squared w of the similarity solution plus the flux the entrainment line sink drives through the same disc (scipy quad) within 3 percent

  Scenario: The ground is impermeable
    Given a plume and a jet above the ground plane
    When the field is sampled on the ground
    Then its vertical velocity is zero

  Scenario Outline: Streams and tracers follow the field
    Given the field of the <case>
    When a stream is traced from a seed and a tracer rides along it
    Then its points match scipy solve_ivp in arc length, its travel time the integral of the slowness, and the tracer the particle scipy advects in time

    Examples:
      | case  |
      | plume |
      | jet   |
      | twin  |
