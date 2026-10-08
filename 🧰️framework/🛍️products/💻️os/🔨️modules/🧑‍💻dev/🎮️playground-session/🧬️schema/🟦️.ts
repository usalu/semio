import schema from "./🔣️.json";

export interface PlaygroundSessionPublicationRequestV1 { readonly contractId: string; readonly variant: string; readonly catalogPath: string; readonly outputRoot: string }

/** 🎮️ Admits explicit selection, catalog and output authority without an ambient inventory. */
export function parsePlaygroundSessionPublicationRequestV1(value: unknown): PlaygroundSessionPublicationRequestV1 {
  const refuse = (): never => { throw new Error("Invalid explicit playground session publication request"); };
  if (!value || typeof value !== "object" || Array.isArray(value)) return refuse();
  const row = value as Record<string, unknown>;
  if (schema.required.some(key => !Object.hasOwn(row, key)) || Object.keys(row).some(key => !schema.required.includes(key))) return refuse();
  if (typeof row.contractId !== "string" || row.contractId.length > schema.properties.contractId.maxLength || !new RegExp(schema.properties.contractId.pattern, "u").test(row.contractId)) return refuse();
  if (typeof row.variant !== "string" || row.variant.length > schema.properties.variant.maxLength || !new RegExp(schema.properties.variant.pattern, "u").test(row.variant)) return refuse();
  const path = (value: unknown, empty: boolean): string => {
    if (typeof value !== "string" || value.length > schema.properties.catalogPath.maxLength || (!empty && !value) || value !== value.normalize("NFC") || value.includes("\\") || /[:\u0000-\u001f]/u.test(value) || (value && value.split("/").some(segment => !segment || segment === "." || segment === ".."))) return refuse();
    return value;
  };
  const catalogPath = path(row.catalogPath, true), outputRoot = path(row.outputRoot, false);
  if (!catalogPath && row.variant) return refuse();
  return Object.freeze({ contractId: row.contractId, variant: row.variant, catalogPath, outputRoot });
}
