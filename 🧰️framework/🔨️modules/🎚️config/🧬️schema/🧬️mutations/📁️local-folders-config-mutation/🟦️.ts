/** 📁️ Folder binding record, payloads, dispatch union and constructors of the local folder mutation vocabulary, shared by its two direct leaves. */

//#region 🔖️Schema
/** 📍️ Where a document's folder is on this device: an absolute path the host opens. */
export type LocalFolderRef = { readonly kind: "path"; readonly path: string };

/** 📎️ One document this device attached to a local folder: the document's own identity, the program that holds it and the
 * folder its archive persists in. */
export interface LocalFolderBinding {
  readonly documentId: string;
  readonly pluginId: string;
  readonly appId: string;
  readonly folder: LocalFolderRef;
}
//#endregion 🔖️Schema

//#region 🔖️Payloads
/** 🔗️ Remembers one document's folder, replacing any binding of the same document. */
export type AttachLocalFolder = LocalFolderBinding;

/** ✂️ Forgets the folder of one document, if it has one; the folder and the archive in it stay where they are. */
export interface DetachLocalFolder {
  readonly documentId: string;
}
//#endregion 🔖️Payloads

//#region 🔖️Dispatch
/** 🎚️ Typed, invertible local folder binding mutation vocabulary (persisted local-only). */
export type LocalFoldersConfigMutation = ({ readonly mutation: "attachLocalFolder" } & AttachLocalFolder) | ({ readonly mutation: "detachLocalFolder" } & DetachLocalFolder);

/** 🏗️ Wraps an attach payload in the local-folders dispatch union. */
export function attachLocalFolder(binding: LocalFolderBinding): LocalFoldersConfigMutation {
  return { mutation: "attachLocalFolder", documentId: binding.documentId, pluginId: binding.pluginId, appId: binding.appId, folder: { kind: binding.folder.kind, path: binding.folder.path } };
}

/** 🔨️ Wraps a detach payload in the local-folders dispatch union. */
export function detachLocalFolder(documentId: string): LocalFoldersConfigMutation {
  return { mutation: "detachLocalFolder", documentId };
}
//#endregion 🔖️Dispatch
