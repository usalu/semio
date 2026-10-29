Feature: Academic thesis declarations
  Thesis authors explicitly choose English or German and a document theme.
  Bachelor, master, doctoral and custom theses share academic metadata.

  Scenario: Schema and independent validator agree
    Given every language-neutral thesis declaration fixture
    When the owned validator and AJV evaluate each declaration
    Then both return the fixture validity
    And custom degrees require a nonempty authored thesis heading

  Scenario: Safe preamble inference
    Given every shared thesis preamble fixture
    When the thesis preamble is inferred
    Then the output equals the language-neutral expected TeX
    And metadata characters remain literal text

  Scenario: Invalid metadata cannot emit TeX
    Given every invalid thesis declaration fixture
    When a thesis preamble is requested
    Then inference fails before emitting TeX

  Scenario: Actual academic documents retain their metadata
    Given each English and German bachelor, master, doctoral and custom thesis in both themes
    When the production compiler renders the authored declaration
    Then the independent PDF reader finds the authored academic metadata and localized thesis heading
    And front matter, abstract languages, declaration and research body remain readable

  Scenario: The actual compiler rejects incomplete declarations
    Given a thesis with missing language, missing level, invalid level or a missing custom heading
    When the production compiler evaluates the document
    Then the compiler refuses with the expected diagnostic

  Scenario: Thesis matter transitions retain numbering intent
    Given a title page and a caller-selected section numbering depth
    When front matter is entered twice
    Then Roman page numbering continues after the title and does not restart
    And generic front matter chapters remain unnumbered
    When main matter is entered
    Then Arabic page numbering starts at one and the caller depth is restored
    When back matter is entered
    Then generic chapters do not increment the chapter number

  Scenario: Doctoral specialization obeys custom heading validation
    Given a phd document with a custom level and no heading
    When the actual compiler evaluates the document
    Then it refuses with the custom heading diagnostic

  Scenario: Authorship and front matter customization retain explicit precedence
    Given each supported language with a default declaration
    When setup metadata customizes the declaration body and heading
    And an optional declaration body is authored for a later declaration
    Then the independent PDF reader finds the localized default body
    And the setup body and optional body each appear exactly once
    And the custom dedication and abbreviation headings appear
    And localized acknowledgements retain authored content

  Scenario: Minimal thesis metadata omits optional cover windows
    Given English master and German doctoral declarations with only required metadata
    When the actual compiler renders each complete document
    Then the title page contains its title, author and localized academic type
    And institutional, qualification, supervision, date and identity fields remain absent

  Scenario: Abstract language scopes restore the document language
    Given each supported document language
    When Abstract and Kurzfassung environments are rendered
    Then actual Babel transcript markers show english inside Abstract
    And ngerman inside Kurzfassung
    And the document language immediately after each environment

  Scenario: Bare rendering tests receive genuine declared authority
    Given a schema-admitted native build caller grant for the exact thesis rendering target
    When the official test issuer admits a bare test process
    Then its actual native request and workspace equal the authored declaration
    And the full rendering matrix runs under the production probe operation

  Scenario: Registered thesis and doctoral templates publish actual products
    Given a declared native grant naming the thesis and doctoral build targets
    When the production publisher compiles each registered template
    Then its exact light and dark PDFs are retained under their real producer IDs
    And the independent PDF reader consumes the expected academic content

  Scenario: Preserve Document Class Command Ownership
    Given a thesis using the KOMA document class
    When its academic shorthand commands are declared
    Then making a dedication uses makededication
    And the Dedication environment remains available
    And the document class dedication command is not redeclared

  Scenario: Private Native Birth Uses Declared Real Owner Sources
    Given exact live owner registry and Print project manifest snapshot inputs
    When the native issuer resolves the exact thesis rendering target
    Then its selected owner and target equal the neutral fixture
    And only those two declared metadata sources are read
    And consumed metadata retains original source hashes
