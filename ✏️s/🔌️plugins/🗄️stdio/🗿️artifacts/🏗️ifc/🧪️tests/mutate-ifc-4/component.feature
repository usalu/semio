@capability-ifc-4-any-mutate
@oracle-ruststep-ifc-4-any-mutate
@comparison-semantic-ifc-v1
@mutations-ifc-4-any
Feature: Apply every typed IFC4 mutation to a real-world exchange structure
  The input is a real 2.5 MB, 24792-entity IFC4 exchange structure, not a synthetic fixture:
  `shared://🏗️nakagin-capsule-tower.ifc`, a real IfcOpenShell 0.8.4.post1 export of the Nakagin
  Capsule Tower produced 2026-03-20, copied verbatim (byte-identical, confirmed by md5) from ticket
  26/03/20/EXPORT-NAKAGIN-CAPSULE-TOWER-IFC-FILE-TO-REPORTS's `test-nakagin.ifc` into this artifact's
  own `🧫️fixtures` directory, which is its durable home. Its header declares
  `FILE_SCHEMA(('IFC4'))`, matching this subset exactly (confirmed by reading the header directly).
  Every scenario copies it into the case work directory before touching it; the committed asset is
  never written to.

  KEY INSIGHT this subset's assignment names directly, confirmed here rather than taken on trust:
  IFC4 is not "IFC syntax" -- it is a real ISO 10303-21 (STEP physical file / Part-21) EXCHANGE
  STRUCTURE whose DATA section happens to carry IFC4's own EXPRESS schema instead of an AP-series
  one. This wave's `mutate-step-ap214` case already registered `ruststep` 0.4 as a real Part-21
  reader; that registration is repeated here for this subset (own catalog entry, own capability,
  `ifc-4-any-mutate`), and it applies unchanged because `ruststep`'s `ast` module parses the Part-21
  GRAMMAR only -- it compiles no generated IFC4 EXPRESS schema module at all, so it is genuinely
  schema-agnostic. This subset's own oracle test (`../../🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/
  🦀️component.rs`'s `parses_the_real_fixture_and_projects_it`) feeds it this real 24792-entity
  fixture and it parses with zero errors.

  Per the fleet brief's §6, `ruststep` has NO writer (confirmed independently here: no `Display`/
  `fmt::Formatter` impl on `Exchange`/`DataSection`/`Record`/`Parameter` anywhere in the crate, and
  `ast::ser::to_record` only builds an in-memory `Record` from an already-typed struct), so it cannot
  be a second PRODUCER of mutated bytes to diff the subject against and every scenario below is typed
  `@mode-property`/`@mode-round-trip`, never `@mode-differential`. It is registered as the
  INDEPENDENT READER instead: every mutation this subset's own oracle dispatcher performs (`ruststep`
  parses the real input; this subset's own from-scratch Part-21 writer re-serializes it) is read back
  through a FRESH, independent `ruststep::ast::Exchange::from_str` call (`project_ifc_4_any`) before
  `semantic-ifc-v1` compares it, and the identity round trip is checked the same way.

  `insert-entity`/`remove-entity`/`set-entity-arg` are exercised on real building entities, not
  synthetic ones: the real capsule proxy `#16976` (`IFCBUILDINGELEMENTPROXY`, name `'b'`) is removed
  deliberately BECAUSE a real `IFCRELAGGREGATES` (`#16991`) references it by id inside its member
  aggregate -- the integrity question the assignment calls out. This dispatcher removes only the one
  targeted DATA record and does not rewrite `#16991`'s reference, leaving it dangling; that is the
  honest behaviour of a positional entity-graph removal, and matches this subset's own production
  `IfcMutation::RemoveEntity` semantics (`schema::diff::diff_remove_entity`), which do not cascade
  either -- confirmed by reading that file, not assumed.


  THE FIRST SUBJECT RUN OF THIS CASE COULD NOT READ THE FIXTURE AT ALL, AND THE READER WAS WRONG.
  All 22 executable scenarios failed with `parse_part21 failed: part21: unsupported string escape at
  138718: unsupported escape start Some('\\')`. Byte 138718 is
  `#966=IFCBUILDINGELEMENTPROXYTYPE('1Cr_EEDPz6fuVrxIH6lX$j',$,'\\',$,$,…)` — a real one-character
  backslash name IfcOpenShell wrote as the doubled reverse solidus that ISO 10303-21's own STRING
  production defines, exactly as it defines the doubled apostrophe. This repository's shared Part-21
  lexer implemented `\X\HH\` and `\X2\…\X0\` and nothing else; `ruststep`, the registered
  independent reader, read the same file without complaint. The lexer now implements the complete
  §6.4.2 directive set — `\\`, `\X\HH` (two hex digits and NO terminator, per the grammar's own
  `arbitrary = "\X\" hex_one`, which the old code got wrong too), `\X2\`/`\X4\` runs, `\S\`
  and `\P?\` — refusing an ISO 8859 page it cannot map rather than guessing a character. No
  fixture was touched and no assertion relaxed.

  AND THE PROJECTION WAS COMPARING ENCODINGS. `ruststep`'s `string` combinator is
  `many0(none_of("'"))`: it returns the raw text between the apostrophes and decodes no control
  directive, so the oracle's `'\\'` lexeme and the subject's conformant `\X2\005C\X0\`
  re-encoding of the SAME one-character value read as different argument values. The projection now
  decodes each literal through a from-scratch §6.4.2 reader in the shared AP214 oracle — written
  independently of the production codec it is evidence about, refusing a malformed directive rather
  than waving it through. `semantic-ifc-v1` compares exactly the same axes; nothing was ignored,
  loosened or swapped.

  📌️ Every Examples row below other than `no-mutation` is required to MOVE the semantic projection,
  and the adapter fails the scenario in role when it does not: a row whose parameters make the
  mutation a no-op passes whenever the reference library merely declined to error, which is not a
  test. The baseline it is measured against runs one `no-mutation` cycle first, so the comparison
  isolates the mutation rather than the writer's own normal form.
  @id-mutate
  @level-exhaustive
  @mode-property
  Scenario Outline: Apply <id> to the real exchange structure
    Given the real input document shared://🏗️nakagin-capsule-tower.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the independently read projection shows the mutation's real effect
    Examples:
      | id                    | params                                                                                                                                                                                                        |
      | no-mutation           | {}                                                                                                                                                                                                            |
      | set-snapshot          | {"fileSchema": ["IFC4X3"]}                                                                                                                                                                                    |
      | set-file-description  | {"values": [{"t": "aggregate", "v": [{"t": "string", "v": "wave-7 mutation"}]}, {"t": "string", "v": "2;1"}]}                                                                                                |
      | set-file-name         | {"values": [{"t": "string", "v": "wave-7-mutated.ifc"}, {"t": "string", "v": "2026-08-23T00:00:00"}, {"t": "aggregate", "v": [{"t": "string", "v": "Ueli"}]}, {"t": "aggregate", "v": [{"t": "string", "v": "semio"}]}, {"t": "string", "v": "semio-ifc"}, {"t": "string", "v": "semio"}, {"t": "string", "v": ""}]} |
      | set-file-schema       | {"values": [{"t": "aggregate", "v": [{"t": "string", "v": "IFC4X3"}]}]}                                                                                                                                      |
      | insert-entity         | {"index": 24792, "entity": {"id": 90001, "name": "IFCCARTESIANPOINT", "args": [{"t": "aggregate", "v": [{"t": "real", "v": 1000.0}, {"t": "real", "v": 2000.0}, {"t": "real", "v": 3000.0}]}]}}             |
      | remove-entity         | {"id": 16976}                                                                                                                                                                                                 |
      | set-entity-name       | {"id": 16976, "name": "RENAMED_PROXY"}                                                                                                                                                                       |
      | set-entity-arg        | {"id": 16976, "index": 2, "value": {"t": "string", "v": "origin-marker"}}                                                                                                                                    |
      | insert-entity-arg     | {"id": 16976, "index": 9, "value": {"t": "enum", "v": "T"}}                                                                                                                                                  |
      | remove-entity-arg     | {"id": 16976, "index": 8}                                                                                                                                                                                     |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real exchange structure
    Given the real input document shared://🏗️nakagin-capsule-tower.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the independently read projection matches its pre-mutation projection
    Examples:
      | id                    | params                                                                                                                                                                                                        |
      | no-mutation           | {}                                                                                                                                                                                                            |
      | set-snapshot          | {"fileSchema": ["IFC4X3"]}                                                                                                                                                                                    |
      | set-file-description  | {"values": [{"t": "aggregate", "v": [{"t": "string", "v": "wave-7 mutation"}]}, {"t": "string", "v": "2;1"}]}                                                                                                |
      | set-file-name         | {"values": [{"t": "string", "v": "wave-7-mutated.ifc"}, {"t": "string", "v": "2026-08-23T00:00:00"}, {"t": "aggregate", "v": [{"t": "string", "v": "Ueli"}]}, {"t": "aggregate", "v": [{"t": "string", "v": "semio"}]}, {"t": "string", "v": "semio-ifc"}, {"t": "string", "v": "semio"}, {"t": "string", "v": ""}]} |
      | set-file-schema       | {"values": [{"t": "aggregate", "v": [{"t": "string", "v": "IFC4X3"}]}]}                                                                                                                                      |
      | insert-entity         | {"index": 24792, "entity": {"id": 90001, "name": "IFCCARTESIANPOINT", "args": [{"t": "aggregate", "v": [{"t": "real", "v": 1000.0}, {"t": "real", "v": 2000.0}, {"t": "real", "v": 3000.0}]}]}}             |
      | remove-entity         | {"id": 16976}                                                                                                                                                                                                 |
      | set-entity-name       | {"id": 16976, "name": "RENAMED_PROXY"}                                                                                                                                                                       |
      | set-entity-arg        | {"id": 16976, "index": 2, "value": {"t": "string", "v": "origin-marker"}}                                                                                                                                    |
      | insert-entity-arg     | {"id": 16976, "index": 9, "value": {"t": "enum", "v": "T"}}                                                                                                                                                  |
      | remove-entity-arg     | {"id": 16976, "index": 8}                                                                                                                                                                                     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real exchange structure without passing bytes through
    Given the real input document shared://🏗️nakagin-capsule-tower.ifc
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the independently read projections of input and output agree
