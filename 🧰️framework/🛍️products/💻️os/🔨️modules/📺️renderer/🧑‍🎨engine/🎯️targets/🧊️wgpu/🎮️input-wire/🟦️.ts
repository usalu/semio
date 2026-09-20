import type { BrowserFrameDomEvent } from "../🚚️browser-frame-transport/🟦️.ts";
import { WGPU_ACCESSIBILITY_MIRROR_ID } from "../♿️accessibility-mirror/🟦️.ts";

export type BrowserKeyboardEvent = Extract<BrowserFrameDomEvent, { readonly type: "keydown" | "keyup" }>;

export type BrowserClipboardPasteCandidate = { readonly kind: "image"; readonly file: File } | { readonly kind: "text"; readonly item: DataTransferItem };

/** 📋️ Selects one bounded DOM clipboard item with React's image-first priority. */
export function browserClipboardPasteCandidate(items: ArrayLike<DataTransferItem> | null | undefined): BrowserClipboardPasteCandidate | undefined {
  if (!items) return undefined;
  const count = Math.min(items.length, 16);
  for (let index = 0; index < count; index++) {
    const item = items[index];
    if (item?.kind !== "file" || !item.type.startsWith("image/")) continue;
    const file = item.getAsFile();
    if (file) return { kind: "image", file };
  }
  for (let index = 0; index < count; index++) {
    const item = items[index];
    if (item?.kind === "string" && item.type === "text/plain") return { kind: "text", item };
  }
  return undefined;
}

/** 🖥️ Owns fullscreen transitions for the renderer's visual and accessible surfaces. */
export function wireBrowserFullscreen(root: HTMLElement, canvas: HTMLCanvasElement): { set: (fullscreen: boolean) => Promise<void>; dispose: () => void } {
  const document = root.ownerDocument;
  let preferred: HTMLElement = canvas;
  let owned = false;
  let disposed = false;
  const remember = () => {
    if (document.activeElement instanceof HTMLElement && root.contains(document.activeElement)) preferred = document.activeElement;
  };
  const changed = () => {
    const active = document.fullscreenElement === root;
    if ((active || owned) && !root.contains(document.activeElement)) {
      (root.contains(preferred) ? preferred : canvas).focus({ preventScroll: true });
    }
    owned = active;
  };
  document.addEventListener("fullscreenchange", changed);
  return {
    set: async fullscreen => {
      if (disposed) return;
      if (fullscreen && document.fullscreenElement !== root) {
        remember();
        await root.requestFullscreen();
      } else if (!fullscreen && document.fullscreenElement === root) {
        remember();
        await document.exitFullscreen();
      }
    },
    dispose: () => {
      disposed = true;
      document.removeEventListener("fullscreenchange", changed);
    },
  };
}

/** ⌨️ Owns browser keyboard admission for the canvas and its accessible controls. */
export function wireBrowserKeyboard(root: HTMLElement, canvas: HTMLCanvasElement, admit: (event: BrowserKeyboardEvent) => void): () => void {
  const document = root.ownerDocument;
  const window = document.defaultView;
  let heldModifiers: string[] = [];
  let disposed = false;
  const reset = () => {
    const held = heldModifiers;
    heldModifiers = [];
    for (const key of held) admit({ type: "keyup", key, shift: false, ctrl: false, alt: false, meta: false });
  };
  const focusout = () => queueMicrotask(() => {
    if (!disposed && !root.contains(document.activeElement)) reset();
  });
  const visibility = () => {
    if (document.visibilityState === "hidden") reset();
  };
  const key = (event: KeyboardEvent, type: "keydown" | "keyup") => {
    if (event.defaultPrevented || event.isComposing || !(event.target instanceof HTMLElement)) return;
    if (event.target !== canvas) {
      if (!event.target.closest(`#${WGPU_ACCESSIBILITY_MIRROR_ID}`)) return;
      const modifier = ["Control", "Meta", "Alt", "Shift"].includes(event.key);
      const combobox = event.target.closest('[role="combobox"]');
      const editableCombobox = combobox instanceof HTMLInputElement || combobox instanceof HTMLTextAreaElement || combobox?.matches('[contenteditable]:not([contenteditable="false"])') === true;
      const retainedComboboxKey = ["Escape", "ArrowDown", "ArrowUp", "Home", "End", "PageDown", "PageUp", "Enter"].includes(event.key);
      if (!modifier && (event.key === "Tab" || (editableCombobox && !retainedComboboxKey) || (!combobox && event.target.closest('input,textarea,select,[contenteditable]:not([contenteditable="false"])')))) return;
      if (!combobox && event.target.closest("button") && ["Enter", " "].includes(event.key)) return;
      if (!modifier) event.preventDefault();
    }
    heldModifiers = [["Shift", event.shiftKey], ["Control", event.ctrlKey], ["Alt", event.altKey], ["Meta", event.metaKey]].filter(([, held]) => held).map(([key]) => key as string);
    admit({ type, key: event.key, shift: event.shiftKey, ctrl: event.ctrlKey, alt: event.altKey, meta: event.metaKey });
  };
  const down = (event: KeyboardEvent) => key(event, "keydown");
  const up = (event: KeyboardEvent) => key(event, "keyup");
  root.addEventListener("keydown", down);
  root.addEventListener("keyup", up);
  root.addEventListener("focusout", focusout);
  window?.addEventListener("blur", reset);
  document.addEventListener("visibilitychange", visibility);
  return () => {
    disposed = true;
    reset();
    root.removeEventListener("keydown", down);
    root.removeEventListener("keyup", up);
    root.removeEventListener("focusout", focusout);
    window?.removeEventListener("blur", reset);
    document.removeEventListener("visibilitychange", visibility);
  };
}
