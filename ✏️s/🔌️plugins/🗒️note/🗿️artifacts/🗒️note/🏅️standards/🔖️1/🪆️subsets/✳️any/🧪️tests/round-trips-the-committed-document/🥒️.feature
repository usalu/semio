@capability-note-1-mutate
@oracle-note-python-independent
@comparison-ordered-json-v1
Feature: Parse the real committed example document and print it back without losing or copying anything
  This case carries the whole-document identity law that used to live inside the artifact-level
  `mutate-note-1` case alongside the `✳️document`, `✳️canvas`, `✳️ink`, `✳️asset`, `✳️block`,
  `✳️text`, `✳️math` and `✳️table` mutation Examples. It has no vector and no mutation kind, so
  unlike its eight mutation siblings it claims no `@mutations-` catalog — `✳️any` owns no mutation
  catalog of its own now that every block kind and document field has its smallest owner.

  🚧️ THIS SCENARIO THE REFERENCE REFUSES BY CLAUSE, and reports rather than works around. Unlike its
  `✒️writer`, `🌿️vcs` and `🔌️wires` siblings this subset commits a REAL grammar rather than the
  repository-wide `payload = OCTET+` placeholder — which is what makes the gap citable. Grammar and
  artifact disagree on three points: `block = text-block | image-block | shape-block` covers three of
  the SIX declared block kinds, leaving stroke, table, math and group with no production at all;
  `block-field` names `paragraphs` and `asset-id` while the committed artifact writes neither and
  writes `content=child_id=… target="…"`, a flattened nested record nothing bounds; and
  `artifact-mark = "note.note"` is contradicted by the artifact's own first line
  `semio note.note.dsl v1`.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed to a NoteSnapshot, printed back to `.note` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
