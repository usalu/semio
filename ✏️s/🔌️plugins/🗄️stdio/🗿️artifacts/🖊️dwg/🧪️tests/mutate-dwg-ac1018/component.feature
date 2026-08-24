@capability-dwg-ac1018-mutate
@no-oracle-dwg-ac1018-proprietary-container
@comparison-semantic-dwg-preamble-v1
@mutations-dwg-ac1018-any
Feature: Apply every typed DWG AC1018 mutation to the real committed drawing
  The input is asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg,
  this artifact's own committed 148,638-byte architectural drawing — a real DWG, not a stub. Every
  scenario copies it into the case work directory before touching it; the committed file is never
  written to.

  ⚠️ A fact about that fixture, recorded rather than smoothed over: it is filed under the ac1018
  example tree, but its first six bytes read `AC1024`. It is a DWG R2010 container living in the
  R2004 standard's example directory. It is the ONLY real DWG committed to this repository (the
  other, `📚️examples/🎬️demo/🖼️assets/🖊️example.dwg`, is a 22-byte preamble-only stub, and it too
  says `AC1024`), so both DWG cases read it, and the ac1018 case says so here instead of implying it
  is exercising an R2004-stamped container. This costs the ac1018 case nothing it could otherwise
  have had: the version string is the DATA this vocabulary mutates, not a precondition for reading
  the file, and `set-version-info` is exercised precisely by rewriting it.

  There is no oracle. DWG is proprietary and undocumented; the only independent implementation of
  any weight is LibreDWG, which is GPL-3.0 C and would put a copyleft C library on this
  repository's test host with no owner ruling permitting it, and no permissively licensed Rust DWG
  reader exists at all (`dxf` 0.6, registered for the sibling 🖊️dxf artifact, reads the PUBLISHED
  DXF interchange format and explicitly not DWG). The evidence is therefore specification vectors
  and the metamorphic laws, exercised by an independently hand-written reader/writer of the one
  part of DWG that IS publicly specified — the preamble every file since R13 begins with: six ASCII
  version characters at 0x00-0x05, the `maint_version` byte at 0x12, and the little-endian
  `codepage` u16 at 0x13-0x14. Those are the offsets LibreDWG's own `header.spec` documents and
  this subset's `DwgSnapshot` doc comments already cite, and the real fixture confirms both (0x02
  and 30 = ANSI_1252). That reader never calls this repository's own 12,000-line R2004+ decoder.

  The narrowness is real and is stated rather than hidden. Everything after the preamble is the
  R2004+ section map — compressed, checksummed, section-encrypted — which nothing here and nothing
  in the permissively licensed Rust ecosystem can regenerate, so it is carried through unchanged
  and the projection is the preamble triple plus the document's byte length. `byteLength` is what
  keeps `set-snapshot` (a whole-document replacement, which collapses the container to a
  preamble-only document) observably different from `set-version-info` (a field set that leaves
  every other byte where it was), rather than the two verbs collapsing into one indistinguishable
  edit.

  🔒️ The identity round trip asserts the EXACT-BYTES law, not the no-byte-pass-through law, and
  that is the correct law here rather than a missing one. The preamble is fixed-width with no
  writer freedom whatsoever and the body is uncopyable by construction, so reproducing the input
  exactly is the right answer and anything else is the defect. The check is still not a tautology:
  the handler ZEROES the 21-byte preamble region before writing it back, so byte equality proves
  every one of those bytes was re-derived from the parse rather than left in place by a memcpy.

  ⚠️ This subset's vocabulary is `pub use`-identical to the other DWG standard's: `🔖️ac1018/
  🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` is a one-line re-export of `🔖️ac1024`'s, as
  are its `🧬️schema` and `📸️snapshot` facets, so `DwgMutation` is ONE Rust enum shared by both
  standards. The `dwg-ac1018-any` and `dwg-ac1024-any` catalogs therefore declare the same three
  kinds by CONSTRUCTION, not by a copy that could rot, and the AC1024 oracle module's
  `every_ac1018_facet_is_a_re_export_of_this_one` test reads the committed sources and fails the
  moment that stops being true. Two catalogs exist anyway because the contract gate counts coverage
  per subset and a catalog claimed by no feature is itself a breach.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real drawing
    Given the real input drawing asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the preamble projection reports the values the specification's own offsets predict
    Examples:
      | id               | params |
      | no-mutation      | {} |
      | set-snapshot     | {"version": "AC1018", "maintenanceVersion": 0, "codepage": 0} |
      | set-version-info | {"version": "AC1032", "maintenanceVersion": 7, "codepage": 29} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real drawing
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
      | set-snapshot     | {"version": "AC1018", "maintenanceVersion": 0, "codepage": 0} |
      | set-version-info | {"version": "AC1032", "maintenanceVersion": 7, "codepage": 29} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real drawing's preamble
    Given the real input drawing asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg
    When the preamble region is zeroed and rewritten from the parsed fields alone
    Then the preamble projection is unchanged, asserted in role
    And the re-encoded bytes reproduce the input exactly, asserted in role
