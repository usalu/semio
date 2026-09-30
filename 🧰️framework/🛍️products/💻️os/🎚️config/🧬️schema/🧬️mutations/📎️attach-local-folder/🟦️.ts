/** 📎️ Authoritative direct TypeScript leaf for remembering which local folder one program's document is attached to on this device. */

import type { LocalFoldersConfigMutation } from "../🟦️.ts";
import { detachLocalFolder } from "../✂️detach-local-folder/🟦️.ts";

//#region 🔖️Schema
/** 📁️ Where a document's folder is on this device: an absolute path the host opens. */
export type LocalFolderRef = { readonly kind: "path"; readonly path: string };

/** 📎️ One document this device attached to a local folder: the document's own identity, the program that holds it and the
 * folder its archive persists in. */
export interface LocalFolderBinding {
  readonly documentId: string;
  readonly pluginId: string;
  readonly appId: string;
  readonly folder: LocalFolderRef;
}

/** 📁️ `os.config.local-folders` — every folder binding of this device (persisted local-only: never shared, never in a URL),
 * one per document, ordered by document id. */
export interface LocalFolderBindings {
  readonly bindings: readonly LocalFolderBinding[];
}

/** 📁️ The schema id for the local folder binding config facet. */
export const LOCAL_FOLDERS_CONFIG_SCHEMA = "os.config.local-folders";
//#endregion 🔖️Schema

//#region 🔖️Mutation
/** 📎️ Remembers one document's folder, replacing any binding of the same document. */
export type AttachLocalFolder = LocalFolderBinding;

/** 🏗️ Wraps an attach payload in the local-folders dispatch union. */
export function attachLocalFolder(binding: LocalFolderBinding): LocalFoldersConfigMutation {
  return { mutation: "attachLocalFolder", documentId: binding.documentId, pluginId: binding.pluginId, appId: binding.appId, folder: { kind: binding.folder.kind, path: binding.folder.path } };
}

/** 🔺️ Replaces or inserts the binding of the payload's document, keeping the bindings ordered by document id; an identical
 * binding leaves them as they were. */
export function diff(payload: AttachLocalFolder, base: LocalFolderBindings): LocalFolderBindings {
  const attached: LocalFolderBinding = { documentId: payload.documentId, pluginId: payload.pluginId, appId: payload.appId, folder: { kind: payload.folder.kind, path: payload.folder.path } };
  if (base.bindings.some((entry) => sameBinding(entry, attached))) return base;
  return { bindings: [...base.bindings.filter((entry) => entry.documentId !== attached.documentId), attached].sort((left, right) => (left.documentId < right.documentId ? -1 : left.documentId > right.documentId ? 1 : 0)) };
}

/** ↩️ Re-attaches the prior binding of the document, or detaches a document that had none. */
export function inverse(payload: AttachLocalFolder, base: LocalFolderBindings): LocalFoldersConfigMutation[] {
  const prior = base.bindings.find((entry) => entry.documentId === payload.documentId);
  return prior ? [attachLocalFolder(prior)] : [detachLocalFolder(payload.documentId)];
}

function sameBinding(left: LocalFolderBinding, right: LocalFolderBinding): boolean {
  return left.documentId === right.documentId && left.pluginId === right.pluginId && left.appId === right.appId && left.folder.kind === right.folder.kind && left.folder.path === right.folder.path;
}
//#endregion 🔖️Mutation
