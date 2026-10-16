/** 🧷️ The context every retained view reads: the interpreter context type, the plugin surface actions context and the label resolver.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { createContext, useContext } from "react";
import { type LocalDocumentOwnerV1 } from "../🧭️local-document-owner/🟦️.ts";
import { type InputOutcomeV1, type ShellInputActionV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import { type UiLabel, type UiTranslationKey } from "@semio-tech/ui-react";
import { type ActionDispatchLifecycleV1, type ContextMenuItemSpec, type PluginContextMenuRequest } from "@semio-tech/framework";
import { UiDocumentStore } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type UiIntent } from "@semio-tech/framework";
import { shellLabel } from "../../🛠️ShellHelpers/🟦️.tsx";

//#region 🔖️UiInterpreter
export function interpLabel(key: UiTranslationKey): UiLabel {
  return shellLabel(key);
}

//#region UiInterpreterContext
export type UiInterpreterContext = {
  readonly store: UiDocumentStore;
  readonly localDocumentOwner?: LocalDocumentOwnerV1 | null;
  /** 🌉️ Legacy ActionDescriptor channel — the seam this Interpreter still speaks to the 14 unowned
   * scene-host elements through (see `🌉️SurfaceBridge`). Semantic components (button/input/select/…)
   * never use this; they go through `emitIntent`/`UiIntent` instead. */
  readonly onAction: (action: ShellInputActionV1, lifecycle?: ActionDispatchLifecycleV1) => void | Promise<unknown>;
  /** 🎬️ Semantic dispatch — fires a `UiIntent` built from the node's own `ActionBinding`s. */
  /** 🔁️ Answers with the promise the shell's own dispatch settles on, so a CONTINUOUS control (a
   * dragged slider, a held spinner) can tell whether its last value has landed. `void` is still
   * accepted for a sink that has nothing to settle. */
  readonly onIntent: (intent: UiIntent, windowId?: string) => void | Promise<void | InputOutcomeV1>;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  /** 🪪️ The DOM id of the row this subtree renders inside — a tree row or a table row. A node key is unique only among its
   * siblings, so a row's inline controls and cells are addressed under their row ({@link uiNodeDomId}). */
  readonly domScope?: string;
};
//#endregion UiInterpreterContext

export const PluginSurfaceActionsContext = createContext<UiInterpreterContext["requestContextMenu"]>(undefined);

export function usePluginSurfaceActions(): UiInterpreterContext["requestContextMenu"] {
  return useContext(PluginSurfaceActionsContext);
}
//#endregion 🔖️UiInterpreter
