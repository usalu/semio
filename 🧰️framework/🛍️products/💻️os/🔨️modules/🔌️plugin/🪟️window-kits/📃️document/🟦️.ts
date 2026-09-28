// #region 📄️DocumentWindowKit
/// <reference types="vitest/importMeta" />
/** 📄️ Shared document view and addressed draft contracts for the semantic document window. */
import type { BuiltNode, Component, LayoutSpec, StyleSpec, AccessibilitySpec, UiValue } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `DocumentWindowKit::KIND_ID`. */
export const DOCUMENT_WINDOW_KIND_ID = "framework.window.document";

/** 📄️ One page of plain text — twin of Rust `DocumentPage`. */
export type DocumentPage = {
  readonly text: string;
};

/** 📄️ A paginated text document — twin of Rust `DocumentView`. */
export type DocumentView = {
  readonly pages: readonly DocumentPage[];
};

/** ✏️ One explicitly addressed document text target. */
export type EditableDocumentPage = {
  readonly pageIndex: number;
  readonly itemIndex: number;
  readonly text: string;
  readonly arguments?: Readonly<Record<string, UiValue>>;
};

/** ✏️ Settings consumed by the shared explicit text-draft host. */
export type EditableDocumentDraft = {
  readonly page: number;
  readonly item: number;
  readonly revision: string;
  readonly text: string;
  readonly arguments: Readonly<Record<string, UiValue>>;
  readonly labels: { readonly apply: string; readonly discard: string; readonly cancel: string };
};

/** 🔐️ TS twin of Rust `DocumentWindowKit::text_revision`. */
export function documentTextRevision(text: string): string {
  let hash = 0xcbf29ce484222325n;
  for (const byte of new TextEncoder().encode(text)) hash = BigInt.asUintN(64, (hash ^ BigInt(byte)) * 0x100000001b3n);
  return hash.toString(16).padStart(16, "0");
}

/** ✏️ Produces one localized, prefilled document draft contract. */
export function editableDocumentDraft(page: EditableDocumentPage, locale: "en" | "de"): EditableDocumentDraft {
  const revision = documentTextRevision(page.text);
  return {
    page: page.pageIndex,
    item: page.itemIndex,
    revision,
    text: page.text,
    arguments: page.arguments === undefined ? { page: page.pageIndex, item: page.itemIndex, revision } : structuredClone(page.arguments),
    labels: locale === "de" ? { apply: "Anwenden", discard: "Verwerfen", cancel: "Abbrechen" } : { apply: "Apply", discard: "Discard", cancel: "Cancel" },
  };
}

const DEFAULT_LEAF_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const DEFAULT_STACK_LAYOUT: LayoutSpec = { kind: "stack", axis: "vertical", gap: "md", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false };
const DEFAULT_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const DEFAULT_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

/** 🧱️ Stamps a {@link BuiltNode} from `component`/`layout`/`children`, filling every other field with the shared defaults. */
function builtNode(key: string, component: Component, layout: LayoutSpec, children: readonly BuiltNode[] = []): BuiltNode {
  return { key, component, layout, style: DEFAULT_STYLE, activity: "idle", disabled: false, accessibility: DEFAULT_ACCESSIBILITY, bindings: [], menu: null, children: [...children] };
}

/** 📄️ Twin of Rust `DocumentWindowKit::render` — one text child per page inside an unlabeled vertical stack. */
export function renderDocument(view: DocumentView): BuiltNode {
  const children = view.pages.map((page, index) => builtNode(`page-${index}`, { type: "text", value: page.text, emphasize: null, dataAttributes: null }, DEFAULT_LEAF_LAYOUT));
  return builtNode(DOCUMENT_WINDOW_KIND_ID, { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, DEFAULT_STACK_LAYOUT, children);
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️renderdocument/🟦️.ts");
  await registerTests1(import.meta.vitest, { renderDocument, editableDocumentDraft }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
// #endregion 📄️DocumentWindowKit
