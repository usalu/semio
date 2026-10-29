Feature: PNG sequence ZIP reopens in independent readers
  Scenario: The walk cycle exported as PNG sequence is a valid ZIP with a manifest
    Given the walk cycle of the walking commuter at 1:100 and 12 fps
    When it is exported as png-sequence
    Then unzip -t reports no errors and python zipfile finds no bad member
    And the archive lists manifest.json and frame-000000.png to frame-000013.png in order
    And every frame starts with the PNG signature
    And the manifest states fps 12, 14 frames, exact frame times n / 12, the asset, the view and the scale 1:100
