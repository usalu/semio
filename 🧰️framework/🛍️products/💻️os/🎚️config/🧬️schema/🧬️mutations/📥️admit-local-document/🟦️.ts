/** 📥️ Authoritative direct TypeScript leaf for listing one document in this device's local catalog. */

import type { LocalCatalogConfigMutation } from "../🟦️.ts";
import { retireLocalDocument } from "../📤️retire-local-document/🟦️.ts";

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

/** 🗂️ `os.config.local-catalog` — every document this device keeps locally (persisted local-only), ordered by id. */
export interface LocalCatalog {
  readonly documents: readonly LocalDocument[];
}

/** 🗂️ The schema id for the local document catalog config facet. */
export const LOCAL_CATALOG_CONFIG_SCHEMA = "os.config.local-catalog";
//#endregion 🔖️Schema

//#region 🔖️Mutation
/** 📥️ Lists one document in the local catalog, replacing any entry with the same id. */
export type AdmitLocalDocument = LocalDocument;

/** 🏗️ Wraps an admit payload in the local-catalog dispatch union. */
export function admitLocalDocument(document: LocalDocument): LocalCatalogConfigMutation {
  return { mutation: "admitLocalDocument", ...document };
}

/** 🔺️ Replaces or inserts the entry for the payload's id, keeping the catalog ordered by id. */
export function diff(payload: AdmitLocalDocument, base: LocalCatalog): LocalCatalog {
  const admitted: LocalDocument = { documentId: payload.documentId, schema: payload.schema, name: payload.name, storage: payload.storage, target: payload.target, admittedAtMs: payload.admittedAtMs };
  if (base.documents.some((entry) => sameDocument(entry, admitted))) return base;
  return { documents: [...base.documents.filter((entry) => entry.documentId !== admitted.documentId), admitted].sort((left, right) => (left.documentId < right.documentId ? -1 : left.documentId > right.documentId ? 1 : 0)) };
}

/** ↩️ Re-admits the prior entry, or retires the id that was not listed before. */
export function inverse(payload: AdmitLocalDocument, base: LocalCatalog): LocalCatalogConfigMutation[] {
  const prior = base.documents.find((entry) => entry.documentId === payload.documentId);
  return prior ? [admitLocalDocument(prior)] : [retireLocalDocument(payload.documentId)];
}

function sameDocument(left: LocalDocument, right: LocalDocument): boolean {
  return left.documentId === right.documentId && left.schema === right.schema && left.name === right.name && left.storage === right.storage && left.target === right.target && left.admittedAtMs === right.admittedAtMs;
}
//#endregion 🔖️Mutation
