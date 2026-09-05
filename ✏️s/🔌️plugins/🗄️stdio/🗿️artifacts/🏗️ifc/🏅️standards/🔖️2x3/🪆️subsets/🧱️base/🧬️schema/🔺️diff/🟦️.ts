/** 🔺️ Ifc2x3Diff schema. */
export interface Ifc2x3EdmPreamble {
  producer: string; module: string; creationDate: string; host: string; database: string; databaseVersion: string;
  databaseCreationDate: string; schema: string; model: string; modelCreationDate: string; headerModel: string;
  headerModelCreationDate: string; user: string; group: string; license: string; options: string;
}
export interface Ifc2x3Diff {
  schema?: string;
  header?: unknown;
  removedInstances?: number[];
  upsertedInstances?: unknown[];
  edmPreamble?: Ifc2x3EdmPreamble | null;
  instanceOrder?: number[];
}
