import schema from "./🧬️schema/🔣️.json";

const selection = schema.$defs.SelectionV1;
const variantSpec = selection.items.properties.variant;
const variantPattern = new RegExp(variantSpec.pattern, "u");

/** 🎮️Admits a selected variant at an operation that needs a concrete playground. */
export function requirePlaygroundVariant(input: unknown): string {
  if (typeof input !== "string" || [...input].length > variantSpec.maxLength || !variantPattern.test(input)) throw Error("A declared playground variant is required");
  return input;
}

/** ⭐️Selects only the default authority explicitly granted by a present owner. */
export function defaultPlaygroundVariant(input: unknown): string | undefined {
  if (!Array.isArray(input) || input.length > selection.maxItems) throw Error("Invalid playground default catalog");
  let selected: string | undefined;
  for (const row of input) {
    if (row === null || typeof row !== "object" || Array.isArray(row)) throw Error("Invalid playground default row");
    if (Object.keys(row).some(key => !Object.hasOwn(selection.items.properties, key))) throw Error("Undeclared playground default field");
    const variant = requirePlaygroundVariant(row.variant);
    if (row.catalogDefault !== undefined && typeof row.catalogDefault !== "boolean") throw Error("Invalid playground default declaration");
    if (row.catalogDefault !== true) continue;
    if (selected !== undefined) throw Error("Multiple playground owners declare the catalog default");
    selected = variant;
  }
  return selected;
}

/** 📜️Reads one canonical boolean default declaration from its owning playground block. */
export function declaredPlaygroundCatalogDefaultV1(block: string): boolean | undefined {
  const rows = block.split(/\r?\n/u).map(line => line.trim()).filter(line => /^catalog-default\s*=/u.test(line));
  if (!rows.length) return undefined;
  if (rows.length !== 1) throw Error("Repeated playground default declaration");
  const value = rows[0]!.slice(rows[0]!.indexOf("=") + 1).trim().match(/^(true|false)\s*(?:#.*)?$/u)?.[1];
  if (value === undefined) throw Error("Invalid playground default declaration");
  return value === "true";
}
