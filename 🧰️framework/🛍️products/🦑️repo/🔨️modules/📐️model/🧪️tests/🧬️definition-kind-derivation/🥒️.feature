@capability-repo.model.definition-kind
@no-oracle-repo-definition-kind
@comparison-ordered-json-v1
Feature: A raw declaration keyword always derives one of the four definition kinds
  Every language plugin reports the keyword it matched, and the model turns that keyword into one of
  `implementation`, `interface`, `constant` or `test`. The mapping is this repository's own taxonomy
  of what a definition IS, so nothing third party can arbitrate it — see the recorded no-oracle
  decision `repo-definition-kind`. The derivation is total: an unknown keyword is an implementation,
  never an error and never a blank, and it is case-insensitive but not trimmed.

  @id-every-keyword-derives-one-kind
  @level-fundamental
  @mode-differential
  Scenario: The recognised, the case-shifted and the nonsense keyword all derive the same kind everywhere
    Given the keyword vectors shared://🧬️definition-kind-derivation/🔣️vectors.json
    When each implementation derives the definition kind of every keyword
    Then every implementation projects the same kind for every keyword

  @id-derivation-is-total-and-valid
  @level-fundamental
  @mode-conformance
  Scenario: No keyword derives a value outside the four kinds
    Given the keyword vectors shared://🧬️definition-kind-derivation/🔣️vectors.json
    When each implementation derives the definition kind of every keyword
    Then every derived kind is one of the four declared kinds and none is blank
