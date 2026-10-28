Feature: GLB framing and represented transform orientation
  Scenario Outline: Read each shared GLB framing case
    Given the neutral glTF JSON text and the case's JSON tail
    And its binary padding and optional future chunk after BIN
    When either language reads the GLB frame
    Then its acceptance equals the neutral case
    And an accepted frame retains the single embedded byte

  Scenario Outline: Import a nonsingular represented transform
    Given the original Khronos Box GLB with a neutral scale
    When it is imported as IFC triangulated geometry
    Then its world bounds equal the independent three.js result
    And a tiny negative determinant retains its reflection sign

  Scenario Outline: Classify a represented binary64 determinant
    Given the neutral nine linear coefficients in row order
    When either language evaluates the exact determinant sign
    Then the sign equals the neutral expectation
    And an independent high precision mpmath LU decomposition agrees
    And nonfinite matrix entries cannot publish an orientation

  Scenario Outline: Retain the original caller cancellation
    Given one neutral glTF exchange route with an original abort signal
    When its first callback replaces the caller options and aborts the original signal
    Then the exchange refuses as cancelled without publishing a result
