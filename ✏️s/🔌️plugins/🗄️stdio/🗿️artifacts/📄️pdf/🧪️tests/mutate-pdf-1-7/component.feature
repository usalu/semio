@capability-pdf-1-7-mutate
@oracle-lopdf-pdf-1-7-mutate
@comparison-semantic-pdf-v1
@mutations-pdf-1-7-any
Feature: Apply every typed PDF 1.7 mutation to a real-world document
  The input is a real 65-page bachelor thesis produced by LaTeX, not a synthetic fixture, and it is
  read where the domain already keeps it. Every scenario copies it into the case work directory
  before touching it; the committed document is never written to. The oracle drives the registered
  `lopdf` reference implementation over this subset's own real object-graph model (18 mutation
  kinds: page insert/remove/reorder/media-box/crop-box/rotate/content-replace/content-append, plus
  the raw object-graph vocabulary — insert/remove/set-object, dict-entry and trailer-entry edits).
  `remove-page` and `set-info` route through the shared `document` module's own
  `oracle_delete_page`/`oracle_replace_metadata`; every other kind is this module's own. Both the
  oracle's and the subject's results are read back by the SAME independent `lopdf`-backed projection
  before comparison, never against each other's own writing.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                    |
      | no-mutation          | {}                                                                                                                                                                        |
      | set-snapshot         | {"declaredVersion": "2.0", "title": "Wave 7 Snapshot Title"}                                                                                                             |
      | insert-page          | {"index": 30, "page": {"mediaBox": [0, 0, 612, 792], "rotate": 0, "text": "Inserted page for wave 7 mutation testing"}}                                                  |
      | remove-page          | {"index": 7}                                                                                                                                                              |
      | set-page-media-box   | {"index": 15, "mediaBox": [0, 0, 595, 842]}                                                                                                                              |
      | set-page-crop-box    | {"index": 16, "cropBox": [10, 10, 580, 820]}                                                                                                                             |
      | append-page-content  | {"index": 17, "text": "Appended content line for wave 7 testing"}                                                                                                        |
      | set-info             | {"title": "Wave 7 Replaced Title", "author": "Wave 7 Test Author"}                                                                                                       |
      | insert-object        | {"id": {"num": 900001, "gen": 0}, "value": {"kind": "dict", "entries": [{"key": "Type", "value": {"kind": "name", "value": "SemioWave7Marker"}}, {"key": "Note", "value": {"kind": "str", "value": "inserted by wave 7"}}]}} |
      | remove-object        | {"id": {"num": 3015, "gen": 0}}                                                                                                                                          |
      | set-object-value     | {"id": {"num": 145, "gen": 0}, "value": {"kind": "dict", "entries": [{"key": "S", "value": {"kind": "name", "value": "GoToR"}}, {"key": "Note", "value": {"kind": "str", "value": "replaced by wave 7"}}]}} |
      | set-dict-entry       | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "PageMode", "value": {"kind": "name", "value": "UseNone"}}                                                           |
      | remove-dict-entry    | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "Outlines"}                                                                                                           |
      | set-trailer-entry    | {"key": "SemioWave7Marker", "value": {"kind": "int", "value": 42}}                                                                                                       |
      | remove-trailer-entry | {"key": "ID"}                                                                                                                                                            |
      | move-page            | {"from": 10, "to": 40}                                                                                                                                                   |
      | set-page-content     | {"index": 20, "text": "Replaced page content for wave 7 mutation testing"}                                                                                               |
      | set-page-rotation    | {"index": 5, "rotation": 90}                                                                                                                                             |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                    |
      | no-mutation          | {}                                                                                                                                                                        |
      | set-snapshot         | {"declaredVersion": "2.0", "title": "Wave 7 Snapshot Title"}                                                                                                             |
      | insert-page          | {"index": 30, "page": {"mediaBox": [0, 0, 612, 792], "rotate": 0, "text": "Inserted page for wave 7 mutation testing"}}                                                  |
      | remove-page          | {"index": 7}                                                                                                                                                              |
      | set-page-media-box   | {"index": 15, "mediaBox": [0, 0, 595, 842]}                                                                                                                              |
      | set-page-crop-box    | {"index": 16, "cropBox": [10, 10, 580, 820]}                                                                                                                             |
      | append-page-content  | {"index": 17, "text": "Appended content line for wave 7 testing"}                                                                                                        |
      | set-info             | {"title": "Wave 7 Replaced Title", "author": "Wave 7 Test Author"}                                                                                                       |
      | insert-object        | {"id": {"num": 900001, "gen": 0}, "value": {"kind": "dict", "entries": [{"key": "Type", "value": {"kind": "name", "value": "SemioWave7Marker"}}, {"key": "Note", "value": {"kind": "str", "value": "inserted by wave 7"}}]}} |
      | remove-object        | {"id": {"num": 3015, "gen": 0}}                                                                                                                                          |
      | set-object-value     | {"id": {"num": 145, "gen": 0}, "value": {"kind": "dict", "entries": [{"key": "S", "value": {"kind": "name", "value": "GoToR"}}, {"key": "Note", "value": {"kind": "str", "value": "replaced by wave 7"}}]}} |
      | set-dict-entry       | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "PageMode", "value": {"kind": "name", "value": "UseNone"}}                                                           |
      | remove-dict-entry    | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "Outlines"}                                                                                                           |
      | set-trailer-entry    | {"key": "SemioWave7Marker", "value": {"kind": "int", "value": 42}}                                                                                                       |
      | remove-trailer-entry | {"key": "ID"}                                                                                                                                                            |
      | move-page            | {"from": 10, "to": 40}                                                                                                                                                   |
      | set-page-content     | {"index": 20, "text": "Replaced page content for wave 7 mutation testing"}                                                                                               |
      | set-page-rotation    | {"index": 5, "rotation": 90}                                                                                                                                             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
