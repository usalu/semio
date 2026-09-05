@capability-zlib-compression
@oracle-flate2
@comparison-ordered-json-v1
Feature: Compress and inflate a zlib stream
  RFC 1950 fixes the container and RFC 1951 the deflate encoding, but neither fixes WHICH encoding a
  compressor chooses — level, block splitting and Huffman table selection are all writer freedom.
  The normative property is therefore the round trip: whatever this repository emits, the reference
  inflater must recover the exact input, and whatever the reference emits, this repository must
  recover the exact input. Byte-equal output is deliberately NOT asserted.

  @id-inflates-what-the-reference-deflated
  @level-quick
  @mode-round-trip
  Scenario: The reference stream inflates to the exact input
    Given the payload
    """
    the quick brown fox jumps over the lazy dog, and then does it again and again and again
    """
    When the payload is compressed and inflated again
    Then the recovered bytes are identical to the input

  @id-handles-an-empty-payload
  @level-quick
  @mode-round-trip
  Scenario: An empty payload round trips
    Given the payload
    """
    """
    When the payload is compressed and inflated again
    Then the recovered bytes are identical to the input

  @id-handles-incompressible-bytes
  @level-quick
  @mode-round-trip
  Scenario: A payload with no repetition still round trips
    Given the payload
    """
    abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ
    """
    When the payload is compressed and inflated again
    Then the recovered bytes are identical to the input
