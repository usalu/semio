@capability-repo.tree.goal-statute-territory
@no-oracle-repo-tree-projection
@comparison-ordered-json-v1
Feature: Goals, statutes and territories project into their own trees
  Besides the monorepo tree the domain owns three smaller projections: the goal/ticket tree that
  nests a subgoal under the goal its id path names and a ticket under the ticket it names as its
  parent; the statute tree that turns a slash-separated statute path into one category per
  segment; and the territory tree that lists the statutes of a territory before its nested
  territories. A policy view regroups the same statutes by the entity kind they apply to.

  Resolving a statute's metadata, artifact id, uri, display label and entity kind belongs to
  📜️statutes, so it is a port: shared://🎯️goal-statute-territory.json states the catalog the host
  satisfies it with. Rendering an entity line belongs to the CLI renderers, so the goal tree
  renders through the same stub the monorepo tree case states:

    key(data)          = the first non-empty string among name, title, slug, id, path, else ""
    human(kind, data)  = "<kind>#<key>"
    markdownLink(...)  = "[<kind>#<key>]"

  The goals and tickets are the ones of shared://🌳️tree-source.json; every expectation is stated
  in shared://📤️goal-statute-territory-expectations.json, one line list per projection.

  @id-builds-the-goal-tree
  @level-fundamental
  @mode-conformance
  Scenario: Goals nest by id path, tickets by parent, and a goalless ticket gets a home
    Given the goals and tickets of the record set
    When the host builds the goal tree
    Then goals sort by due date with a blank date last, a subgoal hangs under its goal, a child ticket hangs under its parent ticket, and the ticket that names no goal lands under the synthetic "No Goal" node appended after every real goal

  @id-renders-the-goal-tree
  @level-fundamental
  @mode-conformance
  Scenario: The goal tree renders as connector text and as a markdown list
    Given the goal tree
    When the host renders it in both formats through the stub renderer
    Then the text form draws a connector per level while the markdown form indents by two spaces, and the subgoals of a goal render before its tickets

  @id-builds-the-statute-tree
  @level-fundamental
  @mode-conformance
  Scenario: A statute path becomes one category per segment
    Given the four statutes of the fixture
    When the host builds the statute tree through the catalog
    Then the roots keep the order the statutes were declared in, every deeper level is sorted by segment, an inner node is a category identified as `breachCategory:<prefix>`, and a leaf carries its priority icon, its reason and its autofixable flag

  @id-builds-the-territory-tree
  @level-fundamental
  @mode-conformance
  Scenario: A territory lists its statutes before its nested territories
    Given the territory forest of the fixture
    When the host builds the territory tree through the catalog
    Then each territory is a category carrying its scopes, its statutes come first in declaration order and its nested territories follow

  @id-groups-statutes-by-entity-kind
  @level-quick
  @mode-conformance
  Scenario: A policy view regroups the same statutes by entity kind
    Given the territory forest of the fixture
    When the host groups its statutes by the entity kind the catalog reports
    Then the entity kinds are sorted, each carries its statutes sorted by id, and a statute of a nested territory is grouped with the statutes of the outer one

  @id-counts-open-subgoals-and-tickets
  @level-quick
  @mode-conformance
  Scenario: A goal counts the open work below it
    Given the goal tree
    When the host counts the open subgoals and open tickets of every root goal
    Then the counts reach through every level of subgoals and child tickets
