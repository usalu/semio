@capability-repo.metrics.time-bucketing
@no-oracle-repo-metrics-time-bucketing
@comparison-ordered-json-v1
Feature: A commit stream groups into UTC time buckets
  A LOC history is a time series, so the domain owns the arithmetic that turns an author timestamp
  into a bucket: RFC 3339 for a per-commit step, `YYYY-MM-DDThhZ`, `YYYY-MM-DD`, ISO-8601
  `YYYY-Www`, `YYYY-MM` and `YYYY`. No third-party date library may be linked into a domain crate,
  so both implementations hand-roll the proleptic-Gregorian days-from-civil algorithm and are held
  to the vectors stated here.

  The commits come from the recorded transcript shared://🎞️git-transcript.json and the expected
  keys from shared://📤️time-buckets.json, whose values were produced by a third arithmetic — the
  JavaScript `Date` object — while recording.

  @id-groups-commits-by-granularity
  @level-fundamental
  @mode-differential
  Scenario: Every granularity groups the same commits into the same ordered buckets
    Given the recorded commit stream
    When the host groups it at each of the six granularities
    Then the bucket keys, their first UTC second, their commit ids and their summed deltas agree

  @id-formats-utc-timestamps
  @level-fundamental
  @mode-conformance
  Scenario: The stated instants render and bucket exactly as specified
    Given the instants -2208988800, 0, 951782400, 1234567890, 1767225600, 1769904000 and 4102444800
    When the host renders each as RFC 3339 UTC and as a key at each granularity
    Then a pre-epoch instant, a leap day and a century boundary all render correctly

  @id-civil-date-round-trips
  @level-quick
  @mode-property
  Scenario: Converting a day to a civil date and back is the identity
    Given every 97th day from 30000 days before the epoch to 30000 days after it
    When the host converts each to a civil date and back
    Then no day changes, so the two directions of the algorithm agree
