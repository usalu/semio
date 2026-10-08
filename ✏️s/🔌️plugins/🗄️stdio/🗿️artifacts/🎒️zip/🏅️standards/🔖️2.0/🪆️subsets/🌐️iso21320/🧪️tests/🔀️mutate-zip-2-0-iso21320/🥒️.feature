@capability-zip-2-0-iso21320-mutate
@oracle-zip-2-0-iso21320-mutate
@comparison-semantic-zip-iso21320-v1
@mutations-zip-2-0-iso21320
Feature: Apply every typed ISO/IEC 21320-1 mutation to a real-world document container
  The input is `shared://🗜️.zip`, the real 20-entry, ~1.53 MB archive of real
  architecture photographs this artifact already commits for its `🧱️base` case. It is used here
  because it is genuinely an ISO/IEC 21320-1:2015 container and not merely a ZIP: all 20 members are
  Deflate-compressed (method 8, admitted by §4.4), none carries the encryption bit (§4.1 forbids it),
  and the archive carries a real 88-byte EOCD comment. Verified by reading the central directory, not
  assumed.

  This is `🌐️iso21320`'s own vocabulary, not `🧱️base`'s. The profile IS a restriction of the
  compression method: §4.4 admits exactly Stored (0) and Deflate (8) out of the twenty-odd methods
  APPNOTE defines. `🧱️base`'s `add-entry` declares no method at all — whichever one a member ends up
  with on the wire is a consequence of the canonical serializer's filename-extension policy — so this
  subset splits it into `add-stored-entry` and `add-deflated-entry`, and its `ZipIso21320Method` type
  makes every non-admitted method unrepresentable. The subset's own production builder already
  declared that distinction as `with_stored_entry`/`with_deflate_entry`; until this ticket the two
  were byte-identical functions that both called `🧱️base`'s ungated `AddEntry`, which is now fixed.

  The shared `ZipSnapshot` now carries each member's native header facet: a `ZipEntry` is
  `{name, data, metadata}`, and `metadata.compressionMethod` is the method `encode_zip` writes. Every
  `params` cell below is the leaf's own wire payload in exactly that shape — a member's data is its
  byte array, a stored member declares method 0 and a deflated one method 8, and `add-stored-entry`/
  `add-deflated-entry` stamp the method from their own kind whatever the carried metadata says. The
  registered `zip` reference writer reads the same wire and honours the method per member through
  `ZipWriter::start_file`.

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

  Both implementations read one wire: the oracle by field name, the subject through
  `Mutation::from_payload_value`, whose re-emitted payload must equal the row exactly; the subject
  undoes every kind with `Mutation::inverse` itself.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real container
    Given the real input archive shared://🗜️.zip
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                  | params |
      | set-archive-comment | {"comment":"Zwischenbericht Projektfotos, ISO/IEC 21320-1 Stand Mutation","commentUtf8":true} |
      | add-stored-entry    | {"entry":{"name":"projekt/beleg.png","data":[80,83,69,85,68,79,45,80,78,71,45,66,69,76,69,71,58,32,117,110,107,111,109,112,114,105,109,105,101,114,116,32,97,98,103,101,108,101,103,116,46],"metadata":{"compressionMethod":0,"local":{"versionNeeded":20,"flags":2048,"modifiedTime":0,"modifiedDate":33,"extraFields":[]},"central":{"versionMadeBy":45,"versionNeeded":20,"flags":2048,"modifiedTime":0,"modifiedDate":33,"extraFields":[],"comment":"","internalAttributes":0,"externalAttributes":0},"dataDescriptorSignature":false}}} |
      | add-deflated-entry  | {"entry":{"name":"projekt/notiz.txt","data":[78,97,99,104,116,114,97,103,58,32,119,101,105,116,101,114,101,115,32,66,101,115,116,97,110,100,115,112,114,111,106,101,107,116,32,102,111,108,103,116,46],"metadata":{"compressionMethod":8,"local":{"versionNeeded":20,"flags":2048,"modifiedTime":0,"modifiedDate":33,"extraFields":[]},"central":{"versionMadeBy":45,"versionNeeded":20,"flags":2048,"modifiedTime":0,"modifiedDate":33,"extraFields":[],"comment":"","internalAttributes":0,"externalAttributes":0},"dataDescriptorSignature":false}}} |
      | remove-entry        | {"name":"projekt/P05_recypark_demets.jpg"} |
      | rename-entry        | {"name":"projekt/P10_haus_hos.jpg","newName":"projekt/P10_haus_hos_bestand.jpg"} |
      | set-entry-data      | {"name":"projekt/P08_holbein_gardens.jpg","data":[69,82,83,65,84,90,73,78,72,65,76,84,58,32,66,105,108,100,98,101,108,101,103,32,100,117,114,99,104,32,80,108,97,116,122,104,97,108,116,101,114,116,101,120,116,32,101,114,115,101,116,122,116,46]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real container
    Given the real input archive shared://🗜️.zip
    When the <id> mutation is applied and then undone with its own inverse
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the restored container's semantic projection matches what the original archive's does
    Examples:
      | id                  | params |
      | set-archive-comment | {"comment":"Zwischenbericht Projektfotos, ISO/IEC 21320-1 Stand Mutation","commentUtf8":true} |
      | add-stored-entry    | {"entry":{"name":"projekt/beleg.png","data":[80,83,69,85,68,79,45,80,78,71,45,66,69,76,69,71,58,32,117,110,107,111,109,112,114,105,109,105,101,114,116,32,97,98,103,101,108,101,103,116,46],"metadata":{"compressionMethod":0,"local":{"versionNeeded":20,"flags":2048,"modifiedTime":0,"modifiedDate":33,"extraFields":[]},"central":{"versionMadeBy":45,"versionNeeded":20,"flags":2048,"modifiedTime":0,"modifiedDate":33,"extraFields":[],"comment":"","internalAttributes":0,"externalAttributes":0},"dataDescriptorSignature":false}}} |
      | add-deflated-entry  | {"entry":{"name":"projekt/notiz.txt","data":[78,97,99,104,116,114,97,103,58,32,119,101,105,116,101,114,101,115,32,66,101,115,116,97,110,100,115,112,114,111,106,101,107,116,32,102,111,108,103,116,46],"metadata":{"compressionMethod":8,"local":{"versionNeeded":20,"flags":2048,"modifiedTime":0,"modifiedDate":33,"extraFields":[]},"central":{"versionMadeBy":45,"versionNeeded":20,"flags":2048,"modifiedTime":0,"modifiedDate":33,"extraFields":[],"comment":"","internalAttributes":0,"externalAttributes":0},"dataDescriptorSignature":false}}} |
      | remove-entry        | {"name":"projekt/P05_recypark_demets.jpg"} |
      | rename-entry        | {"name":"projekt/P10_haus_hos.jpg","newName":"projekt/P10_haus_hos_bestand.jpg"} |
      | set-entry-data      | {"name":"projekt/P08_holbein_gardens.jpg","data":[69,82,83,65,84,90,73,78,72,65,76,84,58,32,66,105,108,100,98,101,108,101,103,32,100,117,114,99,104,32,80,108,97,116,122,104,97,108,116,101,114,116,101,120,116,32,101,114,115,101,116,122,116,46]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real container, where bit-stability IS the correct answer
    Given the real input archive shared://🗜️.zip
    When the container is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes reproduce the input exactly, which is the reference writer's own bit-stability on an archive it authored rather than a byte pass-through
