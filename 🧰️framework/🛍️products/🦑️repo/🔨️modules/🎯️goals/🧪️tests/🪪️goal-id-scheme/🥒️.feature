@capability-repo.goals.id-scheme
@no-oracle-repo-goals-id-scheme
@comparison-ordered-json-v1
Feature: A goal identifier states where the goal sits and what management it owns
  A goal is addressed by its `GOAL/SUBGOAL` path, and the depth of that path is the whole rule:
  a root goal owns a milestone, a first generation goal owns an issue under that milestone, and
  anything deeper owns an issue nested under its parent's. The same goal is also addressed by the
  🎯️-tagged compose identifier stored documents carry, and both forms have to resolve to the same
  place. The fixture freezes inputs only — real identifiers out of `.🧬semio/🦑️repo/🎯️goals`
  alongside the boundaries the rule has to answer for — because writing the answers down by hand
  would let a shared misreading pass; see `repo-goals-id-scheme`.

  @id-identifiers-resolve-to-one-place
  @level-fundamental
  @mode-differential
  Scenario: Depth, root, parent, filesystem form and composition agree everywhere
    Given the identifier vectors shared://🪪️id-vectors.json
    When each implementation classifies every path, composes every title under its parent and reads the number out of every reference
    Then every implementation projects the same depth, root, parent, filesystem form, compose identifier, composed identifier and parsed number
