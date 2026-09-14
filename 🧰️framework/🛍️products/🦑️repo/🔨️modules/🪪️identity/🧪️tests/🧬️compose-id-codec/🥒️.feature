@capability-repo-identity-entity-emoji
@capability-repo-identity-artifact-ref
@capability-repo-identity-compose-id
@capability-repo-identity-generation
@no-oracle-repo-identity-owned-codecs
@comparison-ordered-json-v1
Feature: The owned identity codecs round trip on one shared vocabulary
  The presentation-selector normalisation for text-default code points, the artifact reference
  prefixes, the goal and contributor compose ids and the entity emoji vocabulary are this
  repository's own conventions, defined once in `🧬️schema/🔣️entity-emojis.json` and read by every
  implementation from that one file. Identifier generation is judged by shape and by reproducibility
  under an injected seed, never against a reference, because a random identifier has no correct
  value. Nothing third party knows any of this; see `repo-identity-owned-codecs`.

  @id-the-owned-codecs-round-trip
  @level-fundamental
  @mode-round-trip
  @seed-1
  Scenario: Normalisation, artifact references, compose ids and seeded identifiers agree
    Given the shared vector set shared://📡️emoji-vectors.json
    When the host normalises every emoji, classifies every reference, round trips every compose id and renders every seeded identifier
    Then every implementation projects the same result per vector and the same entity emoji vocabulary
