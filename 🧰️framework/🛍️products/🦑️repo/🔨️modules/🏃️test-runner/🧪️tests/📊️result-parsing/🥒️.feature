@capability-repo.test-runner.result-parsing
@oracle-repo-test-runner-second-parser
@comparison-ordered-json-v1
Feature: Six runner dialects fold into one outcome model
  go test -json, the vitest JSON reporter, pytest's terminal report, libtest, cargo-nextest, the
  dotnet console logger and rspec --format json each describe a test run in their own vocabulary. A
  caller of this module sees none of that: it sees tests with a name, a suite, a verdict, an optional
  duration and an optional message, plus totals that are always recounted from those tests rather
  than read from the report's own summary line. No published library parses all of these into one
  model, so the reference is a second parser written independently in TypeScript against the same
  recorded transcripts, registered as the reference `repo-test-runner-second-parser` and run in the
  oracle role by the harness.

  @id-every-dialect-folds-into-one-model
  @level-fundamental
  @mode-differential
  Scenario: Every recorded transcript projects the same outcome in both implementations
    Given the recorded runner transcripts shared://📜️runner-transcripts.json
    When the host parses each transcript with the parser its runner selects
    Then the subject and the reference project the same tests, verdicts, durations and messages

  @id-totals-are-recomputed-not-trusted
  @level-fundamental
  @mode-conformance
  Scenario: Totals come from the parsed tests, never from the report's own summary
    Given the recorded runner transcripts shared://📜️runner-transcripts.json
    When the host parses each transcript and recounts the verdicts itself
    Then the totals equal the recount, and go's package-level verdict is not counted as a test

  @id-unparseable-output-is-an-empty-run
  @level-fundamental
  @mode-error
  Scenario: Output that is not a report at all yields no tests and a failed run
    Given the recorded runner transcripts shared://📜️runner-transcripts.json
    When the host parses the transcript whose stdout is a module-resolution error
    Then it reports zero tests and a failed run rather than an empty green result
