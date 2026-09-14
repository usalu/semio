@capability-presentation-markdown-html
@oracle-remark
@comparison-ordered-json-v1
Feature: Slide markdown compiles to the HTML remark compiles it to
  A slide body is authored in markdown and rendered as an HTML fragment inside a reveal.js section.
  The product owns that compiler — `🔨️modules/📝️markdown-html-compiler` of the React target — because
  a deck must not carry a markdown runtime, and because the fragment it emits is a safety boundary:
  raw HTML blocks are dropped and only `http`, `https`, `mailto` and `tel` links survive.

  Owning a markdown compiler means owning the risk that it agrees only with itself. The reference is
  the `remark` stack — `unified` + `remark-parse` + `remark-gfm` + `remark-rehype` +
  `rehype-stringify` — the implementation CommonMark and GFM are read through in the JavaScript
  ecosystem. Every vector below is a markdown fixture both producers compile; the projection carries
  the resulting fragment per vector, so a scenario names the constructs it fixes rather than a hash.

  Both sides are normalised the same way, and deliberately as little as possible: line endings are
  brought to `\n` so a checkout on Windows reads like a checkout on Linux, and the fragment's outer
  whitespace is trimmed. Nothing inside the fragment is touched. Collapsing whitespace between tags
  would have been the convenient normalisation and is refused here: the space in
  `<em>slide</em> <strong>title</strong>` is the space a reader sees, so a compiler that loses it
  must fail this case rather than be normalised into agreement.

  The vectors are deliberately the subset of markdown the owned compiler implements. It does not
  implement block quotes, thematic breaks, images, setext headings, strikethrough, task lists,
  reference links, indented code blocks or loose lists, and a deck that uses one of those gets the
  literal text rather than the construct. Fixing those divergences here would freeze them; the case
  therefore states what the compiler claims, and the unclaimed constructs stay out of the contract
  until the compiler grows them.

  @id-prose
  @level-fundamental
  @mode-differential
  Scenario: Paragraphs and headings compile to the same block elements
    Given the slide markdown vectors
      | vector               | fixture                          |
      | paragraph            | local://paragraph.md             |
      | paragraph-soft-break | local://paragraph-soft-break.md  |
      | headings             | local://headings.md              |
      | heading-inline       | local://heading-inline.md        |
    When each vector is compiled to an HTML fragment
    Then the owned compiler and the reference implementation agree on every fragment

  @id-inline
  @level-fundamental
  @mode-differential
  Scenario: Emphasis, inline code, escapes and hard breaks compile to the same inline elements
    Given the slide markdown vectors
      | vector                  | fixture                             |
      | emphasis-asterisk       | local://emphasis-asterisk.md        |
      | emphasis-underscore     | local://emphasis-underscore.md      |
      | inline-code             | local://inline-code.md              |
      | inline-code-double-tick | local://inline-code-double-tick.md  |
      | escapes                 | local://escapes.md                  |
      | hard-break              | local://hard-break.md               |
    When each vector is compiled to an HTML fragment
    Then the owned compiler and the reference implementation agree on every fragment

  @id-lists
  @level-quick
  @mode-differential
  Scenario: Unordered and ordered lists compile to the same list elements
    Given the slide markdown vectors
      | vector               | fixture                          |
      | list-unordered       | local://list-unordered.md        |
      | list-unordered-plus  | local://list-unordered-plus.md   |
      | list-unordered-star  | local://list-unordered-star.md   |
      | list-ordered         | local://list-ordered.md          |
      | list-ordered-start   | local://list-ordered-start.md    |
      | list-ordered-paren   | local://list-ordered-paren.md    |
      | list-nested          | local://list-nested.md           |
      | list-inline          | local://list-inline.md           |
    When each vector is compiled to an HTML fragment
    Then the owned compiler and the reference implementation agree on every fragment

  @id-links
  @level-quick
  @mode-differential
  Scenario: Links and autolinks compile to the same anchors
    Given the slide markdown vectors
      | vector       | fixture                  |
      | link         | local://link.md          |
      | link-title   | local://link-title.md    |
      | link-inline  | local://link-inline.md   |
      | link-mailto  | local://link-mailto.md   |
      | autolink     | local://autolink.md      |
    When each vector is compiled to an HTML fragment
    Then the owned compiler and the reference implementation agree on every fragment

  @id-code
  @level-quick
  @mode-differential
  Scenario: Fenced code blocks keep their language, their escaping and their trailing newline
    Given the slide markdown vectors
      | vector               | fixture                          |
      | code-fence-language  | local://code-fence-language.md   |
      | code-fence-plain     | local://code-fence-plain.md      |
      | code-fence-escaping  | local://code-fence-escaping.md   |
      | code-fence-tilde     | local://code-fence-tilde.md      |
    When each vector is compiled to an HTML fragment
    Then the owned compiler and the reference implementation agree on every fragment

  @id-tables
  @level-long
  @mode-differential
  Scenario: GFM tables compile to the same table, with the same column alignment
    Given the slide markdown vectors
      | vector        | fixture                   |
      | table-aligned | local://table-aligned.md  |
      | table-plain   | local://table-plain.md    |
    When each vector is compiled to an HTML fragment
    Then the owned compiler and the reference implementation agree on every fragment

  @id-slide
  @level-long
  @mode-differential
  Scenario: A whole slide body compiles to the same fragment
    Given the slide markdown vectors
      | vector | fixture           |
      | slide  | local://slide.md  |
    When each vector is compiled to an HTML fragment
    Then the owned compiler and the reference implementation agree on every fragment
