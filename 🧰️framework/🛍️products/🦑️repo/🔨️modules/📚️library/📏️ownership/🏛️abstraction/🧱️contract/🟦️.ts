import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../🔍️discovery/📖️source-access/🟦️.ts";

export type AbstractionOwnership = { owner: "os" | "surface" | "artifact"; fields: readonly string[]; commands: readonly string[] };

export type AbstractionOwnershipSchema = { $defs: { OsField: { enum: string[] }; OsCommand: { enum: string[] }; ArtifactExcludedField: { enum: string[] } } };

/** 📜️ Loads the normative OS versus surface ownership contract as admitted source data. */
export function abstractionOwnershipSchema(root: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): AbstractionOwnershipSchema {
  const path = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧬️schema/🔣️.json",
    source = policySourceText(root, path, operations);
  if (source.state !== "file") throw new Error(`Abstraction ownership schema ${path} is ${source.state}.`);
  return JSON.parse(source.text) as AbstractionOwnershipSchema;
}

/** ⚖️ Reports OS-owned declarations incorrectly stored or executed by a surface. */
export function abstractionOwnershipViolations(declaration: AbstractionOwnership, schema: AbstractionOwnershipSchema): string[] {
  if (declaration.owner === "artifact") return declaration.fields.filter((field) => schema.$defs.ArtifactExcludedField.enum.includes(field)).map((field) => `field:${field}`);
  if (declaration.owner !== "surface") return [];
  const fields = new Set(schema.$defs.OsField.enum),
    commands = new Set(schema.$defs.OsCommand.enum);
  return [...declaration.fields.filter((field) => fields.has(field)).map((field) => `field:${field}`), ...declaration.commands.filter((command) => commands.has(command)).map((command) => `command:${command}`)];
}
