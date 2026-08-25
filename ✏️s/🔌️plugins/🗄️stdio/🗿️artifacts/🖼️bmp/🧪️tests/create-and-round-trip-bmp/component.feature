@capability-bmp-raster
@oracle-image-bmp
@comparison-semantic-raster-v1
Feature: Create and read a BMP image
  BMP v3 is a POSITIONAL format: a 14-byte BITMAPFILEHEADER, a 40-byte BITMAPINFOHEADER, then one
  uncompressed pixel array whose every scanline is padded up to a 4-byte boundary
  (`((width * bpp + 31) / 32) * 4`) and whose rows are stored BOTTOM-UP unless the header's height is
  negative. Nothing in that layout is self-describing — a decoder that miscounts the stride or reads
  the rows in file order rather than image order still produces a plausible image of the right size,
  which is exactly the class of defect these two scenarios exist to catch. The reference `image`
  encoder is handed RGB, because a 24-bit `BI_RGB` bitmap carries no alpha channel at all; our
  `decode_bmp` canonicalizes whatever it finds into an 8-bit RGBA buffer with row 0 = image top, and
  `encode_bmp` writes it back as a 40-byte-header uncompressed 24-bit `BI_RGB` bitmap honouring the
  snapshot's own `row_order`. So `semantic-raster-v1` compares recovered samples, never bytes: the
  re-encoded file is legitimately free to differ from the reference's in row order and in header
  fields the format leaves to the writer.

  The two committed sizes are chosen against those two hazards specifically, and they are not
  interchangeable with any other case's. 4x4 gives a 12-byte scanline, already a multiple of 4, so it
  isolates row ORDER with no padding in play. 5x3 gives a 15-byte scanline that must be padded to 16
  — one dead byte per row, three rows — so a stride computed as `width * 3` reads the image skewed by
  a byte more on every row down, and an odd height leaves no symmetry to hide a flipped image behind.

  @id-gradient-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A 4x4 gradient survives decode and re-encode with no row padding in play
    Given the image
    """
    { "width": 4, "height": 4 }
    """
    When the image is written and read back
    Then the recovered image matches the reference

  @id-non-square-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A 5x3 gradient survives the 4-byte scanline padding and the odd height
    Given the image
    """
    { "width": 5, "height": 3 }
    """
    When the image is written and read back
    Then the recovered image matches the reference
