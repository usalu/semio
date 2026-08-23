@capability-style-variant-compilation
@oracle-cva
@comparison-utf8-text-v1
Feature: Compile a typed style-variant selection
  `styleVariants()` re-implements the published `class-variance-authority` contract: a base class, a
  finite variant schema, default selections, boolean choices, compound conjunctions, and caller
  `class`/`className` overrides. Because it is a re-implementation of a documented contract rather
  than a repository invention, `cva` is a genuine oracle for it.

  One deliberate scoping note: `styleVariants()` finishes through `cn()`, which additionally resolves
  conflicts across repository-owned utility families, while `cva` finishes through plain flattening.
  Every vector below therefore uses UNCLASSIFIED tokens, where the two are genuinely comparable. The
  conflict-resolution half is specified separately by the 🏷️class-name-composition owner and must not
  be smuggled in here, where agreement would be an accident of the vectors rather than evidence.

  @id-base-only-and-caller-classes
  @level-fundamental
  @mode-differential
  Scenario: A base with no schema, plus caller-supplied classes
    Given the style-variant program
    """
    {
      "base": "base",
      "config": null,
      "selections": [null, { "class": ["class", false], "className": "class-name" }]
    }
    """
    When each selection is compiled
    Then the reference implementation and this repository produce the same class lists

  @id-single-variant-matrix
  @level-fundamental
  @mode-differential
  Scenario: The complete matrix of a single variant, including an explicit null opt-out
    Given the style-variant program
    """
    {
      "base": "control",
      "config": {
        "variants": { "variant": { "default": "default", "ghost": "ghost", "outline": "outline" } },
        "defaultVariants": { "variant": "default" }
      },
      "selections": [null, {}, { "variant": null }, { "variant": "default" }, { "variant": "ghost" }, { "variant": "outline" }]
    }
    """
    When each selection is compiled
    Then the reference implementation and this repository produce the same class lists

  @id-boolean-choices-and-compound-conjunctions
  @level-quick
  @mode-differential
  Scenario: Defaults, boolean choices and compound conjunctions
    Given the style-variant program
    """
    {
      "base": "base",
      "config": {
        "variants": { "size": { "sm": "sm", "lg": "lg" }, "active": { "true": "on", "false": "off" } },
        "defaultVariants": { "size": "sm", "active": false },
        "compoundVariants": [
          { "size": "sm", "active": false, "class": "sm-off" },
          { "size": "sm", "active": true, "className": "sm-on" },
          { "size": "lg", "class": "large" },
          { "size": ["sm", "lg"], "class": "any-size" }
        ]
      },
      "selections": [
        null,
        { "size": "sm", "active": true, "className": "caller" },
        { "size": "lg", "active": false },
        { "size": "lg", "active": true },
        { "size": null, "active": true }
      ]
    }
    """
    When each selection is compiled
    Then the reference implementation and this repository produce the same class lists
