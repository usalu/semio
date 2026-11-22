Feature: Air flows around the body of the asset
  The body of the evaluated asset is a small set of analytic obstacles. No air crosses a wall, streams and parcels slide around the body, foliage lets a share of the air through, and marks behind the body are hidden from the eye of a view.

  Scenario: Uniform wind past a sphere is the potential flow
    Given a uniform wind and one sphere far above the ground
    When the field is sampled around the sphere and just outside its surface
    Then the velocity matches the closed-form potential flow, the normal velocity on the surface is zero, the speed at the equator is 1.5 times the wind and the air stagnates at the two points on the axis of the wind

  Scenario: Streams go around the sphere
    Given a uniform wind and one sphere
    When streams are traced from seeds upwind of it
    Then their points and travel times match scipy solve_ivp of the analytic field in arc length and no point lies inside the sphere

  Scenario: Several obstacles and the ground stay impermeable
    Given an ellipsoid, a rounded cylinder and a rounded box that meet above the ground
    When the field is sampled on the open surface of each and on the ground
    Then the normal velocity is zero everywhere

  Scenario: Foliage is porous
    Given a porous obstacle of blockage 0.6
    When the field is sampled inside it and on both sides of its surface
    Then the air inside moves at 0.4 of the wind and the normal velocity is continuous across the surface

  Scenario Outline: An eye sees past a superellipsoid what a ray cast sees
    Given the <shape> as a triangle mesh in trimesh
    When every probe is looked at from every eye, orthographic and perspective
    Then the probes hidden and seen by the product are those of the ray casts, outside the margin of the silhouette

    Examples:
      | shape     |
      | ellipsoid |
      | cylinder  |
      | box       |

  Scenario: A porous obstacle veils
    Given a porous obstacle of blockage 0.6
    When a point behind it, a point inside it and a point in front of it are looked at
    Then the first two are seen at 1 minus 0.75 times the blockage and the third fully

  Scenario: No stream and no parcel runs through a figure
    Given the sample figure with its authored volume in a wind of 2 m/s
    When its transport is solved with the flow field
    Then its obstacles are at most sixteen walls that hold the posed triangles of the kernel, no stream point and no parcel older than a second lies inside a wall or deep inside the kernel's triangles as trimesh contains them, and the wind streams pass close beside the body

  Scenario: Marks behind a figure are hidden
    Given the flow field of the sample figure in wind
    When probes are looked at from every eye
    Then the hidden probes agree with trimesh ray casts against the kernel's triangles within the fit of the obstacles, and the marks of a side view carry hidden and visible points

  Scenario: An asset without a volume is untouched
    Given the sample figure without its volume
    When its transport is solved
    Then its parcels have the digest recorded before obstacles existed
