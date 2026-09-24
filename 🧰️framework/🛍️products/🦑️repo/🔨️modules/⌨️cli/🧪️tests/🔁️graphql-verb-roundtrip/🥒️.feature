@capability-repo.cli.graphql-verb-roundtrip
@no-oracle-repo-cli-owned-verb-documents
@comparison-ordered-json-v1
Feature: A read verb builds a GraphQL document, the engine executes it and the renderers print it
  Almost every repo verb is a thin front for one GraphQL document. The verb decides the document
  and its variables; the engine turns the execution into an ordered event stream; the renderers
  decide the bytes. This case closes that loop end to end against the frozen repository in
  shared://🔁️graphql-verb-roundtrip/🗄️repo-records.json, so a drift in the document a verb builds, in the event order
  the engine emits or in the way a renderer projects a payload is a failing test.

  The repository is the recording context of 🔗️graphql, not a filesystem: a disagreement here is
  a disagreement about the CLI, never about what happens to be on disk.

  @id-every-verb-document-executes-and-renders
  @level-fundamental
  @mode-conformance
  Scenario: Every read verb's document executes and renders in all three formats
    Given the frozen repository shared://🔁️graphql-verb-roundtrip/🗄️repo-records.json and the vectors shared://🔁️graphql-verb-roundtrip/🔁️verb-queries.json
    When the host executes each accepting document through the engine and renders the stream three ways
    Then each rendering carries the stated marker, the NDJSON body is a single JSON line and the exit code is zero

  @id-an-execution-failure-becomes-a-failing-stream
  @level-fundamental
  @mode-error
  Scenario: A refused document produces an error event and a non-zero exit code
    Given the frozen repository shared://🔁️graphql-verb-roundtrip/🗄️repo-records.json and the vectors shared://🔁️graphql-verb-roundtrip/🔁️verb-queries.json
    When the host executes each refusing document through the engine
    Then the stream carries the refusal on standard error, standard output stays empty and the exit code is one

  @id-rendering-is-deterministic
  @level-quick
  @mode-round-trip
  Scenario: The same document rendered twice produces the same bytes
    Given the frozen repository shared://🔁️graphql-verb-roundtrip/🗄️repo-records.json and the vectors shared://🔁️graphql-verb-roundtrip/🔁️verb-queries.json
    When the host renders each accepting document twice in each format
    Then the two renderings are byte-identical
