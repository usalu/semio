@capability-repo.cli.stream-rendering
@no-oracle-repo-cli-owned-stream-presentation
@comparison-ordered-json-v1
Feature: One event stream renders as NDJSON, as human text and as markdown
  Every repo verb produces the same ordered event stream — `start`, then `result` or `error`,
  then `done` — and the caller chooses only how it is presented. NDJSON writes the result payload
  to standard output and the error payload to standard error and nothing else. The human renderer
  prints entities, an analyze summary and a closing `ok done`/`failed failed` line, painting
  colour only for a terminal. Markdown prints the same payloads as links and list items.

  Every vector in shared://🖨️render-formats/📡️event-streams.json states one stream, one format, whether the
  output is a terminal, whether details are verbose, an elapsed time (supplied rather than
  measured, so the summary line is deterministic) and the exact stdout, stderr and exit code the
  renderer owes.

  @id-every-stream-renders-its-stated-bytes
  @level-fundamental
  @mode-conformance
  Scenario: Every vector renders exactly the stated standard output, standard error and exit code
    Given the vectors shared://🖨️render-formats/📡️event-streams.json
    When the host renders each stream in its stated format
    Then the produced bytes and exit code equal the stated ones

  @id-no-colour-without-a-terminal
  @level-fundamental
  @mode-conformance
  Scenario: The human renderer emits no escape sequence when the output is not a terminal
    Given the vectors shared://🖨️render-formats/📡️event-streams.json
    When the host renders every human vector with the terminal decision forced off
    Then no produced byte is an escape character

  @id-the-exit-code-follows-the-done-event
  @level-quick
  @mode-conformance
  Scenario: Every renderer reports the exit code the terminal event carried
    Given the vectors shared://🖨️render-formats/📡️event-streams.json
    When the host renders each stream in all three formats
    Then all three report the same exit code, and it is the one the `done` event carried
