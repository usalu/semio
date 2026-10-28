Feature: Original glTF and GLB source ownership
  Scenario Outline: Loaded source becomes private before semantic work
    Given the original pinned Khronos Box artifact
    When caller metadata and bytes change at <phase> after completed source capture
    Then <route> retains the original complete supported outcome
    And source accessors are never invoked during admission
    Examples:
      | route      | phase         |
      | importGltf | gltf-import   |
      | importGltf | gltf-accessor |
      | writeGlb   | glb           |
      | writeGlb   | glb-bytes     |
  Scenario: Raw binary input becomes private before header parsing
    Given a genuine Uint8Array view of the pinned Khronos Box GLB
    When caller bytes change during parsing or output materialization
    Then the loaded glTF document and complete buffer equal the admitted private source
    And built-in view slots bypass caller property accessors
  Scenario: Original cancellation and bounds govern capture
    Given finite source value, depth, and record limits
    When admission exceeds a limit or the original signal cancels at an interior checkpoint
    Then no artifact is published and semantic work has not started
    And replacing the caller control object does not replace original authority
