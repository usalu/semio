@capability-dwg-ac1018-mutate
@no-oracle-dwg-ac1018-proprietary-container
@comparison-semantic-dwg-preamble-v1
@mutations-dwg-ac1018-any
Feature: Stamp a real DWG container R2004 and read the AC1018 preamble back at the published offsets
  ⚠️ Read this first, because it is the case's own boundary rather than a footnote. **There is no
  AC1018 file in this repository.** Both `.dwg` files committed outside `./compose` begin with the
  six characters `AC1024`: the 148,638-byte `📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg`
  this case reads (whose own docstring says so — "a real, non-trivial fixture (AC1024, ~145KB)") and
  the 22-byte `📚️examples/🎬️demo/🖼️assets/🖊️example.dwg` preamble stub. So this case reads an R2010
  container filed under the R2004 standard's example tree, and it is written to claim only what that
  supports.

  What it therefore demonstrates is not "an R2004 container was parsed" but "the R2004 stamp is
  producible and readable at the published offsets": every mutating row below drives the container's
  version stamp TO `AC1018`, and the adapter fails the scenario unless an independent preamble
  reader then reads `AC1018` back. That is the exact mirror of the sibling AC1024 case, where the
  native `AC1024` stamp must SURVIVE untouched — the two cases assert opposite things about the same
  bytes, which is why they are two cases and not one text under two names. The version string is the
  DATA this vocabulary mutates, never a precondition for reading the file, so nothing here depends
  on the fixture having been authored by an R2004 writer.

  The complement is asserted too, and deliberately: the identity round trip requires the reader to
  report `AC1024`, the stamp the FILE carries, not `AC1018`, the standard the case is filed under.
  Asserting R2004 there would be asserting a fiction about a fixture this repository does not have.

  Every scenario copies the file into the case work directory first; the committed drawing is never
  written to. The real values the published offsets carry in it — `maint_version` `0x02` at 0x12 and
  codepage 30 (ANSI_1252) at 0x13-0x14 — are what the rows are written against, so the `set-version-
  info` row keeps codepage 30 and moves only the two fields R2004 is about.

  There is no oracle. DWG is proprietary and undocumented; the only independent implementation of
  any weight is LibreDWG, which is GPL-3.0 C and would put a copyleft C library on this repository's
  test host with no owner ruling permitting it, and no permissively licensed Rust DWG reader exists
  at all (`dxf` 0.6, registered for the sibling 🖊️dxf artifact, reads the PUBLISHED DXF interchange
  format and explicitly not DWG). The evidence is therefore specification vectors and the
  metamorphic laws, exercised by an independently hand-written reader/writer of the one part of DWG
  that IS publicly specified — six ASCII version characters at 0x00-0x05, the `maint_version` byte
  at 0x12, and the little-endian `codepage` u16 at 0x13-0x14, the offsets LibreDWG's own
  `header.spec` documents. That reader never calls this repository's own R2004+ decoder.

  The narrowness is real and is stated rather than hidden. Everything after the preamble is the
  R2004+ section map — compressed, checksummed, section-encrypted — which nothing here and nothing
  in the permissively licensed Rust ecosystem can regenerate, so it is carried through unchanged and
  the projection is the preamble triple plus the document's byte length. `byteLength` is what keeps
  `set-snapshot` (a whole-document replacement, collapsing the container to the 22-byte
  preamble-only shape) observably different from `set-version-info` (a field set in place). Every
  row below moves that projection: the adapter fails any non-`no-mutation` row whose projection did
  not change.

  🔒️ The identity round trip asserts the EXACT-BYTES law, not the no-byte-pass-through law, and that
  is the correct law here rather than a missing one. The preamble is fixed-width with no writer
  freedom whatsoever and the body is uncopyable by construction. The check is still not a tautology:
  the handler ZEROES the 21-byte preamble region before writing it back, so byte equality proves
  every one of those bytes was re-derived from the parse rather than left in place by a memcpy.

  ⚠️ This subset's vocabulary is the AC1024 one, shared by CONSTRUCTION rather than copied. The ODA
  `.dwg` specification gives R2004 and R2010 the SAME file-header layout — the three fields above,
  at the same offsets — so one `DwgMutation` addresses both, and `🔖️ac1018/🪆️subsets/✳️any/
  🧬️schema/🧬️mutations/🦀️component.rs` re-exports it rather than restating it; the AC1024 oracle
  module's `every_ac1018_facet_is_a_re_export_of_this_one` test reads the committed sources and
  fails the moment that stops being true. What is NOT identical by specification is what the two
  containers hold BEHIND that header, and this repository decodes both through one R2004 path whose
  own errors name AC1024 framing — a real gap, recorded in the ticket's report rather than dressed
  up here as coverage.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and read the R2004 stamp back
    Given the real input drawing asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the preamble projection reports the values the specification's own offsets predict, asserted in role
    And a mutating row left the container stamped AC1018, asserted in role
    Examples:
      | id               | params |
      | no-mutation      | {} |
      | set-snapshot     | {"version": "AC1018", "maintenanceVersion": 0, "codepage": 30} |
      | set-version-info | {"version": "AC1018", "maintenanceVersion": 0, "codepage": 30} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> brings the original stamp back
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
      | set-snapshot     | {"version": "AC1018", "maintenanceVersion": 0, "codepage": 30} |
      | set-version-info | {"version": "AC1018", "maintenanceVersion": 0, "codepage": 30} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the preamble, reporting the stamp the file really carries
    Given the real input drawing asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg
    When the preamble region is zeroed and rewritten from the parsed fields alone
    Then the preamble projection is unchanged, asserted in role
    And the re-encoded bytes reproduce the input exactly, asserted in role
    And the stamp read back is AC1024, the file's own, not AC1018, this case's filing, asserted in role
