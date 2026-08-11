/** 🔺️ Ifc2x3Diff schema. */
export interface Ifc2x3Diff {
  schema?: string;
  header?: unknown;
  removedInstances?: number[];
  upsertedInstances?: unknown[];
}
