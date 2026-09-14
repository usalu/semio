@capability-repo.goals.tree
@no-oracle-repo-goals-tree
@comparison-ordered-json-v1
Feature: The goal forest nests, sorts and renders the same way everywhere
  Goals nest by identifier prefix, siblings sort by due date and then by identifier with a blank
  due date sorting last, tickets hang under the goal they name and then under the ticket they name,
  and every ticket that names no known goal collects under a trailing `No Goal` node. The rendering
  states that structure twice — once with box drawing connectors and once as a markdown list — and
  counts the open goals and open tickets below every node. Nothing third party has a goal forest to
  compare against; the line renderer is deliberately the identifying members and nothing else, so
  the scenario judges the tree rather than a presentation vocabulary. See `repo-goals-tree`.

  @id-the-forest-nests-and-sorts
  @level-fundamental
  @mode-differential
  Scenario: Assembling and rendering the seeds produces one tree
    Given the tree seeds shared://🌳️tree-vectors.json
    When each implementation assembles the forest and renders it as text and as markdown
    Then every implementation projects the same two renderings and the same open counts per root

  @id-goal-order-does-not-reach-the-rendering
  @level-quick
  @mode-property
  Scenario: The forest does not depend on the order the goal seeds arrive in
    Given the tree seeds shared://🌳️tree-vectors.json
    When each implementation assembles the forest from the goal seeds and from the reversed goal seeds, leaving the ticket seeds in place
    Then every implementation projects the same rendering for both orders, because goals sort and tickets keep the order they were given
