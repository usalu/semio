@capability-drawing-1-mutate
@no-oracle-drawing-mutation-semantics
@comparison-ordered-json-v1
Feature: Parse and reprint the real committed example without passing bytes through
  This case carries the whole-document identity law that used to live inside the artifact-level
  `mutate-drawing-1` case alongside the `🏷️metadata`, `🧱️structure`, `🎨️style` and `🔀️transform`
  mutation Examples. It has no vector and no mutation kind, so unlike its four mutation siblings it
  claims no `@mutations-` catalog — `✳️any` owns no mutation catalog of its own now that every
  layer edit has its smallest owner.

  `identity-round-trip` would still be refused by a second implementation: this subset's committed
  snapshot text grammar is the generic `family-scene` canvas grammar, and the committed artifact
  carries no `layers` block at all. Because this case records the same
  `drawing-mutation-semantics` no-oracle decision (in `../../🔮️oracle/🔣️.json`), the runner executes
  NO oracle role, so every assertion below lives in the subject handler.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_drawing_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
