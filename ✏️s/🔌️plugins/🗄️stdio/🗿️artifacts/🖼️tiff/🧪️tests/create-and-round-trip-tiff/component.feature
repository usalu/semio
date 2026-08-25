@capability-tiff-raster
@oracle-image-tiff
@comparison-semantic-raster-v1
Feature: Create and read a TIFF image
  TIFF 6.0 is the opposite of a positional format: an 8-byte header carrying a byte-order mark (`II`
  or `MM`) and an offset to the first Image File Directory, then a CHAIN of directories, each a
  sorted list of typed tag entries ending in an offset to the next. Geometry lives in the
  `ImageWidth`/`ImageLength` tags, samples in `StripOffsets`/`StripByteCounts`, and every value
  longer than four bytes is stored out-of-line behind a file offset. Almost nothing about the
  encoding is fixed: byte order, strip count, tag placement and directory position are all the
  writer's choice, so two conforming TIFFs of the same picture routinely share no byte at all. That
  is why `semantic-raster-v1` compares recovered samples here rather than bytes — and unlike BMP the
  reference `image` encoder is handed the full RGBA buffer, because TIFF carries four samples per
  pixel natively and the alpha channel is part of what must survive.

  What the round trip therefore exercises is our own `decode_tiff`, which walks the WHOLE next-IFD
  chain and decodes every entry through the generic TIFF 6.0 field-type table — including tags this
  codec assigns no meaning to — and `encode_tiff`, which re-emits that chain, recomputing IFD 0's
  strip and geometry tags from the pixel buffer while carrying every other tag verbatim and
  preserving the snapshot's own recorded byte order. A decoder that read only IFD 0, or that resolved
  an out-of-line value against the wrong base, still yields a well-formed image; the assertion is
  that the samples come back identical anyway.

  The two committed sizes are this case's own. 4x4 is the smallest picture whose 64-byte RGBA payload
  is already too long to inline anywhere and must be reached through `StripOffsets`. 8x4 makes width
  and height differ so that `ImageWidth` and `ImageLength` cannot be confused for one another — a
  transposition a square picture would silently absorb.

  @id-gradient-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A 4x4 RGBA gradient survives decode and re-encode through the tag directory
    Given the image
    """
    { "width": 4, "height": 4 }
    """
    When the image is written and read back
    Then the recovered image matches the reference

  @id-non-square-round-trips
  @level-quick
  @mode-round-trip
  Scenario: An 8x4 image proves ImageWidth and ImageLength are not transposed
    Given the image
    """
    { "width": 8, "height": 4 }
    """
    When the image is written and read back
    Then the recovered image matches the reference
