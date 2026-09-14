@capability-repo-section-parsing
@no-oracle-repo-region-markers
@comparison-ordered-json-v1
Feature: A file's sections are read the same way by every implementation
  A section is a region marker pair in a programming language, an ATX heading in Markdown, or an
  object key in JSON. The marker convention (`#region <emoji><Name>` behind each language's own
  comment syntax, with the emoji split off the name) is this repository's own; no third party
  implements it, so there is no credible oracle for it — see the recorded no-oracle decision
  `repo-region-markers`. Confidence comes from the pattern table being the one source of truth for
  both implementations, and from two independently written parsers projecting identically.

  The JSON scenario deliberately projects names, nesting and start positions only. Two Go defects,
  both recorded in the Go package's `🗣️Pending` region for the split to fix, keep the end position and
  the key path out of reach there: the parser keeps `*Section` pointers into `Children` slices it
  keeps appending to, so every key but the last in an object has its end position written through a
  stale backing array and comes back as -1, and it stores each key's path on the location record
  rather than on the section it returns, so the returned path is always empty. The Rust twin computes
  the real end and the real path for every key.

  @id-marker-regions-across-languages
  @level-fundamental
  @mode-differential
  Scenario: Region markers nest and close identically in every marker language
    Given the per-language sources shared://🟦️sample.ts, shared://🐹️sample.go, shared://🐍️sample.py, shared://🔷️sample.cs, shared://🦀️sample.rs, shared://💎️sample.rb, shared://🐚️sample.sh, shared://📢️sample.toml, shared://🤸️sample.yaml, shared://🕌️sample.sql and shared://🎙️sample.graphql
    When each implementation parses every source with the language its extension selects
    Then every implementation projects the same section tree, emoji, line range and byte range

  @id-markdown-heading-ranges
  @level-fundamental
  @mode-differential
  Scenario: Markdown headings nest by level and close at the next heading of the same or lower level
    Given the shared source shared://📰️sample.md
    When each implementation parses it as Markdown
    Then every implementation projects the same heading tree and the same line and byte ranges

  @id-json-object-key-tree
  @level-fundamental
  @mode-differential
  Scenario: Every JSON object key becomes a section carrying its path and start position
    Given the shared source shared://🔣️sample.json
    When each implementation parses it as JSON
    Then every implementation projects the same key tree, path and start position
