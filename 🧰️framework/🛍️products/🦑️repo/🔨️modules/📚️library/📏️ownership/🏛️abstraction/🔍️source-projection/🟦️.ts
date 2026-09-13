import { inspectRustStructure } from "../../../🔍️discovery/🟦️.ts";

/** 🧬️ Finds surface-owned fields in nested authored JSON schemas. */
export function abstractionOwnershipSchemaFields(schema: Record<string, unknown>): string[] {
  const fields = new Set<string>();
  const visit = (value: unknown): void => {
    if (!value || typeof value !== "object" || Array.isArray(value)) return;
    const record = value as Record<string, unknown>;
    if (record["x-semio-state"] === "artifact") return;
    for (const [name, field] of Object.entries((record.properties ?? {}) as Record<string, unknown>)) {
      if (field === false || (field && typeof field === "object" && (field as Record<string, unknown>)["x-semio-state"] === "artifact")) continue;
      fields.add(name);
      visit(field);
    }
    for (const key of ["$defs", "definitions", "patternProperties", "dependentSchemas"]) {
      for (const field of Object.values((record[key] ?? {}) as Record<string, unknown>)) visit(field);
    }
    for (const key of ["allOf", "anyOf", "oneOf", "prefixItems"]) {
      if (Array.isArray(record[key])) for (const field of record[key] as unknown[]) visit(field);
    }
    for (const key of ["items", "contains", "additionalProperties", "then", "else"]) visit(record[key]);
  };
  visit(schema);
  return [...fields];
}

/** 🎮️ Discovers authored command and mutation variants independently of leaf folder names. */
export function abstractionOwnershipRustCommands(source: string): string[] {
  return inspectRustStructure(source)
    .enums.filter((item) => /(?:Command|Mutation)$/.test(item.name))
    .flatMap((item) => item.variants.map((variant) => variant.name.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase()));
}
