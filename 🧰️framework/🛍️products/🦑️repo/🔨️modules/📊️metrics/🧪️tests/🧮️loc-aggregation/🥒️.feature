@capability-repo.metrics.loc-aggregation
@oracle-semio-library-uloc
@comparison-ordered-json-v1
Feature: Tracked files and git deltas compose into one LOC report
  One line of code is this repository's own unit: physical newline-terminated lines everywhere,
  except `.json` and `.jsonc`, which count their object keys recursively and fall back to physical
  lines only when a document has no keys at all. On top of that unit the report adds the aggregate
  `Code`, `Markup`, `Data` and `Total` rows, each row's share of the tree, each row's share of the
  branch's edited-line churn, and — for a history step — the change in tree LOC against the
  previously printed step.

  Every input comes from the recorded transcript shared://🎞️git-transcript.json, so nothing here
  touches a live repository. The reference is the TypeScript library's `countUnifiedLocForFile`,
  the third and oldest implementation of the same unit, which the repository dashboard already
  ships — a supplement rather than an independent authority, and the reason this feature also
  states its vectors explicitly.

  @id-counts-unified-loc-per-tracked-file
  @level-fundamental
  @mode-differential
  Scenario: Every tracked text file counts to the same number of lines of code
    Given every text blob the transcript recorded at HEAD
    When the host counts unified LOC for each of them
    Then the code, markup and data files agree file by file
    And the JSON document counts its keys rather than its lines

  @id-composes-the-snapshot-table
  @level-fundamental
  @mode-differential
  Scenario: The snapshot table carries the aggregate rows and both percentages
    Given the recorded log stream and the tracked tree at HEAD
    When the host composes the snapshot report
    Then the Code row sums the enabled code languages and the Total row sums code, markup and data
    And every row's percent is its share of the Total row's LOC, and Total is exactly 100

  @id-history-stamps-delta-against-previous-row
  @level-quick
  @mode-differential
  Scenario: Each history step reports the tree at that commit and the change since the last one
    Given the recorded log stream and the recorded tree at every commit
    When the host builds the history series
    Then each step carries the RFC 3339 commit time and the running deltas up to it
    And the first step has no delta against a previous row while every later one does

  @id-renders-the-markdown-snapshot-table
  @level-quick
  @mode-differential
  Scenario: The snapshot renders as one GitHub-flavoured table
    Given the composed snapshot report
    When the host renders it as markdown
    Then the rows are ordered by LOC descending with Total last, and every percent has two decimals
