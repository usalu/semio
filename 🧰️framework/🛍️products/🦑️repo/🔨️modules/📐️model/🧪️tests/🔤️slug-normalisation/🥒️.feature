@capability-repo.model.slug-vocabulary
@no-oracle-repo-slug-vocabulary
@comparison-ordered-json-v1
Feature: The LLM, reasoning-effort and client slug vocabularies mean the same thing everywhere
  A ticket, a goal and an agent session all record which model, which reasoning effort and which
  client produced them, and every one of those three values is normalised and then resolved against
  a closed vocabulary that lives in `🧬️schema/🔣️allowed-values.json`. The normalisation is this
  repository's own camel-case-aware slug rule and the resolution is a longest-containment match, so
  no third party implements either — see the recorded no-oracle decision `repo-slug-vocabulary`.
  Confidence comes from the vocabulary table being loaded rather than restated, and from two
  independently written implementations projecting the same answer for the same inputs, including
  the inputs that must be rejected.

  @id-normalisation-is-canonical-and-idempotent
  @level-fundamental
  @mode-differential
  Scenario: Every implementation normalises a slug the same way, and normalising twice changes nothing
    Given the vocabulary vectors local://🔣️vectors.json
    When each implementation normalises every LLM, effort and client input, then normalises the result again
    Then every implementation projects the same canonical slug and the same second pass

  @id-resolution-picks-the-longest-allowed-match
  @level-fundamental
  @mode-differential
  Scenario: Resolution returns the longest allowed vocabulary member the input contains
    Given the vocabulary vectors local://🔣️vectors.json
    When each implementation resolves every LLM, effort and client input against the allowed table
    Then every implementation projects the same resolved member

  @id-unresolvable-input-is-an-error-class
  @level-fundamental
  @mode-differential
  Scenario: An input outside the vocabulary is rejected, never silently defaulted
    Given the vocabulary vectors local://🔣️vectors.json
    When each implementation resolves an input that no allowed member covers
    Then every implementation reports the not-allowed error class instead of a value

  @id-the-allowed-table-is-loaded-not-restated
  @level-quick
  @mode-conformance
  Scenario: Both implementations read the same vocabulary table
    Given the vocabulary table shipped beside the model schema
    When each implementation reports the table it loaded
    Then the three vocabularies agree in content and in order
