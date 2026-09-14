@capability-diagram-solar-position
@oracle-suncalc
@comparison-viz-probe-v1
Feature: The sun-path family computes the solar altitude and azimuth of a place and a moment
  `arch-sunpath` in `semio-viz-diagram-architecture.sty` draws the sun-path chart of §37 and the
  shadow fan from a real solar position, not from a decorative arc. Its `🔖️Keys-arch-sunpath`
  region implements the SunCalc mean-anomaly model in expl3 floating point:

      d   = days since J2000, from the Gregorian date and the UTC hour
      M   = 357.5291 + 0.98560028·d                        (solar mean anomaly)
      C   = 1.9148 sin M + 0.02 sin 2M + 0.0003 sin 3M     (equation of the centre)
      L   = M + C + 102.9372 + 180                         (ecliptic longitude)
      dec = asin(sin 23.4397° · sin L)
      ra  = atan2(sin L · cos 23.4397°, cos L)
      H   = 280.16 + 360.9856235·d + longitude − ra        (hour angle)
      altitude = asin(sin φ sin dec + cos φ cos dec cos H)
      azimuth  = atan2(sin H, cos H sin φ − tan dec cos φ)  (from south, west positive)

  The day number is the Fliegel–Van Flandern conversion, valid across the Gregorian calendar the
  product targets.

  **The oracle.** `suncalc` implements exactly this model and is registered for
  `diagram-solar-position`, so the reference altitude and azimuth come from it rather than from a
  copy of the formulas: the adapter only converts the sampled date and hour into the UTC instant
  suncalc takes and its radians into the degrees the probe emits. The probe emits
  `geometry/diagram-sun` records of `altitude, azimuth`, one per sampled hour.

  @id-solstice-day-arc
  @level-quick
  @mode-differential
  Scenario: The summer-solstice arc over Hannover
    Given the solar samples
      | latitude | longitude | date       | hour | altitude | azimuth  |
      | 52.37    | 9.73      | 2026-06-21 | 6    | 23.8922  | -97.9287 |
      | 52.37    | 9.73      | 2026-06-21 | 9    | 50.3166  | -57.1091 |
      | 52.37    | 9.73      | 2026-06-21 | 12   | 60.2213  | 17.2392  |
      | 52.37    | 9.73      | 2026-06-21 | 15   | 39.9795  | 76.2667  |
      | 52.37    | 9.73      | 2026-06-21 | 18   | 13.0203  | 111.6385 |
    Then every sample matches the solar-position model

  @id-southern-winter-arc
  @level-quick
  @mode-differential
  Scenario: The same date south of the equator, where the arc runs through the north
    Given the solar samples
      | latitude | longitude | date       | hour |
      | -33.87   | 151.21    | 2026-06-21 | 0    |
      | -33.87   | 151.21    | 2026-06-21 | 3    |
      | -33.87   | 151.21    | 2026-06-21 | 6    |
    Then every sample matches the solar-position model
