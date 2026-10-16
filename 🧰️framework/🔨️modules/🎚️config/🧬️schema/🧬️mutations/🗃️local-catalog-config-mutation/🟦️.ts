/** 🗃️ Document record, payloads, dispatch union and constructors of the local document catalog mutation vocabulary, shared by its two direct leaves. */

//#region 🔖️Schema
/** 🗄️ Where a local document's events persist on this device: a folder event log or one file. */
export type LocalDocumentStorage = "folder" | "file";

/** 📄️ One document this device keeps locally: its id, artifact schema, name and where its events persist. */
export interface LocalDocument {
  readonly documentId: string;
  readonly schema: string;
  readonly name: string;
  readonly storage: LocalDocumentStorage;
  readonly target: string;
  readonly admittedAtMs: number;
}
//#endregion 🔖️Schema

//#region 🔖️Payloads
/** 📥️ Lists one document in the local catalog, replacing any entry with the same id. */
export type AdmitLocalDocument = LocalDocument;

/** 📤️ Removes one document from the local catalog, if listed; its own events stay where they persist. */
export interface RetireLocalDocument {
  readonly documentId: string;
}
//#endregion 🔖️Payloads

//#region 🔖️Dispatch
/** 🎚️ Typed, invertible local document catalog mutation vocabulary. */
export type LocalCatalogConfigMutation = ({ readonly mutation: "admitLocalDocument" } & AdmitLocalDocument) | ({ readonly mutation: "retireLocalDocument" } & RetireLocalDocument);

/** 🏗️ Wraps an admit payload in the local-catalog dispatch union. */
export function admitLocalDocument(document: LocalDocument): LocalCatalogConfigMutation {
  return { mutation: "admitLocalDocument", ...document };
}

/** 🔨️ Wraps a retire payload in the local-catalog dispatch union. */
export function retireLocalDocument(documentId: string): LocalCatalogConfigMutation {
  return { mutation: "retireLocalDocument", documentId };
}
//#endregion 🔖️Dispatch
