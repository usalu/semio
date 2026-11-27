@capability-staffage-scoped-ids
Feature: Scene-scoped ids of a 3D model stay within the Slug limit
  Every solid, material, node, mesh and skin of a scene carries the scene's id as prefix. The `Slug` of the schema allows 64 characters at most, so a scoped id that would be longer
  keeps a readable head and ends in a stable hash of the full text. The rule is one function for the procedural model and the resolved model.

  THE REFERENCES are Ajv with the `Slug` of the staffage schema and Ajv with the kernel schema `semio.model/v1` for exported documents.

  Scenario: A short id is the plain prefix
    Given a scene `s50-body` and the id `torso`
    Then the scoped id is `s50-body-torso`

  Scenario: A long id is bounded, readable, deterministic and distinct
    Given scenes and ids of up to 120 characters, some sharing their first 60 characters
    When their scoped ids are made twice
    Then every one is a valid Slug of at most 64 characters, starts with the head of its scene and id, is the same both times, and no two differ in meaning but are equal

  Scenario: A catalog asset with long names exports valid ids
    Given the operating table and its first pose and clip
    When the procedural and the resolved model are written
    Then every node, solid, material, mesh and skin id is a valid Slug and unique, and the document validates with Ajv
