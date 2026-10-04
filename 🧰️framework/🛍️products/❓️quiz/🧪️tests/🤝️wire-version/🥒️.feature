@capability-quiz-wire-version
@oracle-quiz-python-reference
@comparison-ordered-json-v1
Feature: The wire version is the same everywhere and is raised with every change of the contract
  A proctor and a client agree on the contract by its wire version alone (`$defs/WireVersion` of
  `🧬️schema/🔣️.json`): every framework envelope of a quiz command or query carries it, a proctor
  declares it for every quiz kind it serves at `GET /instance`, and a client sends nothing to a proctor
  that does not declare every quiz kind it uses at its own version — such a proctor is as away as a
  silent one, so nothing waiting is lost and the deputy decides meanwhile. A proctor refuses an envelope
  of another version. This only holds while both cores carry the version the schema states and the
  version is raised with every change of the contract beyond its prose: a proctor of challenge levels
  and a client from before them both spoke version 1, and the client crashed on sheets without a
  challenge.

  The fingerprint of the contract is FNV-1a 64 (16 lowercase hex digits) over the UTF-8 bytes of the
  schema without its prose — every `description`, `title` and `$comment` whose value is a string; a
  property named `description` is a schema and stays — written as JSON with keys sorted by code point,
  no whitespace and no escaping beyond JSON's own. Every number of the schema is an integer, so the
  writers of all three languages agree on it.

  THE REFERENCE is `🐍️.py` beside this file: Python's own JSON writer (`json.dumps` with `sort_keys`,
  compact separators and `ensure_ascii=False`) and an FNV-1a from its definition. It fails unless the
  version and the fingerprint of the schema are the pair committed in shared://🤝️wire-version/🔣️.json
  — else the contract changed without a raise, or a raise was not committed. A change beyond prose raises
  `$defs/WireVersion` and `WIRE_VERSION` in both twins and commits the pair `🐍️.py` prints when run
  directly, from the repository root:
  `.venv/Scripts/python.exe 🧰️framework/🛍️products/❓️quiz/🧪️tests/🤝️wire-version/🐍️.py`
  (`.venv/bin/python` outside Windows).

  @id-agreement
  @level-fundamental
  @mode-differential
  Scenario: Every core speaks the wire version of the schema, whose fingerprint is the committed one
    Given the normative schema and the committed pair shared://🤝️wire-version/🔣️.json
    When every implementation writes the schema without its prose and hashes it
    Then every implementation projects the wire version of the schema and the committed fingerprint
