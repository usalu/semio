@capability-zip-2-0-iso21320-mutate
@oracle-zip-2-0-iso21320-mutate
@comparison-semantic-zip-iso21320-v1
@mutations-zip-2-0-iso21320
Feature: Apply every typed ISO/IEC 21320-1 mutation to a real-world document container
  The input is `shared://🎒️zwischenbericht-projekte.zip`, the real 20-entry, ~1.53 MB archive of real
  architecture photographs this artifact already commits for its `✳️any` case. It is used here
  because it is genuinely an ISO/IEC 21320-1:2015 container and not merely a ZIP: all 20 members are
  Deflate-compressed (method 8, admitted by §4.4), none carries the encryption bit (§4.1 forbids it),
  and the archive carries a real 88-byte EOCD comment. Verified by reading the central directory, not
  assumed.

  This is `✳️iso21320`'s own vocabulary, not `✳️any`'s. The profile IS a restriction of the
  compression method: §4.4 admits exactly Stored (0) and Deflate (8) out of the twenty-odd methods
  APPNOTE defines. `✳️any`'s `add-entry` declares no method at all — whichever one a member ends up
  with on the wire is a consequence of the canonical serializer's filename-extension policy — so this
  subset splits it into `add-stored-entry` and `add-deflated-entry`, and its `ZipIso21320Method` type
  makes every non-admitted method unrepresentable. The subset's own production builder already
  declared that distinction as `with_stored_entry`/`with_deflate_entry`; until this ticket the two
  were byte-identical functions that both called `✳️any`'s ungated `AddEntry`, which is now fixed.

  FINDING, recorded rather than worked around. The shared `ZipSnapshot` models a member as
  `{name, data}` and nothing else. It has no slot for a compression method, no general-purpose flag
  bits and no version-needed field — so every constraint the subset's own
  `check_iso21320_conformance` actually checks (the encryption bit, the Strong Encryption bit, the
  trailing data descriptor, the version-needed ceiling) is a WIRE property that no snapshot mutation
  can address, and the method this vocabulary declares is authoritative only for a writer that can
  honour it. The registered `zip` reference implementation can: `ZipWriter::start_file` takes the
  method explicitly. This repository's `encode_zip` cannot; it derives the method from the member's
  filename extension. The subset's `normalize_entry_for_iso21320` composer hook is likewise still a
  declared no-op. Closing that gap means giving `ZipEntry` a native-header facet, a schema change
  outside this vocabulary's scope.

  That finding is exactly why the `semantic-zip-iso21320-v1` profile compares the ISO PREDICATE and
  not the method: §4.4 admits both Stored and Deflate, so which one a writer picks per member is
  writer freedom, and comparing the method itself would compare this repository's own policy against
  a copy of that policy planted in the oracle. What is compared is what ISO fixes — that every
  member's method is one of the two, and that no member is encrypted — alongside the member set by
  name, uncompressed size and content digest, and the archive comment as a normative field.

  Every scenario copies the immutable fixture into the case work directory before touching it; the
  committed archive is never written to.


  A MEASURED CORRECTION to the `@id-identity-round-trip` scenario below, which until this wave
  claimed the re-encoded bytes are not bit-identical to the input. They are — measured, on all
  1,605,927 of them. That is not a byte pass-through: the reference genuinely inflates every member
  (`read_to_end` on a `ZipFile`) and genuinely re-deflates it on the way out. It is bit-stable
  because THIS fixture was itself authored once by that same `zip` reference writer under the same
  default `FileOptions` this round trip re-encodes under — the archive's `1980-01-01` timestamps and
  version-20/Unix headers are that writer's own defaults, not a real archiver's. A must-differ
  assertion here would therefore have been a fabricated law, so the scenario asserts what this
  pairing can honestly claim instead: exact bit-stability plus preservation of the semantic
  projection, both of which fail loudly if the reader, the writer, the compression defaults or the
  entry order ever drift. What proves genuine parsing for this subset is the exhaustive
  `mutate-<kind>` scenarios against the archive's real members.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real container
    Given the real input archive shared://🎒️zwischenbericht-projekte.zip
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                  | params                                                                                                                                                     |
      | no-mutation         | {}                                                                                                                                                         |
      | set-snapshot        | {"entries": [{"name": "manifest/readme.txt", "content": "Ersatzcontainer fuer den ISO 21320-1 Mutationstest.", "method": "deflate"}, {"name": "manifest/beleg.png", "content": "PSEUDO-PNG-BELEG", "method": "stored"}], "comment": "Ersatzcontainer"} |
      | set-archive-comment | {"comment": "Zwischenbericht Projektfotos, ISO/IEC 21320-1 Stand Mutation"}                                                                                |
      | add-stored-entry    | {"name": "projekt/beleg.png", "content": "PSEUDO-PNG-BELEG: unkomprimiert abgelegt."}                                                                      |
      | add-deflated-entry  | {"name": "projekt/notiz.txt", "content": "Nachtrag: weiteres Bestandsprojekt folgt."}                                                                      |
      | remove-entry        | {"name": "projekt/P05_recypark_demets.jpg"}                                                                                                                |
      | rename-entry        | {"name": "projekt/P10_haus_hos.jpg", "newName": "projekt/P10_haus_hos_bestand.jpg"}                                                                        |
      | set-entry-data      | {"name": "projekt/P08_holbein_gardens.jpg", "content": "ERSATZINHALT: Bildbeleg durch Platzhaltertext ersetzt."}                                           |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real container
    Given the real input archive shared://🎒️zwischenbericht-projekte.zip
    When the <id> mutation is applied and then undone with its own inverse
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the restored container's semantic projection matches what the original archive's does
    Examples:
      | id                  | params                                                                                                                                                     |
      | no-mutation         | {}                                                                                                                                                         |
      | set-snapshot        | {"entries": [{"name": "manifest/readme.txt", "content": "Ersatzcontainer fuer den ISO 21320-1 Mutationstest.", "method": "deflate"}, {"name": "manifest/beleg.png", "content": "PSEUDO-PNG-BELEG", "method": "stored"}], "comment": "Ersatzcontainer"} |
      | set-archive-comment | {"comment": "Zwischenbericht Projektfotos, ISO/IEC 21320-1 Stand Mutation"}                                                                                |
      | add-stored-entry    | {"name": "projekt/beleg.png", "content": "PSEUDO-PNG-BELEG: unkomprimiert abgelegt."}                                                                      |
      | add-deflated-entry  | {"name": "projekt/notiz.txt", "content": "Nachtrag: weiteres Bestandsprojekt folgt."}                                                                      |
      | remove-entry        | {"name": "projekt/P05_recypark_demets.jpg"}                                                                                                                |
      | rename-entry        | {"name": "projekt/P10_haus_hos.jpg", "newName": "projekt/P10_haus_hos_bestand.jpg"}                                                                        |
      | set-entry-data      | {"name": "projekt/P08_holbein_gardens.jpg", "content": "ERSATZINHALT: Bildbeleg durch Platzhaltertext ersetzt."}                                           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real container, where bit-stability IS the correct answer
    Given the real input archive shared://🎒️zwischenbericht-projekte.zip
    When the container is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes reproduce the input exactly, which is the reference writer's own bit-stability on an archive it authored rather than a byte pass-through
