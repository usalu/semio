@capability-semio-v1-image-mutate
@oracle-semio-image-python-pillow-independent
@comparison-ordered-json-v1
@mutations-semio-v1-image
Feature: Apply every typed semio IMAGE mutation to a real animated raster, against Pillow and an independent Python implementation
  Two independent producers answer this case, and each answers the half it genuinely speaks.

  **Pillow (PIL 11.3, littleCMS) is a real third party and it speaks the PAYLOAD.** Every RGBA8
  sample under test was decoded out of a real committed animated GIF by Pillow; the ICC profile
  `set-icc` attaches is a real sRGB profile littleCMS emitted; and every scenario that produces
  planes hands them back to Pillow, which restates the mode, the geometry, the per-band extrema and
  the distinct-colour count of what it is given. The Rust subject computes those same four facts
  from the same planes by hand, so a projection only matches when a raster library that has never
  seen this repository and this repository's own codec agree about the actual samples. What Pillow
  does NOT do is read `.dsl.semio` or hold an opinion about a mutation verb, and that boundary is
  named here rather than blurred.

  **`🐍️component.py` beside this file is the second IMPLEMENTATION, for the half no third party
  speaks.** The `s.stdio.semio.image` carrier and its thirteen verbs are semio's own, so the second
  producer a differential comparison needs is a second implementation, written in Python from the
  committed specification documents alone: the DSL body from
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  the pack frame from `…/📸️snapshot/💾️binary/📡️component.protocol.semio` and its Kaitai mirror
  `…/💾️binary/🥋️component.ksy` — which for this subset describes the trailing chain completely, so
  nothing had to be reverse-engineered from bytes — the envelope from
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`, and the verbs from
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`, `…/🧬️mutations/🔣️.json` and
  the committed per-kind specification vectors. It imports nothing from and transliterates nothing of
  the Rust it judges, and it was pinned before use: it reproduces the committed
  `✳️any/📚️examples/🖼️swatch` artifact byte for byte in BOTH encodings and reaches all twelve
  committed after-snapshots. It is registered as the oracle
  `semio-image-python-pillow-independent`; the recorded no-oracle decision it replaces is gone,
  because a reference now exists.

  **The document under test is real, and its provenance is written down.** `local://🗣️.dsl.semio`
  and its binary twin were derived ONCE from the real committed animated GIF
  `🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️color-animated-text.gif` — 194x84, GIF89a, 16 frames at
  130 ms, NETSCAPE2.0 looping, palette-indexed — by decoding its first three frames with Pillow at
  NATIVE resolution: no resampling, no cropping, no colour conversion beyond the palette resolution
  to RGBA8 the semio image model requires. The declared `colorspace` is `indexed` and the `bitDepth`
  8 because that is what the source genuinely is, the delays are the file's own, and every metadata
  entry states a real fact about the source file, including a German one carrying an en dash and a
  multiplication sign so the hex-encoded UTF-8 leaves are exercised by real text. That is 391 703
  bytes of DSL over 195 552 real samples, which is what puts the pack frame's length prefixes past
  one LEB128 byte — a boundary the committed 2x2 swatch never reaches. The derivation script and its
  provenance note live in the ticket folder, and `asset://` cannot leave this artifact's root, which
  is why the derived document is committed here as a case fixture rather than borrowed in place.

  The parameters are chosen against the artifact's own shape, so a plausible wrong codec fails:
  `set-dimensions` TRANSPOSES 194x84 to 84x194, which leaves the plane length valid and therefore
  catches only a codec that confuses the two axes; `insert-frame` puts the animation's real thirteenth
  frame in the MIDDLE rather than appending; `remove-frame` drops the middle frame; `move-frame` lifts
  the last frame to the front; `set-frame-pixels` repaints the third frame with the real sixteenth frame of
  the same animation, so a no-op write is visible; `set-metadata-entry` rewrites a key that already
  exists and is NOT last, so an implementation that appends instead of rewriting in place fails; and
  `set-icc` attaches 588 real profile bytes where the document carried none, exercising both arms of
  the `option-hex` production.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each of the twelve kinds that owns
  a leaf, applied now by BOTH implementations and checked against the committed after-snapshot by
  each of them in role. Nothing was removed to make room for the oracle. The thirteenth kind,
  `no-mutation`, is nullary and owns no leaf; its vector is the identity against a committed
  before-snapshot, with the payload in the scenario's own doc string.

  `identity-round-trip` carries the BYTE half of the identity law. `.dsl.semio` is a fixed-layout
  record grammar and `.pack.semio` is its binary twin, so reproducing both committed files byte for
  byte is the CORRECT answer here and the wave's must-differ tripwire would be backwards — which is
  why the Rust side asserts `law::carrier_is_exact`. What stops that being a codec agreeing with
  itself is that the Python side reproduces the same two files from the grammar alone, in another
  language, and the two sides' digests of the re-emitted bytes are compared.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived animation
    Given the real derived image artifact local://🗣️.dsl.semio
    And the committed mutation payload local://🧫️<id>/🦠️mutation/🔣️.json
    When the <id> mutation is applied to the animation parsed from it
    Then the independent implementation and the subject agree on the resulting snapshot and on what Pillow says its planes are
    Examples:
      | id                    |
      | no-mutation           |
      | set-snapshot          |
      | set-dimensions        |
      | set-colorspace        |
      | set-bit-depth         |
      | set-icc               |
      | insert-frame          |
      | remove-frame          |
      | move-frame            |
      | set-frame-delay       |
      | set-frame-pixels      |
      | set-metadata-entry    |
      | remove-metadata-entry |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real derived animation
    Given the real derived image artifact local://🗣️.dsl.semio
    And the committed mutation payload local://🧫️<id>/🦠️mutation/🔣️.json
    When the <id> mutation is applied to the animation parsed from it and each side undoes it with its own computed inverse
    Then both sides restore the animation and agree on the mutated and the restored snapshot
    Examples:
      | id                    |
      | no-mutation           |
      | set-snapshot          |
      | set-dimensions        |
      | set-colorspace        |
      | set-bit-depth         |
      | set-icc               |
      | insert-frame          |
      | remove-frame          |
      | move-frame            |
      | set-frame-delay       |
      | set-frame-pixels      |
      | set-metadata-entry    |
      | remove-metadata-entry |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/➡️after/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                    | dir                     | slug                                                        |
      | set-snapshot          | 📸️set-snapshot          | retargets-the-document-onto-a-grayscale-sixteen-bit-variant |
      | set-dimensions        | 📐️set-dimensions        | widens-the-frameless-canvas-to-four-by-two                  |
      | set-colorspace        | 🌈️set-colorspace        | records-the-source-colorspace-as-rgba                       |
      | set-bit-depth         | 🔢️set-bit-depth         | raises-the-source-bit-depth-to-sixteen                      |
      | set-icc               | 🎨️set-icc               | attaches-an-icc-profile-where-there-was-none                |
      | insert-frame          | ➕️insert-frame          | appends-a-second-frame-at-the-end                           |
      | remove-frame          | 📄remove-frame           | removes-the-leading-frame                                   |
      | move-frame            | 🔀️move-frame            | moves-the-last-frame-to-the-front                           |
      | set-frame-delay       | ⏱️set-frame-delay       | slows-the-second-frame-down                                 |
      | set-frame-pixels      | 🟪️set-frame-pixels      | repaints-the-only-frame-black                               |
      | set-metadata-entry    | 🏷️set-metadata-entry    | rewrites-the-existing-author-entry                          |
      | remove-metadata-entry | 🗑️remove-metadata-entry | removes-the-comment-entry-and-keeps-the-author-entry        |

  @id-spec-vector-no-mutation
  @level-exhaustive
  @mode-differential
  Scenario: no-mutation leaves the committed three-frame vector exactly as it stands
    Given the committed before-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/🔀️move-frame/🧪️tests/moves-the-last-frame-to-the-front/📸️snapshot/⬅️before/🔣️.json
    When both implementations apply the nullary mutation to it
      """
      {"mutation": "noMutation"}
      """
    Then each reaches the before-snapshot again and the two agree

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both committed encodings of the real derived animation from the parsed document
    Given the real derived image artifact local://🗣️.dsl.semio
    And its committed binary twin local://🎒️.pack.semio
    When each implementation parses the text artifact, prints it back, decodes the binary twin and re-encodes it
    Then both reproduce the two committed files byte for byte, agree on the animation and on the digests of what they emitted, and agree with Pillow about the planes
