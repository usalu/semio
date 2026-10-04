// #region 🖼️ImageWindowKit
/// <reference types="vitest/importMeta" />
/** 🖼️ `@semio-tech/plugin-window-kits` — semantic-contract twin of Rust `ImageWindowKit`
 * (`framework.window.image`), including its explicit mounted-unavailable state. */
import type { BuiltNode, Component, LayoutSpec, StyleSpec, AccessibilitySpec } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `ImageWindowKit::KIND_ID`. */
export const IMAGE_WINDOW_KIND_ID = "framework.window.image";
export const IMAGE_WINDOW_UNAVAILABLE_NODE_KEY = "framework.window.image.unavailable";

/** 🖼️ Raw pixel payload as a base64 blob — twin of Rust `ImageView`. */
export type ImageView = {
  readonly width: number;
  readonly height: number;
  readonly mime: string;
  readonly base64: string;
};

export type ImagePreview =
  | { readonly availability: "ready"; readonly view: ImageView }
  | { readonly availability: "unavailable" };

export type ImagePreviewLocale = "en" | "de";

const DEFAULT_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const DEFAULT_STACK_LAYOUT: LayoutSpec = { kind: "stack", axis: "vertical", gap: "md", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false };
const DEFAULT_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const DEFAULT_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

/** 🧱️ Stamps a leaf {@link BuiltNode} from `component`, filling every other field with the shared defaults. */
function leafNode(key: string, component: Component): BuiltNode {
  return { key, component, layout: DEFAULT_LAYOUT, style: DEFAULT_STYLE, activity: "idle", disabled: false, accessibility: DEFAULT_ACCESSIBILITY, bindings: [], menu: null, children: [] };
}

/** 🖼️ Twin of Rust `ImageWindowKit::render` — encodes `view` into a base64 data URI image node. */
export function renderImage(view: ImageView): BuiltNode {
  return leafNode(IMAGE_WINDOW_KIND_ID, { type: "image", src: `data:${view.mime};base64,${view.base64}`, alt: `${view.width}x${view.height}` });
}

/** 🚫️ Renders an explicit localized state while preserving the image window's mounted identity. */
export function renderImagePreview(preview: ImagePreview, locale: ImagePreviewLocale): BuiltNode {
  if (preview.availability === "ready") return renderImage(preview.view);
  const value = locale === "de" ? "Vorschau nicht verfügbar" : "Preview unavailable";
  return {
    key: IMAGE_WINDOW_KIND_ID,
    component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
    layout: DEFAULT_STACK_LAYOUT,
    style: DEFAULT_STYLE,
    activity: "idle",
    disabled: false,
    accessibility: DEFAULT_ACCESSIBILITY,
    bindings: [],
    menu: null,
    children: [leafNode(IMAGE_WINDOW_UNAVAILABLE_NODE_KEY, { type: "text", value, emphasize: null, dataAttributes: null })],
  };
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️renderimage/🟦️.ts");
  await registerTests1(import.meta.vitest, { renderImage, renderImagePreview }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
// #endregion 🖼️ImageWindowKit
