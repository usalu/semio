/** 🧬️ Ifc2x3Artifact schema. */
export interface Ifc2x3EdmPreamble {
  producer: string; module: string; creationDate: string; host: string; database: string; databaseVersion: string;
  databaseCreationDate: string; schema: string; model: string; modelCreationDate: string; headerModel: string;
  headerModelCreationDate: string; user: string; group: string; license: string; options: string;
}
export interface Ifc2x3Artifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ document: unknown;
  /** @state artifact */ edmPreamble?: Ifc2x3EdmPreamble;
}
