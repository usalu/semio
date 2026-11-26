Feature: Retained Canonical Fixed Huffman Production
  Scenario: Borrowed byte turns reproduce the ordinary compressor
    Given a portable raw byte vector
    When every input byte and output byte is separately funded
    Then the exact fixed Huffman stream equals the ordinary producer
    And system zlib decompresses the stream to the original vector
    And each physical allocation is at most 4096 bytes

  Scenario: Work per input byte is bounded by the matching limits
    Given the chain, nice-length and insert limits of the canonical compressor
    When repetitive, periodic, random and record-like inputs are compressed under exact grants
    Then the quoted copy bytes plus one bookkeeping byte per step stay below the bound the limits derive
    And the stream equals the ordinary producer and inflates to the input through the miniz oxide reader

  Scenario: Refused exact grants preserve every ownership frontier
    Given the next quoted capacity, copy and depth demands
    When any demand is one byte or one level short
    Then no input, output, cursor or physical allocation changes

  Scenario Outline: Cancellation closes all retained ownership
    Given a producer suspended in <phase>
    When cancellation begins the retained close ladder
    Then each exact release receipt equals physically released heap bytes
    And the producer reaches terminal emptiness

    Examples:
      | phase      |
      | initialize |
      | input      |
      | search     |
      | compare    |
      | token      |
      | insert     |
      | finish     |
      | complete   |
