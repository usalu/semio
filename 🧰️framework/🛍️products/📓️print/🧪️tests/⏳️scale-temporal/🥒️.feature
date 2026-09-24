@capability-viz-scale-temporal
@oracle-d3-scale
@oracle-d3-time
@comparison-viz-probe-v1
Feature: A temporal scale reads ISO timestamps as UTC and ticks on calendar boundaries
  The temporal kind of taxonomy §78 converts `YYYY-MM-DD[THH:MM[:SS]]` to epoch milliseconds
  through the Julian day number of the date and then maps linearly, which is exactly what
  `d3-scale`'s `scaleUtc` does — so the mapping, and the inversion back to epoch milliseconds, are
  compared against it directly. There is no local time zone anywhere in the library: a timestamp
  without an offset is UTC, and `%Z` renders `+0000`.

  The ticks are calendar ticks rather than round numbers, and the `interval` key names which
  calendar unit: `day`, `week`, `month` or `year`. Each is compared against the corresponding
  `d3-time` interval range — `utcDay`, `utcWeek` (which is Sunday based, as in d3), `utcMonth` and
  `utcYear` — over the same domain. Tick values are ISO dates and travel as strings.

  @id-map
  @level-quick
  @mode-differential
  Scenario: Timestamps map and invert as scaleUtc does
    Given the committed probe document shared://⏳️scale-temporal/scale-temporal.tex and the temporal scales
      | name    | domain                              | range |
      | tm      | 2026-01-01,2026-12-31               | 0,364 |
      | tmhours | 2026-03-01T00:00,2026-03-02T00:00   | 0,240 |
    Then the compiled probe and the reference implementation agree on every position and epoch

  @id-day-ticks
  @level-quick
  @mode-differential
  Scenario: Day ticks are the utcDay boundaries inside the domain
    Given the committed probe document shared://⏳️scale-temporal/scale-temporal.tex and the domain
      | domain                | interval |
      | 2026-02-25,2026-03-04 | day      |
    Then the compiled probe and the reference implementation agree on every tick date

  @id-week-ticks
  @level-quick
  @mode-differential
  Scenario: Week ticks are the Sunday boundaries inside the domain
    Given the committed probe document shared://⏳️scale-temporal/scale-temporal.tex and the domain
      | domain                | interval |
      | 2026-01-01,2026-03-01 | week     |
    Then the compiled probe and the reference implementation agree on every tick date

  @id-month-ticks
  @level-quick
  @mode-differential
  Scenario: Month ticks are the first days of the months inside the domain
    Given the committed probe document shared://⏳️scale-temporal/scale-temporal.tex and the domain
      | domain                | interval |
      | 2026-01-01,2026-12-31 | month    |
    Then the compiled probe and the reference implementation agree on every tick date

  @id-year-ticks
  @level-quick
  @mode-differential
  Scenario: Year ticks are the first days of the years inside the domain
    Given the committed probe document shared://⏳️scale-temporal/scale-temporal.tex and the domain
      | domain                | interval |
      | 2019-06-01,2024-06-01 | year     |
    Then the compiled probe and the reference implementation agree on every tick date
