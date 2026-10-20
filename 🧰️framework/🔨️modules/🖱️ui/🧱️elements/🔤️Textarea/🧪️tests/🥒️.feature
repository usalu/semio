Feature: Native multiline document editing
  Scenario: Portable editing preserves UTF8 grapheme boundaries and line endings
    Given the shared bounded document editing vectors
    When characters, whole graphemes, lines and pasted text are edited
    Then Rust and the independent SQLite text mutation oracle produce the exact same document and byte cursor
    And emoji and combining clusters remain intact
    And undo and redo preserve their event order
    And edits exceeding the UTF8 byte limit have no effect
