@capability-csv-rfc4180-mutate
@oracle-csv-rfc4180-mutate
@comparison-semantic-tabular-v1
@mutations-csv-rfc4180-any
Feature: Apply every typed RFC 4180 CSV mutation to a real-world table
  The input is shared://🧪️reuse-marketplaces/📊️.csv, a 50-row, 12-column real research table derived
  ONCE (♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/bauteilboersen.tex → csv, committed here
  verbatim) from a real systematic survey of European building-component reuse marketplaces —
  real platform names, real countries and genuinely comma-laden multi-value cells, so most of the
  50 data rows come back RFC 4180-quoted from the reference writer with no help from this feature.
  Every scenario copies the fixture into the case work directory before touching it; the committed
  file is never written to.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real table
    Given the real input table shared://🧪️reuse-marketplaces/📊️.csv
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"hasHeader": true, "rows": [["Name", "Wert"], ["Testfeld", "Ünïcödé, mit Komma"]]} |
      | set-has-header | {"hasHeader": false} |
      | insert-record | {"index": 5, "fields": ["BB-99", "Marktplätze", "Baustoffbörse Hannover", "Deutschland", "Angebotsübersicht, Detailseite", "öffentlich", "Website", "—", "Beschreibung, Bilder, Preis, Menge, Materialstandort", "Kategorien, Suche, Filter", "Anfrage, Reservierung", "Abholung, Lieferung"]} |
      | remove-record | {"index": 25} |
      | set-field | {"recordIndex": 1, "fieldIndex": 8, "value": "Beschreibung, Bilder, Preis"} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real table
    Given the real input table shared://🧪️reuse-marketplaces/📊️.csv
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original table
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"hasHeader": true, "rows": [["Name", "Wert"], ["Testfeld", "Ünïcödé, mit Komma"]]} |
      | set-has-header | {"hasHeader": false} |
      | insert-record | {"index": 5, "fields": ["BB-99", "Marktplätze", "Baustoffbörse Hannover", "Deutschland", "Angebotsübersicht, Detailseite", "öffentlich", "Website", "—", "Beschreibung, Bilder, Preis, Menge, Materialstandort", "Kategorien, Suche, Filter", "Anfrage, Reservierung", "Abholung, Lieferung"]} |
      | remove-record | {"index": 25} |
      | set-field | {"recordIndex": 1, "fieldIndex": 8, "value": "Beschreibung, Bilder, Preis"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real table without passing bytes through
    Given the real input table shared://🧪️reuse-marketplaces/📊️.csv
    When the table is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
