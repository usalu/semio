@capability-repo-statutes-catalog
@no-oracle-repo-statutes-owned-law
@comparison-ordered-json-v1
Feature: Both implementations read one statute catalog
  The closed vocabulary of statutes, the priority and repair advice of each one, whether it can be
  repaired automatically, and the policy and territory tree that groups them are defined once in
  `🧬️schema/🔣️statutes.json`. Neither implementation carries a second copy of that table, so a
  statute added there appears in both at once and a statute removed there disappears from both at
  once. The catalog is also internally consistent: every statute a territory claims exists, no
  statute is claimed by two policies, and every statute resolves to its own metadata.

  @id-the-catalog-is-one-table
  @level-fundamental
  @mode-conformance
  @seed-1
  Scenario: Every statute, policy and territory projects the same table
    Given the statute catalog both implementations load
    When the host renders every statute with its policy, priority and autofixability and every policy with its territory tree
    Then every implementation projects the same catalog

  @id-the-catalog-is-consistent
  @level-fundamental
  @mode-error
  @seed-1
  Scenario: No territory claims a statute the catalog does not declare
    Given the statute catalog both implementations load
    When the host resolves every statute a territory claims and every statute's own metadata
    Then every implementation reports no unknown statute, no statute claimed twice and no missing metadata
