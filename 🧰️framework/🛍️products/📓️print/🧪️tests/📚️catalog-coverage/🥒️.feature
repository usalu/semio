@capability-viz-catalog-coverage
@oracle-ajv
@oracle-markdown-it
@comparison-ordered-json-v1
Feature: The chart-kind catalogue covers the handcrafted taxonomy exactly once, with a vocabulary the LaTeX packages can serve
  The taxonomy `🖼️assets/📊️viz-taxonomy.md` is the single source of truth for what the print
  visualization library must be able to draw. `🖼️assets/🔣️viz-catalog.json` is the machine catalogue
  that maps every one of its terminal leaves onto exactly one chart kind: a family plus that family's
  default option list. This case is the structural gate in front of every renderer — it needs no TeX
  and therefore runs at the `fundamental` level, before the probe cases that measure geometry.

  Two third-party references adjudicate it rather than the implementation's own readers.
  `markdown-it` parses the taxonomy into a token stream and the leaf set is read out of that stream,
  so the subject's own line-oriented parser cannot agree with itself; `ajv` (draft 2020-12) validates
  the catalogue document against `🧬️schema/🔣️.json`, so the entry shape is judged by a JSON Schema
  implementation and not by TypeScript types that are erased at runtime.

  The remaining scenarios have no third party to compare against because they are relations between
  two files of this repository — a catalogue slug and a `\SemioVizFamily` registration, a catalogue
  option key and the schema's `x-semio-family-options` vocabulary. They are conformance scenarios
  with specification vectors instead, and the specification is architecture §4 of the ticket:
  every leaf covered exactly once, every family registered by exactly one package, every option key
  declared, no two kinds sharing family and options, both languages titled.

  One of them reads the other direction of the same relation. The schema is what a document author
  consults and what `🔓️viz-api.tex` prints, so it must say exactly what a family accepts, and
  `@id-options` only ever proves that a key a *catalogue entry sets* is declared. What a family
  accepts is four things at once: the keys declared on `semio / viz / family / <name>`, the keys a
  shared vocabulary helper installs on that path when it is handed the family's name, the keys an
  `unknown` handler forwards to another path, and the keys of every kernel path the family hands its
  own option list to. `@id-implemented-keys-documented` reads all four out of the LaTeX sources and
  compares them with the schema in both directions.

  @id-schema
  @level-fundamental
  @mode-conformance
  Scenario: The catalogue validates against its own JSON Schema
    Given the catalogue asset://🖼️assets/🔣️viz-catalog.json
    And the schema asset://🧬️schema/🔣️.json
    When the catalogue is validated with ajv in draft 2020-12 mode
    Then ajv reports no errors

  @id-leaves
  @level-fundamental
  @mode-differential
  Scenario: Every taxonomy leaf markdown-it finds is covered by exactly one chart kind
    Given the taxonomy asset://🖼️assets/📊️viz-taxonomy.md
    When the leaf identifiers are read out of the markdown-it token stream
    Then the catalogue's `covers` lists contain each of them exactly once
    And the catalogue contains no leaf identifier the token stream does not hold

  @id-slugs
  @level-fundamental
  @mode-conformance
  Scenario: Chart-kind slugs are global, section-free and unique
    Given the catalogue asset://🖼️assets/🔣️viz-catalog.json
    Then no slug ends in a taxonomy section number
    And no two entries share a slug

  @id-families
  @level-long
  @mode-conformance
  Scenario: Every family a chart kind names is registered by exactly one LaTeX package
    Given every `\SemioVizFamily{name}` occurrence under 🖋️latex
    Then every family named by the catalogue occurs there
    And no family name occurs in two packages

  @id-options
  @level-fundamental
  @mode-conformance
  Scenario: Every option key a chart kind sets is declared for its family in the schema
    Given the schema's `x-semio-family-options` vocabulary
    Then every option key of every catalogue entry is declared for that entry's family
    And every entry sets the required `variant` key
    And every entry's `data` names a declared demo table

  @id-implemented-keys-documented
  @level-fundamental
  @mode-conformance
  Scenario: Every option key a family implements is documented in the schema, and nothing else is
    Given every `\keys_define` block and `\SemioVizFamily` body under 🖋️latex
    Then the schema documents every key each family implements
    And the schema declares no key its family does not implement
    And every family the schema documents is registered by a package

  @id-distinctness
  @level-fundamental
  @mode-conformance
  Scenario: No two chart kinds of one family carry the same options
    Given the catalogue asset://🖼️assets/🔣️viz-catalog.json
    Then the pair of family and rendered option list is unique across all entries

  @id-languages
  @level-fundamental
  @mode-conformance
  Scenario: Every chart kind, taxonomy section and taxonomy group is titled in both document languages
    Given the catalogue and the schema's taxonomy annotations
    Then each carries a non-empty `en` and a non-empty `de` title
    And the generated `semio-viz-catalog-labels.sty` registers both titles for every chart kind

  @id-generated
  @level-fundamental
  @mode-conformance
  Scenario: The generated packages and gallery documents are in sync with the catalogue
    Given the catalogue-derived files `generate viz` would write
      | artifact                       |
      | semio-viz-catalog.sty          |
      | semio-viz-catalog-labels.sty   |
      | the gallery document per section |
    Then every one of them equals the file on disk byte for byte
    And each carries the generated-file header

  @id-api-reference
  @level-long
  @mode-conformance
  Scenario: The machine-readable API reference is in sync with the LaTeX sources
    Given the `🖼️assets/🔣️viz-api.json` that `generate viz` would write
    Then it equals the file on disk byte for byte
    And it carries the generated-file marker
