Feature: Closed Print Inference Result Contract
  Scenario: Owned plans and scenes are admitted consistently
    Given the language-neutral inference-result base and mutation vectors
    When each derived result is admitted
    Then typed geometry, paints, frames, themes and diagnostics equal independent AJV admission
    And malformed variants and partial failed publications are rejected

  Scenario: Every owned drawing vocabulary remains admissible
    Given the all-valid-render-and-scene-variants language-neutral result
    When the complete result is admitted
    Then all six render items and eight drawing nodes equal independent AJV admission
    And all path commands, path segments, paint variants and styled strokes are admitted
    And incomplete ellipse, gradient, stroke and text variants are rejected

  Scenario: Scene text retains printed physical size
    Given the fontPoints language-neutral vectors
    When the same print plan is projected into a millimetre scene
    Then text size equals the independent D3 unit scale
    And native TeX point dimensions retain the same physical size

  Scenario: Native print style remains a backend customization
    Given language-neutral TikZ style strings on all six print plan item variants
    When the result is admitted by the owned contract and independent AJV
    Then style text is accepted in the print plan
    And numeric and object style values and portable scene style properties are rejected

  Scenario: Printed text admits owned vertical alignment
    Given alphabetic, middle, top and bottom text baseline vectors
    When complete plans are admitted by the owned contract and independent AJV
    Then all four text baselines are admitted
    And unknown baselines and baseline properties on rectangles are rejected

  Scenario: Text Font Families Remain an Owned Portable Contract
    Given nonempty tracked and custom font family names on text plans and scenes
    When the owned contract and independent AJV admit the results
    Then the font family names are preserved
    And empty and numeric font names and font properties on rectangles are rejected

  Scenario: Native Chrome Paints Are Explicit Inference Theme Values
    Given resolved native foreground and border paints in the print theme
    When the owned closed contract and independent AJV admit the result
    Then authored hexadecimal chrome paints are preserved
    And missing role paints, empty strings and numeric paints are rejected
