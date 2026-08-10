/**
 * 🚪️ Note IO facet — format table + WASM/host bridge facades.
 * Byte codecs live in Rust (`ArtifactIo` + framework `DocumentCodec`s); the TS side
 * exposes the declared catalog for pickers/menus and forwards to host media actions.
 */

export const NOTE_IO_FORMATS = ["dwg", "dxf", "json", "pdf", "png", "svg"] as const;
export type NoteIoFormat = (typeof NOTE_IO_FORMATS)[number];

/** 🗂️ File-picker accept filter derived from the declared facet. */
export function noteIoAcceptFilter(formats: readonly NoteIoFormat[] = NOTE_IO_FORMATS): string {
  return formats.map((format) => `.${format}`).join(",");
}

/** 📤️ Export menu entries derived from the declared facet. */
export function noteIoExportMenu(formats: readonly NoteIoFormat[] = NOTE_IO_FORMATS): ReadonlyArray<{ format: NoteIoFormat; label: string }> {
  return formats.map((format) => ({ format, label: format.toUpperCase() }));
}

/**
 * 🌉 Host bridge: ask the shell to run `exportMedia` / `importMedia` for this artifact kind.
 * The WASM guest registers handlers via `io::register()`; the host invokes them by kind+format.
 */
export type NoteIoHostBridge = {
  exportMedia: (format: NoteIoFormat) => void | Promise<void>;
  importMedia: (format: NoteIoFormat) => void | Promise<void>;
};

let bridge: NoteIoHostBridge | null = null;

/** 🔌️ Install the host bridge used by leaf TS facades. */
export function installNoteIoHostBridge(next: NoteIoHostBridge): void {
  bridge = next;
}

export async function exportNoteMedia(format: NoteIoFormat): Promise<void> {
  if (!bridge) throw new Error("[DEBUG] note io host bridge missing — installNoteIoHostBridge first");
  await bridge.exportMedia(format);
}

export async function importNoteMedia(format: NoteIoFormat): Promise<void> {
  if (!bridge) throw new Error("[DEBUG] note io host bridge missing — installNoteIoHostBridge first");
  await bridge.importMedia(format);
}
