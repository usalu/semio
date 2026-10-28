Feature: Normative authored BIM horizontal frames
  Scenario Outline: Export and reimport a building rotation
    Given an authored building rotation from bim-axis-cases.json
    When the actual BIM-to-IFC spatial exporter creates its local frame
    Then Axis and RefDirection are both absent or both explicit
    And the IFC source passes independent complete EXPRESS validation
    And the original rotation and origin survive supported BIM reimport

