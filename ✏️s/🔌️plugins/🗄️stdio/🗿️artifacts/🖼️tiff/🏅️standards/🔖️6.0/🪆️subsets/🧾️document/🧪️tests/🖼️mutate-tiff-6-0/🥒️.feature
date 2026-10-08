@capability-tiff-6-0-mutate
@oracle-image-tiff-6-0-mutate-reader
@comparison-semantic-raster-v1
@mutations-tiff-6-0-document
Feature: Apply every typed TIFF 6.0 mutation to a real-world document
  The input is a real 500 DPI architectural floor-plan scan
  (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`,
  483 KB, 2275x2560), converted ONCE to TIFF 6.0 with the registered `image` 0.25 reference encoder
  (`image::codecs::tiff::TiffEncoder`) and committed as this artifact's own
  `shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff`. Its second IFD is a genuinely real second
  page — the actual decoded, downsampled (16x16) pixels of the real
  `🏛️rathaus-ahlen-grundriss/🖼️.png` floor plan, appended by this subset's own independent IFD-chain
  writer (`../../🔮️oracles/🦀️.rs`'s `fixture_derivation` module — `image`'s public TIFF
  encoder can only ever emit a single IFD) — so `InsertIfd`/`RemoveIfd`, TIFF's own multi-page
  operations, are substantive on a genuinely multi-IFD document from the very first `Given`, without
  needing a second fixture per row. Every scenario copies the fixture into the case work directory
  before touching it; the committed document is never written to.

  Examples carry canonical authored entries and exact sample-word blocks.
  ASCII values are ordered strings; native policy and storage offsets are chosen at IO.
  The independent reference and subject must preserve the semantic projection through export.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | insert-ifd | {"index":2,"ifd":{"entries":[{"tag":256,"values":{"kind":"long","value":[8]}},{"tag":257,"values":{"kind":"long","value":[8]}},{"tag":258,"values":{"kind":"short","value":[8,8,8]}},{"tag":262,"values":{"kind":"short","value":[2]}},{"tag":277,"values":{"kind":"short","value":[3]}}],"blocks":[{"x":0,"y":0,"width":8,"height":8,"channels":3,"samples":[{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":249,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":249,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":252,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":251,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":247,"hi":0},{"lo":244,"hi":0},{"lo":244,"hi":0},{"lo":248,"hi":0},{"lo":243,"hi":0},{"lo":243,"hi":0},{"lo":249,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":251,"hi":0},{"lo":249,"hi":0},{"lo":249,"hi":0},{"lo":250,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":250,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":253,"hi":0},{"lo":253,"hi":0},{"lo":253,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":250,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":251,"hi":0},{"lo":249,"hi":0},{"lo":249,"hi":0},{"lo":251,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":251,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":249,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":253,"hi":0},{"lo":252,"hi":0},{"lo":252,"hi":0},{"lo":249,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":249,"hi":0},{"lo":245,"hi":0},{"lo":245,"hi":0},{"lo":248,"hi":0},{"lo":244,"hi":0},{"lo":244,"hi":0},{"lo":250,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":248,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":250,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":244,"hi":0},{"lo":244,"hi":0},{"lo":248,"hi":0},{"lo":245,"hi":0},{"lo":245,"hi":0},{"lo":252,"hi":0},{"lo":252,"hi":0},{"lo":252,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":252,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":252,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":248,"hi":0},{"lo":244,"hi":0},{"lo":244,"hi":0},{"lo":249,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0}]}]}} |
      | remove-ifd | {"index":1} |
      | replace-tag | {"ifdIndex":0,"tag":315,"values":{"kind":"ascii","value":["Derived for ticket 26/08/23/END-TO-END-TESTING-REFACTOR"]}} |
      | remove-tag | {"ifdIndex":0,"tag":282} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And its inverse is applied
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | insert-ifd | {"index":2,"ifd":{"entries":[{"tag":256,"values":{"kind":"long","value":[8]}},{"tag":257,"values":{"kind":"long","value":[8]}},{"tag":258,"values":{"kind":"short","value":[8,8,8]}},{"tag":262,"values":{"kind":"short","value":[2]}},{"tag":277,"values":{"kind":"short","value":[3]}}],"blocks":[{"x":0,"y":0,"width":8,"height":8,"channels":3,"samples":[{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":249,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":249,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":252,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":251,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":247,"hi":0},{"lo":244,"hi":0},{"lo":244,"hi":0},{"lo":248,"hi":0},{"lo":243,"hi":0},{"lo":243,"hi":0},{"lo":249,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":251,"hi":0},{"lo":249,"hi":0},{"lo":249,"hi":0},{"lo":250,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":250,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":253,"hi":0},{"lo":253,"hi":0},{"lo":253,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":250,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":254,"hi":0},{"lo":251,"hi":0},{"lo":249,"hi":0},{"lo":249,"hi":0},{"lo":251,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":251,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":249,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":253,"hi":0},{"lo":252,"hi":0},{"lo":252,"hi":0},{"lo":249,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":249,"hi":0},{"lo":245,"hi":0},{"lo":245,"hi":0},{"lo":248,"hi":0},{"lo":244,"hi":0},{"lo":244,"hi":0},{"lo":250,"hi":0},{"lo":247,"hi":0},{"lo":247,"hi":0},{"lo":248,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0},{"lo":250,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":248,"hi":0},{"lo":244,"hi":0},{"lo":244,"hi":0},{"lo":248,"hi":0},{"lo":245,"hi":0},{"lo":245,"hi":0},{"lo":252,"hi":0},{"lo":252,"hi":0},{"lo":252,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":251,"hi":0},{"lo":250,"hi":0},{"lo":250,"hi":0},{"lo":252,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":252,"hi":0},{"lo":251,"hi":0},{"lo":251,"hi":0},{"lo":248,"hi":0},{"lo":244,"hi":0},{"lo":244,"hi":0},{"lo":249,"hi":0},{"lo":246,"hi":0},{"lo":246,"hi":0}]}]}} |
      | remove-ifd | {"index":1} |
      | replace-tag | {"ifdIndex":0,"tag":315,"values":{"kind":"ascii","value":["Derived for ticket 26/08/23/END-TO-END-TESTING-REFACTOR"]}} |
      | remove-tag | {"ifdIndex":0,"tag":282} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the document is decoded and re-encoded with no mutation
    Then the oracle and the subject agree on the semantic projection

  @id-mutate-owned
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> against authentic exact sample words
    Given the real input document shared://🎨️paint-region-applied/⬅️before.tiff
    When the <id> mutation is applied with its canonical parameters
      """
      {"kind":"<id>","params":<params>}
      """
    Then the independent TIFF reader and subject agree on exact metadata and raster samples
    Examples:
      | id | params |
      | paint-region | {"revision":"1354e519afaf27f6","ifdIndex":0,"x":1,"y":1,"width":1,"height":1,"red":9,"green":8,"blue":7,"alpha":255} |
      | replace-samples | {"ifdIndex":0,"block":0,"offset":0,"samples":[{"lo":24,"hi":0},{"lo":96,"hi":0},{"lo":192,"hi":0}]} |

  @id-inverse-owned
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> against authentic exact sample words
    Given the real input document shared://🎨️paint-region-applied/⬅️before.tiff
    When the <id> mutation is applied with its canonical parameters
      """
      {"kind":"<id>","params":<params>}
      """
    And its concrete inverse is applied
    Then the independent TIFF reader and subject agree on exact metadata and raster samples
    Examples:
      | id | params |
      | paint-region | {"revision":"1354e519afaf27f6","ifdIndex":0,"x":1,"y":1,"width":1,"height":1,"red":9,"green":8,"blue":7,"alpha":255} |
      | replace-samples | {"ifdIndex":0,"block":0,"offset":0,"samples":[{"lo":24,"hi":0},{"lo":96,"hi":0},{"lo":192,"hi":0}]} |
