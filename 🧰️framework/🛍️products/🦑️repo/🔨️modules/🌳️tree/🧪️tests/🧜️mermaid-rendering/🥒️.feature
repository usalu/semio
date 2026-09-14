@capability-repo.tree.mermaid-rendering
@no-oracle-repo-tree-mermaid
@comparison-ordered-json-v1
Feature: A hierarchy renders as a Mermaid treemap
  The `mermaid` verb emits `treemap-beta`: the bare header line, the quoted title, then four
  spaces of indent per level, a quoted label, and `: <value>` on a leaf. The weights themselves
  come from 📊️metrics; the dialect belongs here, so this case renders committed hierarchies
  rather than counting anything.

  The vectors come from shared://🧜️mermaid-vectors.json, which states each diagram as a line list
  so a missing or extra trailing newline cannot hide.

  @id-renders-every-treemap
  @level-fundamental
  @mode-conformance
  Scenario: Each treemap renders into the stated lines
    Given the four treemap vectors
    When the host renders each as a Mermaid treemap
    Then every line list matches, a group carries no value, a leaf carries `: <value>`, a zero is a value rather than an absent one, and a diagram with no node is still a diagram

  @id-projects-a-tree-into-a-treemap
  @level-fundamental
  @mode-conformance
  Scenario: A monorepo tree projects into a treemap by a weight key
    Given the projection tree of the fixture and its weight key
    When the host projects the tree into a treemap and renders it
    Then a node whose data carries the weight key becomes a leaf of that weight and every other node becomes a group

  @id-escapes-quotes-in-labels
  @level-quick
  @mode-property
  Scenario: A double quote never survives into a label
    Given the labels of every vector plus a label made only of double quotes
    When the host escapes each label
    Then no escaped label contains a double quote and escaping an escaped label changes nothing
