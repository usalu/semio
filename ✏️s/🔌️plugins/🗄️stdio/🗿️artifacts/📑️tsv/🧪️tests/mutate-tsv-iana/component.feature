@capability-tsv-iana-mutate
@oracle-tsv-iana-mutate
@comparison-semantic-tabular-mutate-v1
@mutations-tsv-iana-any
Feature: Apply every typed IANA TSV mutation to a real-world table
  The input is shared://reuse-marketplaces.tsv, a 51-record, 12-column real research table derived
  ONCE through a real provenance chain: a genuine systematic survey of European building-component
  reuse marketplaces (♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/bauteilboersen.tex) was
  first committed as RFC 4180 CSV at
  ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv by the csv rfc4180
  wave, and THIS fixture is derived from that same committed CSV, ONE more hop, with the identical
  `csv` reference crate reconfigured for IANA text/tab-separated-values (tab delimiter, quoting
  disabled entirely on both read and write — see the ticket's own
  tsv-iana-any-fixture-gen/src/main.rs). IANA TSV (unlike RFC 4180) has NO quoting or escaping
  mechanism, so a field that genuinely contained a literal tab or newline byte would be
  unrepresentable; the derivation program scanned every one of the source's 51×12 real cells for
  both before writing anything and found NEITHER (real German survey text is comma-laden — many
  cells hold multi-value lists like "Beschreibung, Bilder, Preis, Menge, Materialstandort" — but a
  comma is not TSV's delimiter, so those commas simply carry over as ordinary characters, no policy
  decision required). The committed result is LF-terminated with a trailing newline.

  IANA TSV also draws no header/data distinction (unlike RFC 4180's optional convention), so this
  subset's own real serialization concerns — which line ending the file uses, and whether it ends
  in a trailing line break — take that role instead: `set-trailing-newline` and `set-line-ending`
  genuinely rewrite bytes on this real table, and the `semantic-tabular-mutate-v1` comparison
  profile keeps both fields live (unlike the base `semantic-tabular-v1` profile, which treats RFC
  4180's writer-freedom equivalents as ignorable).

  Every scenario copies the fixture into the case work directory before touching it; the committed
  file is never written to.

  A note on the `@id-identity-round-trip` scenario below, which in most OTHER cases this wave
  asserts the re-encoded bytes are NOT bit-identical to the input (proof that real parsing, not a
  byte-copy shortcut, produced them): for IANA TSV that assertion would be dishonest. The format has
  no quoting and no escaping mechanism at all, so a writer has nothing to choose; the only two
  choices it does leave — which line terminator the file uses and whether the last record is
  terminated — are exactly the two fields this subset's `TsvBody`/`TsvSnapshot` carry and reproduce
  verbatim. Decode→encode reproducing the input byte-for-byte is therefore the CORRECT outcome here,
  and the oracle module's own `no_mutation_is_a_true_byte_identity` test states the same property
  independently. The scenario asserts that exact-byte law instead of a fabricated must-differ one,
  and what proves genuine parsing for this subset is the exhaustive `mutate-<kind>` scenarios: you
  cannot insert, remove or replace row 25 of a real 51-record table without actually having parsed
  it.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real table
    Given the real input table shared://reuse-marketplaces.tsv
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params |
      | no-mutation          | {} |
      | set-snapshot         | {"records": [["Name", "Wert"], ["Testfeld", "Ünïcödé ohne Tab"]], "trailingNewline": true, "lineEnding": "lf"} |
      | set-trailing-newline | {"trailingNewline": false} |
      | set-line-ending      | {"lineEnding": "crlf"} |
      | insert-row           | {"index": 5, "row": ["BB-99", "Marktplätze", "Baustoffbörse Hannover", "Deutschland", "Angebotsübersicht, Detailseite", "öffentlich", "Website", "—", "Beschreibung, Bilder, Preis, Menge, Materialstandort", "Kategorien, Suche, Filter", "Anfrage, Reservierung", "Abholung, Lieferung"]} |
      | remove-row           | {"index": 25} |
      | set-cell             | {"rowIndex": 1, "fieldIndex": 8, "value": "Beschreibung, Bilder, Preis"} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real table
    Given the real input table shared://reuse-marketplaces.tsv
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original table
    Examples:
      | id                   | params |
      | no-mutation          | {} |
      | set-snapshot         | {"records": [["Name", "Wert"], ["Testfeld", "Ünïcödé ohne Tab"]], "trailingNewline": true, "lineEnding": "lf"} |
      | set-trailing-newline | {"trailingNewline": false} |
      | set-line-ending      | {"lineEnding": "crlf"} |
      | insert-row           | {"index": 5, "row": ["BB-99", "Marktplätze", "Baustoffbörse Hannover", "Deutschland", "Angebotsübersicht, Detailseite", "öffentlich", "Website", "—", "Beschreibung, Bilder, Preis, Menge, Materialstandort", "Kategorien, Suche, Filter", "Anfrage, Reservierung", "Abholung, Lieferung"]} |
      | remove-row           | {"index": 25} |
      | set-cell             | {"rowIndex": 1, "fieldIndex": 8, "value": "Beschreibung, Bilder, Preis"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real table, where byte identity IS the correct answer
    Given the real input table shared://reuse-marketplaces.tsv
    When the table is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are bit-identical to the input, which is this format's total absence of writer freedom working correctly rather than a byte pass-through
