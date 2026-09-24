@capability-tiff-6-0-baseline-round-trip
@no-oracle-tiff-6-0-baseline-round-trip-normalization
@comparison-ordered-json-v1
Feature: Decode and re-encode a real scanned TIFF without leaving the Adobe TIFF 6.0 Baseline class
  The input is the real scanned TIFF the `✳️any` and `🧱️baseline` mutation cases read, shared rather
  than copied. This repository's encoder regenerates every strip tag from the raster it writes, so a
  third-party writer cannot reproduce its bytes and none is asked to (the
  `tiff-6-0-baseline-round-trip-normalization` decision). The scenario judges the round trip by laws
  instead — the re-encoded bytes equal the reference writer's file, flipping one decoded byte changes
  them, the result is still inside the Baseline class — and by the INDEPENDENT `image` reader the
  `🧾️document` subset registers, which must see the same geometry on both sides.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real scan without passing bytes through
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the scan is decoded into a snapshot and re-serialized from that snapshot alone
    Then the re-encoded bytes reproduce the reference writer's own file exactly, flipping one byte of the decoded raster changes them, the document is still Baseline-conforming, and the INDEPENDENT IFD reader agrees on the geometry of both
