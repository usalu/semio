@capability-class-name-flattening
@oracle-clsx
@comparison-utf8-text-v1
Feature: Flatten recursive class-name inputs
  The repository owns `cn()`, whose FLATTENING half — recursive arrays, truthy object keys, falsey
  suppression, numeric coercion — is exactly the contract `clsx` implements. That half therefore has
  a credible reference implementation and is tested differentially against it.

  The CONFLICT-RESOLUTION half is deliberately NOT tested here: `cn()` resolves conflicts over
  repository-owned utility groups (`ui-surface`, `px-tiny`, `text-element`, …) that no third-party
  merger knows about, so no library is a valid oracle for it. That half is specified by vectors in
  the sibling `merge-conflicting-utilities` case. Every vector below is free of conflicting
  utilities, so the two implementations are genuinely comparable rather than accidentally agreeing.

  @id-flattens-nested-arrays-and-objects
  @level-fundamental
  @mode-differential
  Scenario: Recursively flatten arrays and truthy object keys
    Given the class-name inputs
    """
    ["relative", ["flex", [false, "h-full", null]], { "pointer-events-none": true, "hidden": false }, 0, 2]
    """
    When the inputs are composed
    Then the reference implementation and this repository produce the same class list

  @id-suppresses-every-falsey-value
  @level-fundamental
  @mode-differential
  Scenario: Suppress every falsey value
    Given the class-name inputs
    """
    [null, false, 0, "", "block", [null, [false, "gap-y-single"]]]
    """
    When the inputs are composed
    Then the reference implementation and this repository produce the same class list

  @id-preserves-unclassified-application-classes
  @level-fundamental
  @mode-differential
  Scenario: Preserve unclassified application classes, duplicates included
    Given the class-name inputs
    """
    ["introduction-demo-callout", "selection-marquee", "introduction-demo-callout"]
    """
    When the inputs are composed
    Then the reference implementation and this repository produce the same class list

  @id-flattens-deeply-nested-mixed-input
  @level-quick
  @mode-differential
  Scenario: Flatten a deeply nested mixture of every supported input shape
    Given the class-name inputs
    """
    [
      "sticky",
      ["inline-flex", ["items-center", { "justify-between": true, "justify-around": false }]],
      { "select-none": 1, "select-all": 0 },
      [[["shrink-0"]]],
      undefined,
      "z-overlay"
    ]
    """
    When the inputs are composed
    Then the reference implementation and this repository produce the same class list
