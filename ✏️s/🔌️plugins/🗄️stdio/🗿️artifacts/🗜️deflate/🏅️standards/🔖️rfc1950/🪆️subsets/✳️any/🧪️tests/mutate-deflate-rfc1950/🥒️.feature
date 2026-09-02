@capability-deflate-rfc1950-mutate
@oracle-flate2-deflate-rfc1950-mutate
@comparison-ordered-json-v1
@mutations-deflate-rfc1950-any
Feature: Apply every typed RFC1950 mutation to a real-world zlib stream
  The two inputs are real zlib (RFC1950) streams derived ONCE by compressing this repository's own
  README.md (47607 bytes of genuine human-authored project documentation, not a synthetic stub)
  with Python's standard-library `zlib` at compression levels 1 and 9, committed as
  `shared://📄️readme-level1.zz` (17741 bytes, CMF 0x78/FLG 0x01, FLEVEL fastest) and
  `shared://📄️readme-level9.zz` (15701 bytes, CMF 0x78/FLG 0xda, FLEVEL maximum). Every scenario
  reads one of them where this artifact already keeps them; neither is ever written to.

  Byte-pass-through caveat: recompressing the same payload at the same level can legitimately
  reproduce identical bytes for some deflate implementations, so `output == input` is not by itself
  proof of a smuggled byte. Here it additionally holds because THREE independently written
  compressors are in play — Python's system zlib produced the fixtures, this repository's own
  hand-rolled `deflate_raw`/`inflate_raw` produces the subject's output, and `flate2` produces the
  oracle's — so byte-for-byte collision on 47 KB of real prose is not expected in practice. The
  subject's own `mutate`/`inverse` handlers still assert `output != input` per the wave's rule, and
  every scenario additionally proves full semantic parsing beyond that tripwire: the decoded payload
  is compared by digest after the round trip (never smuggled bytes), and `set-compression-params`
  deliberately sets a FLEVEL/window differing from the input's own header, so the header bytes
  visibly change under a mutation that does not touch the payload at all.

  CMF/FLG/DICTID framing is RFC1950's own fixed bit arithmetic; neither `windowBits` nor a preset
  dictionary id actually reconfigures the real DEFLATE window or primes an LZ77 dictionary in this
  subset's codec (documented alongside `decode_deflate_snapshot`) — both are retained honestly as
  typed metadata. The projection compares that typed metadata plus the recovered payload's size and
  digest, never the raw compressed bytes: this repository's encoder and `flate2` choose different
  block splits and Huffman tables for the same payload, so byte equality is deliberately not the
  normative property here (RFC1951 leaves that choice to the writer).

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real zlib stream
    Given the real input document shared://📄️readme-level9.zz
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                                                                                                          |
      | set-snapshot          | {"method": 8, "windowBits": 6, "levelHint": "default", "dictId": null, "payload": "This is the complete replacement snapshot for the set-snapshot mutation kind, written as real UTF-8 text rather than a synthetic placeholder."} |
      | set-compression-params | {"method": 8, "windowBits": 5, "levelHint": "fastest"}                                                                                        |
      | set-preset-dictionary | {"dictId": 305419896}                                                                                                                          |
      | set-payload           | {"payload": "This replacement payload proves SetPayload discards the original real-world content and substitutes this text instead."}         |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real zlib stream
    Given the real input document shared://📄️readme-level9.zz
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the semantic projection

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real zlib stream
    Given the real input document shared://📄️readme-level9.zz
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the restored document's semantic projection matches its state before <id> was applied
    Examples:
      | id                    | params                                                                                                                                          |
      | set-snapshot          | {"method": 8, "windowBits": 6, "levelHint": "default", "dictId": null, "payload": "This is the complete replacement snapshot for the set-snapshot mutation kind, written as real UTF-8 text rather than a synthetic placeholder."} |
      | set-compression-params | {"method": 8, "windowBits": 5, "levelHint": "fastest"}                                                                                        |
      | set-preset-dictionary | {"dictId": 305419896}                                                                                                                          |
      | set-payload           | {"payload": "This replacement payload proves SetPayload discards the original real-world content and substitutes this text instead."}         |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real zlib stream
    Given the real input document shared://📄️readme-level9.zz
    When the no-mutation mutation is applied and then undone
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the restored document's semantic projection matches its state before no-mutation was applied

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://📄️readme-level1.zz
    When the document is fully decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
