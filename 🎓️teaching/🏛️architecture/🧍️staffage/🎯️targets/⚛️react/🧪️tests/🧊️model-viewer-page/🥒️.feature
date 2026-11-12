Feature: The 3D model in the viewer and the address
  Scenario: The address
    When a link carries vp=model, vp=axonometric and cam=front
    Then they are read, written canonically with the defaults dropped and an unknown viewpoint or camera is reported in words
    And the comparison and the playground say they draw planar viewpoints only and compare in the side elevation
  Scenario: The viewpoint selector
    Then it is one radio group of the three viewpoints, the sheet, the axonometric view and the 3D model in English and German
    And an asset without a volume keeps both new choices with the reason
  Scenario: The viewer in 3D
    When the 3D model is chosen
    Then one patch changes the viewpoint, pose, time and scale stay, the stage and the shared transport are shown, the 2D zoom controls are not, and the camera is written back to the address
