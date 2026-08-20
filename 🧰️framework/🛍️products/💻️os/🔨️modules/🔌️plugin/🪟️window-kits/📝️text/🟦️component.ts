//#region 🔖️TextWindowKit
/** @emoji 📝 `@semio-tech/plugin-window-kits` — TypeScript twin of the Rust `TextWindowKit`
 * (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` `#region 🔖️WindowKits`, ticket
 * 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §2.6). One language-tagged text buffer; `readOnly`
 * stamps `TextEditorScene.settingsJson` so the host renderer can disable input without a separate
 * view-model shape for the editable variant. */
import type { LocalizedLabel, UiComponentSceneNode } from "@semio-tech/framework";

export const KIND_ID = "framework.window.text";

export const LABEL: LocalizedLabel = { en: "Text", de: "Text" };

export type TextView = {
  readonly text: string;
  readonly language?: string;
  readonly readOnly: boolean;
};

/** 🖼️ Renders a {@link TextView} into a `text-editor` component-scene node. */
export function render(view: TextView): UiComponentSceneNode {
  return {
    type: "componentScene",
    surfaceId: KIND_ID,
    controllerId: KIND_ID,
    componentKind: "text-editor",
    textEditor: {
      buffer: view.text,
      language: view.language,
      settingsJson: view.readOnly ? JSON.stringify({ readOnly: true }) : undefined,
    },
  };
}
//#endregion 🔖️TextWindowKit
