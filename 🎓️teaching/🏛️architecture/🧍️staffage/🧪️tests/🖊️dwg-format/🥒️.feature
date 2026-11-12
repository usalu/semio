Feature: The DWG writer owns its format code and agrees with independent implementations
  Scenario: Checksums, masks and the magic sequence equal acadrust's
    Given the bytes of a page, of a header and of a 70 000 byte section
    When CRC-16, CRC-32, the page checksum, the magic sequence and the header mask are computed
    Then every value equals the one acadrust computes

  Scenario: Compressed pages decompress to their input
    Given data of zeros, of one repeated byte, of mixed runs and literals, and of 100 000 pseudo-random bytes
    When it is compressed and decompressed with acadrust's AC18 decompressor
    Then the original bytes come back and a zero page of 29 696 bytes shrinks below 200 bytes

  Scenario: Bit codes are read back by an independent decoder
    Given a pseudo-random sequence of B, BB, BS, BL, BD, DD, RC, RS, RL, RD, H, CMC and OT values
    When they are written and read with acadrust's bit reader
    Then every value equals the written one

  Scenario: The JSON reader equals serde_json
    Given documents with escapes, surrogate pairs, exponents, nesting and invalid input
    When they are parsed by this crate and by serde_json
    Then accepted documents yield equal values and rejected ones are rejected by both

  Scenario: The container is valid for a second independent reader
    Given the model scene fixture, the sheet fixture and a scene of 4 000 polylines
    When ezdwg-core opens the file
    Then it reports AC1024, the header variables (metre units, extents), one object record per written handle, and every record parses with a valid size
