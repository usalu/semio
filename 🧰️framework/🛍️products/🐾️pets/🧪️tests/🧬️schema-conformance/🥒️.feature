@capability-pets-schema-conformance
@oracle-pets-jsonschema
@comparison-ordered-json-v1
Feature: Every pets document conforms to the normative schema and to the rules beyond it
  `🧬️schema/🔣️.json` (JSON Schema draft-07) is the normative contract of menageries, species and
  ensembles; the cores validate them with an owned validator (`menagerieIssues`, `speciesIssues`,
  `ensembleIssues`) and link no schema library at runtime. A finding is a JSON pointer and a kebab-case
  code, findings are sorted by pointer, then code, in code point order. This case holds that validator
  to the schema itself and to the document rules of the design (§3).

  THE REFERENCE is python-jsonschema's Draft 7 validator reading the normative file (`🐍️.py` beside
  this file). It judges three populations of the committed vectors.

  The `accepted` vectors are the sample menagerie (a walker, a hopper and a floater with bonds and the
  casts of the scenes `home` and `meadow`), its ensemble, each of its species documents, the minimal
  bases and the edges the rules allow (a looping rotation that ends a whole number of turns from where
  it starts, an easing that overshoots, a floater without a walk clip, a `$schema` hint). The schema
  accepts every one of them.

  The `structural` vectors break the type-level structure at exactly one keyword: a wrong JSON type, a
  missing or undeclared property, a literal outside its enumeration, a tuple of the wrong length. The
  schema reports the keyword and the pointer each vector names (`required` and `additionalProperties`
  at the property concerned, keywords inside the `Shape` union at the value they judge).
  Implementations project only that they reject the document, because a core that decodes into typed
  twins refuses such a document before its validator sees it; the owned TypeScript validator is
  additionally held to the committed pointer and code in its unit suite.

  The `rules` vectors keep a sound structure and break a rule: `duplicate-id`, `unknown-reference`,
  `bone-order`, `key-order`, `loop-seam`, `ease-range`, `out-of-range`, `self-bond`, `duplicate-bond`,
  `missing-gait-clip`, `float-hover`, `empty-cast`, `duplicate-scene`, and the value-level findings
  `slug-invalid`, `length-invalid` and `items-too-few`. The schema either cannot see the rule — it
  accepts the document, which proves the finding comes from the rule alone — or reports the keyword
  the vector names (ranges, the colour pattern, ease abscissas, the two different species of a bond,
  the core of a cast, fewer than two keys). The findings themselves are recomputed in the adapter by a
  second implementation of the rules written from the design text; it is a supplement that holds both
  cores to one reading of the rules, not third-party evidence.

  The subjects are `@semio-tech/pets` (`menagerieIssues`, `speciesIssues`, `ensembleIssues`) and the
  `pets` crate (`menagerie_issues`, `species_issues`, `ensemble_issues`).

  The vectors shared://🧬️schema-conformance/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_schema_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-accepted-documents
  @level-fundamental
  @mode-differential
  Scenario: The sample menagerie, its ensemble, its species and every allowed edge are accepted
    Given the committed vectors shared://🧬️schema-conformance/🔣️.json
    When every accepted document is validated against the definition its vector names
    Then jsonschema accepts every one of them
    And every implementation reports no finding for any of them

  @id-structural-rejections
  @level-fundamental
  @mode-error
  Scenario: Documents that break the structure at one keyword are rejected everywhere
    Given the committed vectors shared://🧬️schema-conformance/🔣️.json
    When every structurally broken document is validated against the definition its vector names
    Then jsonschema reports the keyword at the pointer each vector names
    And every implementation rejects every one of them

  @id-rule-violations
  @level-fundamental
  @mode-differential
  Scenario: Documents that break a rule yield the same findings everywhere
    Given the committed vectors shared://🧬️schema-conformance/🔣️.json
    When every rule document is validated against the definition its vector names
    Then jsonschema accepts the documents whose rule lies beyond the schema and reports the named keyword for the others
    And every implementation projects the same findings per document, pointer and code, in code point order
