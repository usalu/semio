@capability-bcf-2-1-viewpoint-mutate
@oracle-jszip-bcf-2-1-mutate-reader
@comparison-semantic-bcf-v1
@mutations-bcf-2.1-viewpoint
Feature: Apply every typed BCF 2.1 mutation to a real-world coordination review
  See ../⚪️mutate-bcf-2-1/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real coordination review
    Given the real input document shared://wellness-center-coordination-review.bcf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                        | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
      | insert-viewpoint          | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f11", "viewpoint": {"guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f34", "camera": {"kind": "orthogonal", "viewPoint": {"x": 0, "y": 0, "z": 2.1}, "direction": {"x": 0, "y": 1, "z": 0}, "upVector": {"x": 0, "y": 0, "z": 1}, "viewToWorldScale": 0.9}, "components": {"selection": ["3frUtLYhn2wRjxngLympdN"], "visibility": {"defaultVisibility": true, "exceptions": []}, "coloring": [{"color": "FF00FF00", "components": ["3frUtLYhn2wRjxngLympdN"]}]}, "snapshot": "89504e470d0a1a0a0000000d4948445200000040000000400802000000250be6890000007d49444154789cd5ce410d002000c4b031ff9a2f88e04156053ddb28933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889f377e0d505e1e1037a40fb9a070000000049454e44ae426082"}}                             |
      | remove-viewpoint          | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f11", "guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f14"}                                                                                                                                                                                                                                                                                                                                                                                                                                       |
      | set-viewpoint-camera      | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f01", "guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f03", "camera": {"kind": "orthogonal", "viewPoint": {"x": 6, "y": 4, "z": 2.5}, "direction": {"x": -1, "y": 0, "z": 0}, "upVector": {"x": 0, "y": 0, "z": 1}, "viewToWorldScale": 1.5}}                                                                                                                                                                                                                                                    |
      | set-viewpoint-components  | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f01", "guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f03", "components": {"selection": ["0HG2A49bzDARlPHy2ZDHwJ", "0PfeWE7Aj7GBHCsLa67379", "2lrUU8Tqz92AICLQu1TLwD"], "visibility": {"defaultVisibility": true, "exceptions": []}, "coloring": [{"color": "FF00FF00", "components": ["2lrUU8Tqz92AICLQu1TLwD"]}]}}                                                                                                                                                                            |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the coordination review
    Given the real input document shared://wellness-center-coordination-review.bcf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                        | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
      | insert-viewpoint          | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f11", "viewpoint": {"guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f34", "camera": {"kind": "orthogonal", "viewPoint": {"x": 0, "y": 0, "z": 2.1}, "direction": {"x": 0, "y": 1, "z": 0}, "upVector": {"x": 0, "y": 0, "z": 1}, "viewToWorldScale": 0.9}, "components": {"selection": ["3frUtLYhn2wRjxngLympdN"], "visibility": {"defaultVisibility": true, "exceptions": []}, "coloring": [{"color": "FF00FF00", "components": ["3frUtLYhn2wRjxngLympdN"]}]}, "snapshot": "89504e470d0a1a0a0000000d4948445200000040000000400802000000250be6890000007d49444154789cd5ce410d002000c4b031ff9a2f88e04156053ddb28933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889933889f377e0d505e1e1037a40fb9a070000000049454e44ae426082"}}                             |
      | remove-viewpoint          | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f11", "guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f14"}                                                                                                                                                                                                                                                                                                                                                                                                                                       |
      | set-viewpoint-camera      | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f01", "guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f03", "camera": {"kind": "orthogonal", "viewPoint": {"x": 6, "y": 4, "z": 2.5}, "direction": {"x": -1, "y": 0, "z": 0}, "upVector": {"x": 0, "y": 0, "z": 1}, "viewToWorldScale": 1.5}}                                                                                                                                                                                                                                                    |
      | set-viewpoint-components  | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f01", "guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f03", "components": {"selection": ["0HG2A49bzDARlPHy2ZDHwJ", "0PfeWE7Aj7GBHCsLa67379", "2lrUU8Tqz92AICLQu1TLwD"], "visibility": {"defaultVisibility": true, "exceptions": []}, "coloring": [{"color": "FF00FF00", "components": ["2lrUU8Tqz92AICLQu1TLwD"]}]}}                                                                                                                                                                            |
