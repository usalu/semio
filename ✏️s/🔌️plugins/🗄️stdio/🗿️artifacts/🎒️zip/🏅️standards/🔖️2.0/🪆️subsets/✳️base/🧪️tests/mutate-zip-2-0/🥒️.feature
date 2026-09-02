@capability-zip-2-0-mutate
@oracle-zip-2-0-mutate
@comparison-semantic-archive-mutate-v1
@mutations-zip-2-0-base
Feature: Apply every typed ZIP 2.0 mutation to a real-world multi-entry archive
  The input is `shared://🗜️.zip`, a real 20-entry, ~1.53 MB archive derived
  ONCE (not a test step) with the `zip` reference library from 20 of the 67 real architecture
  photographs committed at `♻️mit-bestand/📋️bericht/📋️zwischenbericht/asset/projekt/`, no genuine
  multi-entry ZIP being reachable outside the forbidden `compose/` tree. Every member keeps its
  source filename under a `projekt/` prefix; the archive comment is
  "Zwischenbericht Projektbeispiele – 20 von 67 realen Bestandsarchitektur-Referenzfotos." Exact file
  list, in archive order (`projekt/<name>`):
  P01_k118_kopfbau_halle_118.jpg, P02_bedzed.jpg, P03_biopartner_5.jpg, P04_ka13.jpg,
  P05_recypark_demets.jpg, P06_svanen_kindergarten.jpg, P07_villa_welpeloo.jpg,
  P08_holbein_gardens.jpg, P09_werkhof_29.jpg, P10_haus_hos.jpg, P11_mehrow_pilot_house.jpg,
  P12_broethen_twin_house.jpg, P13_crclr_house.jpg, P14_recyclinghaus_hannover.jpg,
  P15_thoravej_29.jpg, P16_timber_square.jpg, P17_tbc_london.jpg, P18_55_great_suffolk_street.jpg,
  P19_brent_cross_town_substation.jpg, P20_boulder_fire_station_3.jpg — the alphabetically-first 20
  of the 67 real project photographs beneath that directory (total source bytes: 1,613,500).

  Every scenario copies the immutable fixture into the case work directory before touching it; the
  committed archive is never written to. `AddEntry`/`RemoveEntry`/`RenameEntry` are exercised against
  this real archive's real members — never a synthetic two-entry stand-in — so they are the archive
  analogue of a genuine add/remove/move, not a no-op.

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
  Scenario Outline: Apply <id> to the real archive
    Given the real input archive shared://🗜️.zip
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection

    Examples:
      | id                  | params                                                                                                                 |
      | set-snapshot         | {"entries": [{"name": "manifest/readme.txt", "content": "Ersatzarchiv fuer den Mutationstest."}, {"name": "manifest/index.txt", "content": "Eintraege: 1"}], "comment": "Ersatzarchiv"} |
      | set-archive-comment  | {"comment": "Zwischenbericht Projektfotos, Stand Mutation"}                                                           |
      | add-entry            | {"name": "projekt/notiz.txt", "content": "Nachtrag: weiteres Bestandsprojekt folgt."}                                 |
      | remove-entry         | {"name": "projekt/P05_recypark_demets.jpg"}                                                                           |
      | rename-entry         | {"name": "projekt/P10_haus_hos.jpg", "newName": "projekt/P10_haus_hos_bestand.jpg"}                                   |
      | set-entry-data       | {"name": "projekt/P08_holbein_gardens.jpg", "content": "ERSATZINHALT: Bildbeleg durch Platzhaltertext ersetzt."}      |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real archive
    Given the real input archive shared://🗜️.zip
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the semantic projection

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the archive
    Given the real input archive shared://🗜️.zip
    When the <id> mutation is applied and then undone with its own inverse
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the restored archive's semantic projection matches what the original archive's does

    Examples:
      | id                  | params                                                                                                                 |
      | set-snapshot         | {"entries": [{"name": "manifest/readme.txt", "content": "Ersatzarchiv fuer den Mutationstest."}, {"name": "manifest/index.txt", "content": "Eintraege: 1"}], "comment": "Ersatzarchiv"} |
      | set-archive-comment  | {"comment": "Zwischenbericht Projektfotos, Stand Mutation"}                                                           |
      | add-entry            | {"name": "projekt/notiz.txt", "content": "Nachtrag: weiteres Bestandsprojekt folgt."}                                 |
      | remove-entry         | {"name": "projekt/P05_recypark_demets.jpg"}                                                                           |
      | rename-entry         | {"name": "projekt/P10_haus_hos.jpg", "newName": "projekt/P10_haus_hos_bestand.jpg"}                                   |
      | set-entry-data       | {"name": "projekt/P08_holbein_gardens.jpg", "content": "ERSATZINHALT: Bildbeleg durch Platzhaltertext ersetzt."}      |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the archive
    Given the real input archive shared://🗜️.zip
    When the no-mutation mutation is applied and then undone with its own inverse
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the restored archive's semantic projection matches what the original archive's does

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real archive, where bit-stability IS the correct answer
    Given the real input archive shared://🗜️.zip
    When the archive is decoded into the typed snapshot and re-encoded, with no mutation applied
    Then the re-encoded archive reproduces the input exactly, which is the reference writer's own bit-stability on an archive it authored rather than a byte pass-through
    And its semantic projection matches the oracle's own decode-then-reencode of the same input
