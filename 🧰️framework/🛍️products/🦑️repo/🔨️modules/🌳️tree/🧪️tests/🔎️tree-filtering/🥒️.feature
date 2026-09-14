@capability-repo.tree.filtering
@no-oracle-repo-tree-filtering-and-sorting
@comparison-ordered-json-v1
Feature: A filter prunes and collapses the monorepo tree
  Filtering keeps the shape of the tree: a category is structural and always survives its own
  criteria, an entity node that fails one criterion disappears with its subtree, and an entity
  node that is merely hidden by a kind filter has its surviving children lifted into its parent
  instead of disappearing with them.

  The tree and every vector come from shared://🔎️filter-vectors.json, which restates the tree of
  the Go original's own `TestFilterMonorepoTree` and states each surviving outline in full as
  `<depth>|<kind>|<id>|<label>` rows.

  @id-applies-every-filter-vector
  @level-fundamental
  @mode-conformance
  Scenario: Each vector prunes exactly the stated nodes
    Given the filter tree and the eight vectors
    When the host filters the tree with each vector
    Then every surviving outline equals the stated one, an excluded kind lifts its children into its parent, and an allow list of one kind drops every other entity together with its subtree

  @id-absent-filter-returns-the-tree
  @level-fundamental
  @mode-conformance
  Scenario: No filter is not an empty filter
    Given the filter tree
    When the host filters it with no filter at all and, separately, with the empty filter
    Then both return the whole tree unchanged

  @id-filtering-is-idempotent
  @level-quick
  @mode-property
  Scenario: Filtering an already filtered tree changes nothing
    Given every vector of the fixture
    When the host applies the same filter twice
    Then the second pass returns the outline of the first
