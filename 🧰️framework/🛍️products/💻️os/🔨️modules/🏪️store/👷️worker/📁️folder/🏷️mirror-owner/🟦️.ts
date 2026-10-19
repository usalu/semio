import type { PersistenceBinding } from "../../../../../🟦️";

/** 🏷️ The private owner of one canonical-bootstrap folder mirror: the folder binding, document and mirror epoch that reserved it and the capability that stages and publishes it. Types only, so the document state and the wire bridge hold it without importing the folder module. */
export type FolderCanonicalBootstrapMirrorOwner = Readonly<{
  binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>;
  documentId: string;
  epoch: number;
  capability: string;
}>;
