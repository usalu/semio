@capability-quiz-identity-shapes
@oracle-quiz-jsonschema
@comparison-ordered-json-v1
Feature: Ids and handles are held to their shapes before anything is decided or stored
  The proctor has no passwords: a learner id is a learner's only credential and a handle is public on the
  leaderboard, so no string a client chose may reach an event, a stream id or a projection key before it
  has its shape — and the server never relies on the client for it. Both cores carry the same validator.

  Ids. A learner, run and command id is exactly 32 lowercase hex characters (`isId`), a quiz and task id a
  slug of at most 64 characters (`isSlug`). `commandRejection` answers `id-invalid` for a command with any
  other id, `queryRejection` the same for a query (the optional caller of a leaderboard query included) and
  `handle-invalid` for a `handle` query whose handle the policy refuses.

  Handles. `normalizeHandle` first collapses every run of `White_Space` (the 25 code points of the Unicode
  property, written out in both cores) to one space, trims, and turns the typographic apostrophe U+2019
  into the apostrophe. What remains must be 1…64 code points of the handle alphabet with at least one
  letter or digit: the upper- and lowercase letters of Basic Latin, Latin-1 Supplement, Latin Extended-A,
  Latin Extended-B and Latin Extended Additional that have no compatibility decomposition (681 letters:
  umlauts, ß and ẞ, accents, Turkish, Polish and Vietnamese letters; no ligatures, no long s, no digraph
  letters), the ASCII digits, single spaces between words and `'` `.` `_` `-`. Everything else is refused:
  control characters, format characters (zero-width space and joiners, word joiner, soft hyphen, bidi
  overrides, isolates and marks, U+FEFF), combining marks, every other script — so the Cyrillic and Greek
  look-alikes of Latin letters, fullwidth and mathematical letters — emoji, other punctuation, a handle of
  punctuation only, and raw input over 256 code points. The key that identifies a handle is the lowercase
  of the display; the stream of a handle is named by the lowercase hex of the key's UTF-8 bytes
  (`handleActorId`).

  Normalization. Rust's standard library has no Unicode normalizer and the cores link none. They need
  none: the alphabet holds only starters that NFC leaves unchanged and no two of them compose, so every
  string spelled in it is in NFC, and every NFD spelling contains a combining mark and is refused — a
  client sends `handle.normalize("NFC")`. Visually identical handles in two normalization forms or two
  scripts can therefore not both be registered.

  THE REFERENCE is `🐍️.py` beside this file. python-jsonschema's Draft 7 validator judges every command and
  query against the normative `Command` and `Query` definitions and every normalized handle against
  `$defs/Handle` (pattern and length). The validator is itself held to the Unicode Character Database
  (`unicodedata`): the alphabet its pattern admits, scanned over every Unicode scalar value, must be
  exactly the set the database derives from the properties above, every member a starter that NFC leaves
  unchanged, and every admitted display its own NFC form. Lowercase is Python's `str.lower`, an
  implementation of the Unicode case mappings unrelated to ICU and to Rust's tables; the `alphabet`
  scenario compares the lowercase of every letter, so a runtime whose Unicode version maps a letter
  differently fails here instead of silently splitting one handle into two.

  The vectors shared://🪪️identity-shapes/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-handles
  @level-fundamental
  @mode-differential
  Scenario: Handles normalise to a display, a lowercase key and a stream id, or are refused
    Given the committed vectors shared://🪪️identity-shapes/🔣️.json
    When every committed handle is normalised
    Then every implementation projects the same display, key and stream id per handle
    And none for a handle that is blank, over-long, not in NFC, invisible, of another script or outside the alphabet

  @id-alphabet
  @level-fundamental
  @mode-differential
  Scenario: The handle alphabet and the lowercase of its letters are the same everywhere
    Given every Unicode scalar value
    When each is placed between two letters and normalised
    Then every implementation keeps exactly the committed alphabet unchanged
    And lowercases every letter to the committed key

  @id-shapes
  @level-fundamental
  @mode-differential
  Scenario: Commands and queries with ill-shaped ids are refused with id-invalid
    Given the committed vectors shared://🪪️identity-shapes/🔣️.json
    When every committed command and query is held to its shapes
    Then every implementation projects id-invalid for an id that is not 32 lowercase hex or a slug, handle-invalid for a refused handle query, and nothing otherwise
