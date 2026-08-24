@capability-dwg-ac1024-mutate
@no-oracle-dwg-ac1024-proprietary-container
@comparison-semantic-dwg-preamble-v1
@mutations-dwg-ac1024-any
Feature: Apply every typed DWG AC1024 mutation to the container that is actually stamped R2010
  The input is asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg,
  a real 148,638-byte architectural drawing, and for THIS case it is a native fixture: its first six
  bytes read `AC1024`. That is the whole reason the two DWG cases are not one. The sibling AC1018
  case reads the same file and has to say so as a limitation; here there is nothing to disclose, and
  the case can ask the question only an AC1024 case can ask — does the R2010 stamp survive, byte for
  byte, a full decode and re-encode of the container it labels? Every scenario copies the file into
  the case work directory first; the committed drawing is never written to.

  The expectations below are the values the published offsets actually carry in that file, read out
  of it rather than invented: version `AC1024`, `maint_version` `0x02` at offset 0x12, and codepage
  30 (ANSI_1252) as a little-endian u16 at 0x13-0x14. Both roles are measured against those, which
  is what `@mode-conformance` means here — each side answers to the specification, not to the other.

  There is no oracle. DWG is proprietary and undocumented; the only independent implementation of
  any weight is LibreDWG, which is GPL-3.0 C and would put a copyleft C library on this repository's
  test host with no owner ruling permitting it, and no permissively licensed Rust DWG reader exists
  at all (`dxf` 0.6, registered for the sibling 🖊️dxf artifact, reads the PUBLISHED DXF interchange
  format and explicitly not DWG). The evidence is therefore specification vectors plus the
  metamorphic laws, exercised by an independently hand-written reader/writer of the one part of DWG
  that IS publicly specified — the preamble every file since R13 begins with, at the offsets
  LibreDWG's own `header.spec` documents and this subset's `DwgSnapshot` doc comments already cite.
  That reader never calls this repository's own 12,000-line R2004+ decoder.

  The narrowness is real and is stated rather than hidden. Everything after the preamble is the
  R2004+ section map — compressed, checksummed, section-encrypted — which nothing here and nothing
  in the permissively licensed Rust ecosystem can regenerate, so it is carried through unchanged and
  the projection is the preamble triple plus the document's byte length. `byteLength` is what keeps
  `set-snapshot` (a whole-document replacement, which collapses the container to the 22-byte
  preamble-only shape this artifact's own demo example already has) observably different from
  `set-version-info` (a field set that leaves every other byte where it was), rather than the two
  verbs collapsing into one indistinguishable edit. Every row below is chosen to move that
  projection: the adapter fails any non-`no-mutation` row whose projection did not change.

  🔒️ The identity round trip asserts the EXACT-BYTES law, not the no-byte-pass-through law, and that
  is the correct law here rather than a missing one. The preamble is fixed-width with no writer
  freedom whatsoever and the body is uncopyable by construction, so reproducing the input exactly is
  the right answer and anything else is the defect. The check is still not a tautology: the handler
  ZEROES the 21-byte preamble region before writing it back, so byte equality proves every one of
  those bytes was re-derived from the parse rather than left in place by a memcpy. It additionally
  asserts the stamp read back is `AC1024` — the one claim a native-fixture case is entitled to make.

  ⚠️ This standard's vocabulary is shared with AC1018's, by CONSTRUCTION rather than by copy. The
  ODA `.dwg` specification gives R2004 and R2010 the SAME file-header layout — six ASCII version
  characters at 0x00, the application maintenance-release byte at 0x12, the codepage RS at
  0x13-0x14 — so one `DwgMutation` addresses both, and `🔖️ac1018/🪆️subsets/✳️any/🧬️schema/
  🧬️mutations/🦀️component.rs` re-exports this standard's enum instead of restating it. The AC1024
  oracle module's `every_ac1018_facet_is_a_re_export_of_this_one` test reads the committed sources
  and fails the moment that stops being true. What is NOT identical by specification is what the two
  containers hold BEHIND that header, and this repository has one decoder for both — recorded as a
  real gap in the ticket's report, not resolved by this case.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the R2010 container
    Given the real input drawing asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the preamble projection reports the values the specification's own offsets predict, asserted in role
    And a mutating row moved that projection, asserted in role
    Examples:
      | id               | params |
      | no-mutation      | {} |
      | set-snapshot     | {"version": "AC1024", "maintenanceVersion": 0, "codepage": 0} |
      | set-version-info | {"version": "AC1032", "maintenanceVersion": 7, "codepage": 29} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the R2010 container
    Given the real input drawing asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation computed against the untouched original is applied to that result
    Then the restored drawing's preamble projection equals the original's, asserted in role
    Examples:
      | id               | params |
      | no-mutation      | {} |
      | set-snapshot     | {"version": "AC1024", "maintenanceVersion": 0, "codepage": 0} |
      | set-version-info | {"version": "AC1032", "maintenanceVersion": 7, "codepage": 29} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the R2010 container's preamble
    Given the real input drawing asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg
    When the preamble region is zeroed and rewritten from the parsed fields alone
    Then the preamble projection is unchanged, asserted in role
    And the re-encoded bytes reproduce the input exactly, asserted in role
    And the stamp read back is AC1024, asserted in role
