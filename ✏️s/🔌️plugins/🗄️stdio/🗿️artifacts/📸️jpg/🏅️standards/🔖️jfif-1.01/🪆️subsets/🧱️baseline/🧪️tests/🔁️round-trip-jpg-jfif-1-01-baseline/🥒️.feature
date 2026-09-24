@capability-jpg-jfif-1-01-baseline-round-trip
@no-oracle-jpg-jfif-1-01-baseline-round-trip-normalization
@comparison-ordered-json-v1
Feature: Decode and re-encode a real photographic JPEG without leaving the T.81 baseline class
  The input is the real architectural scan the `🧾️document` and `🧱️baseline` mutation cases read.
  This repository's encoder writes a conforming baseline file with its own Annex K tables at its own
  quality, so no third-party writer can reproduce its bytes and none is asked to (the
  `jpg-jfif-1-01-baseline-round-trip-normalization` decision). The scenario judges the round trip by
  laws — the bytes were re-derived rather than copied, the result is still inside the baseline class,
  the conformance projection survives — and by the INDEPENDENT `image` reader the `🧾️document`
  subset registers, which must see the same geometry on both sides.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real scan without passing bytes through
    Given the real input document shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the scan is decoded into a snapshot and re-serialized from that snapshot alone
    Then the re-encoded bytes differ from the input, the document is still baseline-conforming, and the INDEPENDENT image reader agrees on the geometry of both
