@capability-pdf-1-4-text-extraction
@oracle-pypdf-pdf-1-4-text
@comparison-ordered-json-v1
Feature: Recover the text layer of a real PDF 1.4 document
  The input is the real ~6.3MB, 65-page bachelor thesis committed under this standard's own
  examples directory — a LaTeX-produced document with embedded subset fonts, ligatures and a
  two-level heading structure, not a synthetic fixture. It is read where the domain already keeps
  it and never written to.

  This case exists because text extraction is the one PDF capability with no Rust reference in this
  repository. `lopdf`, the registered editing and parsing oracle, hands back the object graph and
  the raw content streams; turning those into a page's reading order means decoding the font's
  /Encoding and /ToUnicode CMaps, undoing glyph-index and ligature mappings and re-assembling the
  text-showing operators into lines. No crate linked by the oracle host does that, so the reference
  is `pypdf` and the oracle runs in Python.

  The evidence is the document's own printed content: the title page names its author, degree and
  institution, the abstract names its subject, and every one of the 65 pages carries text. Those
  are properties of the artifact, so the assertion holds independently of which library reads it.

  @id-declared-pages-carry-their-printed-text
  @level-quick
  @mode-conformance
  Scenario: The declared pages carry the text the document prints
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the reference implementation extracts the text layer of each declared page
      | page | contains                         |
      | 1    | Ueli Saluz                       |
      | 1    | Bachelor of Science Architecture  |
      | 1    | Technische Universität Berlin     |
      | 2    | Abstract                         |
      | 2    | construction industry            |
      | 5    | Computer science glossary        |
      | 11   | Architects know this problem very well |
      | 65   | BIBLIOGRAPHY                     |
    Then every declared page carries its declared text

  @id-every-page-yields-text
  @level-quick
  @mode-property
  Scenario: Every page of the document yields a non-empty text layer
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the reference implementation extracts the text layer of every page in turn
    Then the page count matches the document's own page tree and no page comes back empty
