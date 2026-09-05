@capability-bcf-2-1-snapshot-mutate
@oracle-jszip-bcf-2-1-mutate-reader
@comparison-semantic-bcf-v1
@mutations-bcf-2.1-snapshot
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
      | set-viewpoint-snapshot    | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f11", "guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f14", "snapshot": "89504e470d0a1a0a0000000d4948445200000040000000400802000000250be689000001f849444154789cedda316beb301405e0938b03c910702081041c48a183a0867ac898f141ff74c60c1d3c78d020a8200eb858832043021a2cc2a309bce1c17b4b25870bfd7ec095cfc192357870bd5ec1198139027304e608cc1198233047608ec01c8139027304e608cc1198233047608ec01c81390273c9a31676ced575cd3540d775bbdd4e6b9d2409cb004a29ad753a998897177e014ed65655e5bdfff5f6b65aad986de2aeebd4c747d334799e2f168bef0fec3b80b5564a994e26799e0f87436601beea57ca5a9bbfbe06a91f3d0730c69465996599102248fde83340d7756559264992e7f97c3e0f3596d097baaeb5d6ebf55a0811702ca117e7f3f94ffde3f1985f00adb531667d13763221beb66dabaa02b0dd6e43edddfe0238e7a494c698cd6693a669f0f984c88c315aebd96c16f0e8ec2fc0bdfecbe55214458cfa113b803146299565d9f3d3538cfa11358073eefdfd7d341a1545319dcd22ad42512ffd4dd33cdfc45b8522cd3d9d4e5555ddeb8ff4f2c40d20a5b4d60a2196cb2562a21843dbb6d55adfeb4764c4eecbf597c43987a0b4d652cab097feff48f6fb3dc2f1de9bcf4feffdd7d1399d22be442915709cbf114204bf75fecbe0783c2228ef7d9aa6fdd40f60f0f3b7ca831198233047608ec01c8139027304e608cc1198233047608ec01c8139027304e692c3e1f0e867f896df5352ca3341ca20590000000049454e44ae426082"} |

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
      | set-viewpoint-snapshot    | {"topicGuid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f11", "guid": "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f14", "snapshot": "89504e470d0a1a0a0000000d4948445200000040000000400802000000250be689000001f849444154789cedda316beb301405e0938b03c910702081041c48a183a0867ac898f141ff74c60c1d3c78d020a8200eb858832043021a2cc2a309bce1c17b4b25870bfd7ec095cfc192357870bd5ec1198139027304e608cc1198233047608ec01c8139027304e608cc1198233047608ec01c81390273c9a31676ced575cd3540d775bbdd4e6b9d2409cb004a29ad753a998897177e014ed65655e5bdfff5f6b65aad986de2aeebd4c747d334799e2f168bef0fec3b80b5564a994e26799e0f87436601beea57ca5a9bbfbe06a91f3d0730c69465996599102248fde83340d7756559264992e7f97c3e0f3596d097baaeb5d6ebf55a0811702ca117e7f3f94ffde3f1985f00adb531667d13763221beb66dabaa02b0dd6e43edddfe0238e7a494c698cd6693a669f0f984c88c315aebd96c16f0e8ec2fc0bdfecbe55214458cfa113b803146299565d9f3d3538cfa11358073eefdfd7d341a1545319dcd22ad42512ffd4dd33cdfc45b8522cd3d9d4e5555ddeb8ff4f2c40d20a5b4d60a2196cb2562a21843dbb6d55adfeb4764c4eecbf597c43987a0b4d652cab097feff48f6fb3dc2f1de9bcf4feffdd7d1399d22be442915709cbf114204bf75fecbe0783c2228ef7d9aa6fdd40f60f0f3b7ca831198233047608ec01c8139027304e608cc1198233047608ec01c8139027304e692c3e1f0e867f896df5352ca3341ca20590000000049454e44ae426082"} |
