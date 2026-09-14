@capability-repo.metrics.benchmark-summary
@no-oracle-repo-metrics-benchmark
@comparison-ordered-json-v1
Feature: Benchmark stdout folds into one comparison table
  The five compose benchmarks each print `name,time` lines and nothing else that matters, so the
  domain filters the noise a build emits — warning lines, lines carrying a colon, lines carrying a
  path separator — and folds what remains into one table with a fixed column order (Typescript,
  Python, Go, C#, Rust), one row per case sorted by name, and the fastest ecosystem per row.

  The recorded stdout of the five ecosystems is shared://⏱️benchmark-output.json and the expected
  table is recorded as shared://📤️benchmark-summary.json

  @id-parses-benchmark-stdout
  @level-fundamental
  @mode-differential
  Scenario: Only real timing lines survive the filter
    Given the recorded stdout of every ecosystem
    When the host parses each stream into timing records
    Then the warning line and the path line are dropped and every timing keeps its ecosystem

  @id-summarizes-and-renders-csv
  @level-fundamental
  @mode-differential
  Scenario: The table and its CSV rendering carry the same rows in the same order
    Given the parsed timings
    When the host summarises them and renders the CSV report
    Then a case missing from an ecosystem leaves an empty cell rather than shifting the row
    And the fastest ecosystem of each row is the smallest parsed duration
