/** ✂️ Authoritative direct TypeScript leaf for forgetting the local folder one document is attached to on this device. */

import type { DetachLocalFolder, LocalFoldersConfigMutation } from "../📁️local-folders-config-mutation/🟦️.ts";
import { attachLocalFolder } from "../📁️local-folders-config-mutation/🟦️.ts";
import type { LocalFolderBindings } from "../📎️attach-local-folder/🟦️.ts";

//#region 🔖️Mutation
/** 🔺️ Drops the binding of the payload's document; a document without one leaves the bindings as they were. */
export function diff(payload: DetachLocalFolder, base: LocalFolderBindings): LocalFolderBindings {
  if (!base.bindings.some((entry) => entry.documentId === payload.documentId)) return base;
  return { bindings: base.bindings.filter((entry) => entry.documentId !== payload.documentId) };
}

/** ↩️ Re-attaches the prior binding, or emits no step when the document had none. */
export function inverse(payload: DetachLocalFolder, base: LocalFolderBindings): LocalFoldersConfigMutation[] {
  const prior = base.bindings.find((entry) => entry.documentId === payload.documentId);
  return prior ? [attachLocalFolder(prior)] : [];
}
//#endregion 🔖️Mutation
