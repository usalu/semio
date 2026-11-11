Feature: Admission Fails Safe When The Host Sample Is Starved

  Scenario: A Failing Read Keeps The Last Good Sample
    Given a host sample that was read a few seconds ago
    When the live read fails under load
    Then admission and the fleet status use the last good sample with its age

  Scenario: A Sample That Is Too Old Is Not Evidence
    Given a host sample older than the stale threshold
    When admission reads it
    Then it reads as raised pressure with partial counters and never as a healthy host

  Scenario: Stale Data Admits One Run At A Time
    Given two waiting requests that both fit a healthy host
    When the sample is stale and nothing else runs
    Then only the first is admitted, with its minimum jobs, and the second waits
