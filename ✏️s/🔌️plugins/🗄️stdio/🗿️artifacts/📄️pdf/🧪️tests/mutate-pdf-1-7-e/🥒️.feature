@capability-pdf-1-7-e-mutate
@oracle-lopdf-pdf-1-7-e-mutate
@comparison-semantic-pdf-conformance-e-v1
@mutations-pdf-1-7-e
Feature: Apply every typed ISO 24517-1:2008 (PDF/E-1) conformance-class mutation to a real document
  The input is the real, committed 6.3 MB bachelor thesis produced by MiKTeX pdfTeX 1.40.21 — 65
  pages, 3,189 indirect objects, a classic cross-reference table, 70 /Type /Font objects and 23
  /Type /FontDescriptor objects, every one of the 23 carrying an embedded font program (5 /FontFile,
  16 /FontFile2, 2 /FontFile3). Scanning the committed file confirms it carries NO /Encrypt, no
  /S /JavaScript action and no /JS key, no /S /Launch action, no /Subtype /Movie or /Sound
  annotation, no /Type /Filespec, no /OutputIntents, no /MarkInfo, no /StructTreeRoot, no /Lang, no
  /ViewerPreferences, no /AcroForm, no /DPartRoot and no /TrimBox or /ArtBox on any page — it is a
  perfectly ordinary PDF that conforms to no conformance class at all, which is exactly what makes
  it the right input here: every scenario moves the real document along ONE axis of the class and
  then back. It is read where the domain already keeps it; every scenario copies it into the case
  work directory before touching it, and the committed document is never written to.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF ✳️any. The ✳️any subset of this same standard
  owns the DOCUMENT vocabulary — insert-page, remove-page, move-page, the media/crop box kinds, page
  content, /Info as authoring metadata, and the raw object/dict/trailer edit primitives. This subset
  owns ISO 24517-1:2008 (PDF/E-1), a CONFORMANCE CLASS, which is a property of the object graph as a whole and
  of no page at all. The catalog is one kind per axis of this subset's own check_e_conformance, which
  reads six axes: any Standard Security Handler `/Encrypt` dictionary object, any `/S /JavaScript` action or bare `/JS` key, any `/S /Launch` action, any `/Subtype /Movie` or `/Subtype /Sound` annotation, a NON-EMPTY `/Root/OutputIntents` array whatever its `/S` marker, and an embedded program on every font's `/FontDescriptor`. No ✳️any mutation moves any of those axes and no mutation here touches
  page content, so the two vocabularies are disjoint by construction.

  Two things separate this vocabulary from PDF/A's and PDF/X's. First, the MEDIA-ANNOTATION pair: `check_e_conformance`'s `movie_or_sound_annotations` matches `/Subtype /Movie` and `/Subtype /Sound` and deliberately never matches `/Subtype /3D`, which ISO 24517-1 explicitly ALLOWS — an engineering-document format that forbids video but permits embedded 3D geometry. Second, the OutputIntent axis is satisfied by ANY intent (`has_any_output_intent` tests only that the array is non-empty), unlike PDF/A's `/GTS_PDFA1` and PDF/X's `/GTS_PDFX` + `/DestOutputProfile`. The engine still has to write SOME `/S` marker, and it writes `/GTS_PDFX` — one of the two subtypes ISO 32000-1 Table 365 actually registers — rather than inventing a PDF/E name the standard does not define.

  THE REFERENCE. `lopdf` 0.44 parses the complete COS object graph of the real document and writes a
  fresh file from that graph alone — never a patch of the input bytes — and it both performs and
  observes every kind this catalog declares, which is why every mutate scenario is @mode-differential
  rather than a weaker mode. It is test-only; this repository's own PDF codec is hand-written and
  links nothing.

  ONE DELIBERATE SCOPE NOTE, RECORDED RATHER THAN GLOSSED. insert-encryption-dictionary adds a
  free-standing Standard Security Handler dictionary OBJECT and does not link it from the trailer's
  /Encrypt. That is faithful to what this subset actually checks — check_e_conformance scans the retained
  objects for the /Filter /Standard + /V + /R + /O + /U shape and never reads the trailer — and it is
  the only form the mutation can take and still leave a document both producers can re-read, since a
  genuinely /Encrypt-linked trailer makes every string and stream in the file ciphertext.

  THE ARRANGED SCENARIOS, AND THAT THEY ARE ARRANGED. The committed document carries none of the
  constructs listed above, so a scenario whose mutation REMOVES one runs on the real document after
  the SAME independent implementation has put it there. The mutation under test is still the
  removal, still performed by the reference, still on a genuine 3,189-object graph:
    remove-encryption-dictionary — an /Encrypt dictionary object is inserted first.
    remove-javascript-action — the /S /JavaScript action is inserted first.
    remove-launch-action — the /S /Launch action is inserted first.
    remove-media-annotation — the annotation is inserted first.
    remove-output-intent — the OutputIntent is installed first.
    embed-font-file — descriptor 4's embedded program is REMOVED first — all 23 of the fixture's /FontDescriptor objects already carry one.

  THE PROJECTION RECORDS CONTENT, NOT OBJECT NUMBERS. A conformance class is defined on what a
  document contains, not on where a writer chose to put it: re-inserting a removed action at a fresh
  object number is a faithful undo, and a projection that recorded the number would report a false
  divergence for it. What the projection carries instead is exactly the content check_e_conformance's own
  diagnostics quote, scoped to the axes this subset reads and to no others.

  THE LAWS THE ORACLE ASSERTS IN-ROLE, so a scenario cannot pass merely because `lopdf` did not
  error. mutate-<id> fails unless the mutation actually MOVED the conformance-class projection.
  inverse-<id> applies the mutation, applies its own independently computed inverse, and fails with
  the first diverging field unless the result projects onto exactly what the pre-state projects onto.
  identity-round-trip fails unless the re-serialized bytes differ from the input AND their projection
  is identical to the input's.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the conformance-class projection
    Examples:
      | id                           | params                                                            |
      | insert-encryption-dictionary | {"version": 2, "revision": 3}                                     |
      | remove-encryption-dictionary | {"version": 2, "revision": 3}                                     |
      | insert-javascript-action     | {"script": "app.alert('this document phones home');"}             |
      | remove-javascript-action     | {"script": "app.alert('this document phones home');"}             |
      | insert-launch-action         | {"target": "render-plots.bat"}                                    |
      | remove-launch-action         | {"target": "render-plots.bat"}                                    |
      | insert-media-annotation      | {"subtype": "Movie", "title": "site walkthrough"}                 |
      | remove-media-annotation      | {"subtype": "Sound", "title": "narration"}                        |
      | set-output-intent            | {"identifier": "sRGB IEC61966-2.1"}                               |
      | remove-output-intent         | {}                                                                |
      | embed-font-file              | {"descriptorOrdinal": 4, "key": "FontFile2", "programOrdinal": 0} |
      | remove-font-file             | {"descriptorOrdinal": 4}                                          |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the conformance-class projection is the one the document started from
    Examples:
      | id                           | params                                                            |
      | insert-encryption-dictionary | {"version": 2, "revision": 3}                                     |
      | remove-encryption-dictionary | {"version": 2, "revision": 3}                                     |
      | insert-javascript-action     | {"script": "app.alert('this document phones home');"}             |
      | remove-javascript-action     | {"script": "app.alert('this document phones home');"}             |
      | insert-launch-action         | {"target": "render-plots.bat"}                                    |
      | remove-launch-action         | {"target": "render-plots.bat"}                                    |
      | insert-media-annotation      | {"subtype": "Movie", "title": "site walkthrough"}                 |
      | remove-media-annotation      | {"subtype": "Sound", "title": "narration"}                        |
      | set-output-intent            | {"identifier": "sRGB IEC61966-2.1"}                               |
      | remove-output-intent         | {}                                                                |
      | embed-font-file              | {"descriptorOrdinal": 4, "key": "FontFile2", "programOrdinal": 0} |
      | remove-font-file             | {"descriptorOrdinal": 4}                                          |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the document is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance-class projection
    And the re-encoded bytes are not bit-identical to the input
