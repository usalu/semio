@capability-staffage-cast
Feature: Every cast names its asset, revision, view, scale, state, clock and scenario
  A scene never guesses what a cast shows. The manifest names every dimension of the drawing, the exporter refuses any missing or unavailable combination, and each exported cast carries a
  provenance record that scenes can read. The vectors live in `🧫️fixtures/🏷️cast-provenance/🔣️.json` and are read by the TypeScript and the Python implementation alike.

  THE REFERENCES are ajv and jsonschema (draft-07 validation of the manifest and of the cast index) and the standard SHA-256 of every runtime.

  Scenario: Missing combinations are refused by two implementations
    Given casts without a scale, with a derived scale, without fps, without ease, without an environment, with transport but no start, and without a view direction
    When each is validated by ajv and by the dependency-free twin
    Then every one is refused and the twin names the missing dimension

  Scenario: The asset revision is the hash of the catalog text
    Given the same catalog text with LF and with CRLF line endings
    When its revision is computed
    Then both give the same sha256 and one more byte gives another

  Scenario: Environment presets are hashed in canonical JSON
    Given a preset with integral floats, small and large numbers and non-ASCII text
    When it is written canonically
    Then keys are sorted, numbers print as ECMAScript prints them and TypeScript and Python agree byte for byte

  Scenario: An exported cast carries its provenance
    Given a clip, a still pose, a thermal clip and a transient radiator
    When the casts are exported
    Then each index names the asset revision, the view, the authored scale, the pose or clip, the clock and the scenario, and passes the index schema

  Scenario: A cast the asset cannot draw is refused before anything is written
    Given a cast at a scale the asset marks unavailable, of an unknown asset, pose, clip or environment preset
    When the casts are exported
    Then the export fails with a message that names the cast and what is available
