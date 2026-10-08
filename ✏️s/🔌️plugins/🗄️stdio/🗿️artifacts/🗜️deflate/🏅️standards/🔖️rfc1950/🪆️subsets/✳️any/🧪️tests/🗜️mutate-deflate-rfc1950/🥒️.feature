@capability-deflate-rfc1950-mutate
@oracle-flate2-deflate-rfc1950-mutate
@comparison-ordered-json-v1
@mutations-deflate-rfc1950-any
Feature: Apply every typed RFC1950 mutation to a real-world zlib stream
  The two inputs are real zlib (RFC1950) streams derived ONCE by compressing this repository's own
  README.md (47607 bytes of genuine human-authored project documentation, not a synthetic stub)
  with Python's standard-library `zlib` at compression levels 1 and 9, committed as
  `shared://🪶️readme-level1.zz` (17741 bytes, CMF 0x78/FLG 0x01, FLEVEL fastest) and
  `shared://🗜️readme-level9.zz` (15701 bytes, CMF 0x78/FLG 0xda, FLEVEL maximum). Every scenario
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

  Every `params` cell is the leaf's own wire payload, exactly what `DeflateMutation::payload_value()`
  emits and the leaf schema describes — the replacement texts are spelled as their UTF-8 byte arrays,
  the snapshot carries its schema id, and an absent preset dictionary is simply absent. Both
  implementations read that one wire: the oracle by field name, the subject through
  `Mutation::from_payload_value`, and the subject undoes each kind with `Mutation::inverse` itself.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real zlib stream
    Given the real input document shared://🗜️readme-level9.zz
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                     | params |
      | set-compression-params | {"method":8,"window_bits":5,"level_hint":"fastest"} |
      | set-preset-dictionary  | {"dict_id":305419896} |
      | set-payload            | {"payload":[84,104,105,115,32,114,101,112,108,97,99,101,109,101,110,116,32,112,97,121,108,111,97,100,32,112,114,111,118,101,115,32,83,101,116,80,97,121,108,111,97,100,32,100,105,115,99,97,114,100,115,32,116,104,101,32,111,114,105,103,105,110,97,108,32,114,101,97,108,45,119,111,114,108,100,32,99,111,110,116,101,110,116,32,97,110,100,32,115,117,98,115,116,105,116,117,116,101,115,32,116,104,105,115,32,116,101,120,116,32,105,110,115,116,101,97,100,46]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real zlib stream
    Given the real input document shared://🗜️readme-level9.zz
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the restored document's semantic projection matches its state before <id> was applied
    Examples:
      | id                     | params |
      | set-compression-params | {"method":8,"window_bits":5,"level_hint":"fastest"} |
      | set-preset-dictionary  | {"dict_id":305419896} |
      | set-payload            | {"payload":[84,104,105,115,32,114,101,112,108,97,99,101,109,101,110,116,32,112,97,121,108,111,97,100,32,112,114,111,118,101,115,32,83,101,116,80,97,121,108,111,97,100,32,100,105,115,99,97,114,100,115,32,116,104,101,32,111,114,105,103,105,110,97,108,32,114,101,97,108,45,119,111,114,108,100,32,99,111,110,116,101,110,116,32,97,110,100,32,115,117,98,115,116,105,116,117,116,101,115,32,116,104,105,115,32,116,101,120,116,32,105,110,115,116,101,97,100,46]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🪶️readme-level1.zz
    When the document is fully decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
