@capability-staffage-schema
Feature: The JSON Schema is the normative contract
  `🧬️schema/🔣️.json` (draft-07, `$id` semio.staffage/v1) defines one `$defs` entry per type of the TypeScript twin, with the same name, the same enums and
  `additionalProperties: false`. A category document is `{ $schema, version, category, assets }`; the manifest is `{ $schema, version, categories,
  collections, entries }`; every Text requires `en` and `de`; a Shape is exactly one of the five geometries plus the common fields.

  THE REFERENCE is ajv, an independent JSON Schema validator: the reference walker must pass, every broken variant of the fixture must fail, and the enums of
  the schema must equal the constants of the TypeScript twin.

  Scenario: The reference walker conforms
    Given the schema and the reference walker asset
    When ajv validates the asset against `#/$defs/Asset`
    Then it is valid

  Scenario: Broken variants are rejected
    Given the reference walker with a defect such as a missing German text or a capsule with ellipse fields
    When ajv validates it
    Then it is invalid

  Scenario: The twin matches
    Given the enum constants of the TypeScript module
    When compared with the `enum` lists of the schema
    Then they are identical
