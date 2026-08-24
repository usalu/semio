@capability-html-5-mutate
@oracle-html5ever-html-5-mutate
@comparison-semantic-html-v1
@mutations-html-5-any
Feature: Apply every typed HTML 5 mutation to a real-world document
  The input is shared://🌐️zukunft-bau-entwerfen-mit-bestand.html, derived once from the real,
  committed presentation page at ../../../../../♻️mit-bestand/🎤️präsentation/📅️33.projektetage/
  🌐️public/🌐️zukunft-bau-entwerfen-mit-bestand.html (a real TYPO3-produced HTML5 document, 2,337
  lines / 149 KB, genuine `<head>`/`<body>` structure, real navigation, real footer, real external
  stylesheet/script references). Every byte outside two spots is that real page verbatim. The
  derivation: that page's own already-`<link rel="stylesheet">`-referenced
  `zukunft-bau-entwerfen-mit-bestand_files/🎨️overwrite.css` and already-`<script src="...">`-referenced
  `.../🟨️default_frontend.js` — both real, committed files from the SAME asset directory — are inlined
  in place of their `<link>`/`<script src>` tags, verbatim, so this subset's `SetRawText` kind (which
  needs an existing RAWTEXT node to retarget) has a genuine target: the real page, read exactly where
  it already lived, carries no inline `<script>`/`<style>` content of its own anywhere. Every scenario
  copies the fixture into the case work directory before touching it; the committed fixture is never
  written to.

  HTML5 parsing is aggressively normalizing by specification, and this subset's own subject parser
  draws its own honest boundary (documented at `../../🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/
  📸️snapshot/🦀️component.rs`'s module doc comment: well-formed HTML5 only, five XML-equivalent named
  character references plus numeric ones, no tag-soup error recovery). This case's independent reader
  (`html5ever`/`markup5ever_rcdom`, the real WHATWG tree-construction algorithm) additionally performs
  two normalizations of its own that this case's `semantic-html-v1` profile absorbs rather than hides:
  a boolean attribute (`<p disabled>`) and its empty-string form (`<p disabled="">`) both collapse to
  the SAME attribute-set-to-empty-string once tokenized (the WHATWG tokenizer gives every attribute a
  value, defaulting to `""`, with no "valueless" state at all) — every `set-attribute` example below
  therefore uses a concrete non-empty value, never the valueless branch, which is instead exercised by
  this subset's own Rust-level unit test
  (`../../🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs::
  set_attribute_tristate_apply_and_inverse_round_trip`); and a DOCTYPE's public/system id does not
  survive `html5ever::serialize`'s own `<!DOCTYPE name>`-only writer, so only the doctype name is ever
  compared — the real page only ever carries the bare `<!doctype html>` this costs nothing on.
  Everything else — implied `<html>`/`<head>`/`<body>` insertion, misnested-tag reordering, attribute
  quote-style normalization — is genuine WHATWG-conformant behaviour this case's real fixture never
  actually exercises (it is already well-formed), so a byte-level round trip is not attempted; the
  `semantic-html-v1` projection (doctype plus the full element tree in document order) is what both
  the oracle and the subject are compared through instead. Attribute order is writer freedom: that
  profile projects each element's attributes as a name/value map sorted by key, never the ordered list
  either side's own tree stores them as. Sibling and child order IS normative and is never sorted.

  `InsertNode`/`RemoveNode` are this subset's structural analogue of page operations: `insert-node`
  adds a real, marked `<div>` as `<body>`'s first child; `remove-node` deletes the real (empty)
  `<div class="sidebars">` at `<body>`'s 10th position — a genuinely real, safely-removable element,
  not an invented one. `html5ever`/`markup5ever_rcdom` read AND write real HTML5, so every kind below
  is genuinely differential: the oracle performs the mutation with them, the subject performs it with
  this subset's own `HtmlSnapshot`/`HtmlMutation`/`parse_html_document`/`write_html_document`, and both
  results are read back through the SAME independent `html5ever` projection before comparison.

  Both non-differential laws are asserted IN ROLE, by the handler that plays the role, and are not
  deferred to the oracle-vs-subject comparison: every `inverse-<kind>` row requires apply-then-undo
  to restore that side's OWN reading of the original document's projection, and
  `identity-round-trip` requires that side's own decode → re-encode both to preserve its own
  projection and to move the bytes. HTML 5 is not a byte-preserving carrier — the tree builder
  inserts the implied `html`/`head`/`body` elements and the serializer re-derives every tag and
  character reference from the tree — so the byte half of the law applies in full on both sides. A
  scenario that only proved the reference library did not error would be vacuous — it is checkable
  without a second producer, so it is checked without one.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🌐️zukunft-bau-entwerfen-mit-bestand.html
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    And the semantic projection moved, unless the kind is no-mutation
    Examples:
      | id                | params                                                                                                                                                                                     |
      | no-mutation        | {}                                                                                                                                                                                        |
      | set-snapshot       | {"doctype": "DOCTYPE html", "root": {"kind":"element","name":"html","attributes":[{"name":"lang","value":"de"}],"children":[{"kind":"element","name":"head","attributes":[],"children":[{"kind":"element","name":"title","attributes":[],"children":[{"kind":"text","text":"Wave 7 Snapshot Title"}]}]},{"kind":"element","name":"body","attributes":[],"children":[{"kind":"text","text":"Wave 7 snapshot replacement content"}]}]}} |
      | set-doctype        | {"doctype": "DOCTYPE htmlWave7"}                                                                                                                                                          |
      | insert-node        | {"parent": [2], "index": 0, "node": {"kind":"element","name":"div","attributes":[{"name":"id","value":"wave7-marker"}],"children":[{"kind":"text","text":"Wave 7 mutation testing"}]}}    |
      | remove-node        | {"parent": [2], "index": 9}                                                                                                                                                               |
      | set-element-name   | {"path": [2, 9], "name": "aside"}                                                                                                                                                         |
      | set-attribute      | {"path": [2, 9], "name": "class", "value": "sidebars-wave7"}                                                                                                                              |
      | set-text           | {"path": [0, 9, 0], "text": "Wave 7 Mutation Testing"}                                                                                                                                    |
      | set-comment        | {"path": [0, 5], "text": " Wave 7 replaced comment "}                                                                                                                                     |
      | set-raw-text       | {"path": [2, 29, 0], "text": "console.log('wave7');"}                                                                                                                                     |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🌐️zukunft-bau-entwerfen-mit-bestand.html
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                | params                                                                                                                                                                                     |
      | no-mutation        | {}                                                                                                                                                                                        |
      | set-snapshot       | {"doctype": "DOCTYPE html", "root": {"kind":"element","name":"html","attributes":[{"name":"lang","value":"de"}],"children":[{"kind":"element","name":"head","attributes":[],"children":[{"kind":"element","name":"title","attributes":[],"children":[{"kind":"text","text":"Wave 7 Snapshot Title"}]}]},{"kind":"element","name":"body","attributes":[],"children":[{"kind":"text","text":"Wave 7 snapshot replacement content"}]}]}} |
      | set-doctype        | {"doctype": "DOCTYPE htmlWave7"}                                                                                                                                                          |
      | insert-node        | {"parent": [2], "index": 0, "node": {"kind":"element","name":"div","attributes":[{"name":"id","value":"wave7-marker"}],"children":[{"kind":"text","text":"Wave 7 mutation testing"}]}}    |
      | remove-node        | {"parent": [2], "index": 9}                                                                                                                                                               |
      | set-element-name   | {"path": [2, 9], "name": "aside"}                                                                                                                                                         |
      | set-attribute      | {"path": [2, 9], "name": "class", "value": "sidebars-wave7"}                                                                                                                              |
      | set-text           | {"path": [0, 9, 0], "text": "Wave 7 Mutation Testing"}                                                                                                                                    |
      | set-comment        | {"path": [0, 5], "text": " Wave 7 replaced comment "}                                                                                                                                     |
      | set-raw-text       | {"path": [2, 29, 0], "text": "console.log('wave7');"}                                                                                                                                     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🌐️zukunft-bau-entwerfen-mit-bestand.html
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
