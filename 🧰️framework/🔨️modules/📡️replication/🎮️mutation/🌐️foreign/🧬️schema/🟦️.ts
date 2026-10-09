/** 🌐️ Borrowed foreign payload admission preserves the original source and sparse dialect. */
export interface ForeignTarget { readonly artifactId: string; readonly artifactKind: string; readonly dialect?: string | null; }
export interface ForeignStep { readonly target: ForeignTarget; readonly mutationId: string; readonly payload: readonly number[]; readonly label: string; }
function record(value: unknown): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("foreign payload must be an object");
  return value as Record<string, unknown>;
}
/** 🌉️ Admits a genuine target without creating replacement text or owner buffers. */
export function parseForeignTarget(value: unknown): ForeignTarget {
  const row = record(value);
  if (typeof row.artifactId !== "string" || typeof row.artifactKind !== "string" || (row.dialect !== undefined && row.dialect !== null && typeof row.dialect !== "string")) throw new Error("foreign target fields are invalid");
  return value as ForeignTarget;
}
/** 🪜️ Admits original octets and target fields without copying the proposal payload. */
export function parseForeignStep(value: unknown): ForeignStep {
  const row = record(value);
  parseForeignTarget(row.target);
  if (typeof row.mutationId !== "string" || typeof row.label !== "string" || !Array.isArray(row.payload)) throw new Error("foreign step fields are invalid");
  for (let index = 0; index < row.payload.length; index++) {
    const byte = row.payload[index];
    if (typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255) throw new Error("foreign step octets are invalid");
  }
  return value as unknown as ForeignStep;
}

/** 🧺️ Admits the original produced foreign row sequence without replacing its backing. */
export function parseForeignSteps(value: unknown): readonly ForeignStep[] {
  if (!Array.isArray(value)) throw new Error("foreign steps must be an array");
  for (let index = 0; index < value.length; index++) parseForeignStep(value[index]);
  return value as readonly ForeignStep[];
}
