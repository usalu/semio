/** 🛬️ OS file drop onto the browser shell: files dragged from the desktop take the same picked-file opening path as Open File, so routing, content check, refusal notices and cancellation are the ones the picker already has. */

/** 🎯️ The part of an event target the drop listeners need. */
export type NaturalFileDropTargetV1 = Readonly<{ addEventListener: (type: string, listener: (event: Event) => void) => void; removeEventListener: (type: string, listener: (event: Event) => void) => void }>;

type DropTransferV1 = { readonly types?: ArrayLike<string>; readonly files?: ArrayLike<File>; dropEffect?: string };
type DropEventV1 = Event & { readonly dataTransfer?: DropTransferV1 | null };

/** 🔎️ Whether a drag carries OS files; drags of text, links and `application/x-semio-` payloads stay with their own handlers. */
export function carriesOsFilesV1(transfer: DropTransferV1 | null | undefined): transfer is DropTransferV1 {
  return transfer !== null && transfer !== undefined && Array.from(transfer.types ?? []).includes("Files");
}

/** 🛬️ Accepts dropped OS files and opens them one after the other (the single-flight slot of Open File answers a busy shell); returns the uninstall function. */
export function installNaturalFileDropV1(target: NaturalFileDropTargetV1, open: (file: File) => Promise<void>): () => void {
  const over = (event: Event): void => {
    const transfer = (event as DropEventV1).dataTransfer;
    if (event.defaultPrevented || !carriesOsFilesV1(transfer)) return;
    event.preventDefault();
    transfer.dropEffect = "copy";
  };
  const drop = (event: Event): void => {
    const transfer = (event as DropEventV1).dataTransfer;
    if (event.defaultPrevented || !carriesOsFilesV1(transfer)) return;
    event.preventDefault();
    const files = Array.from(transfer.files ?? []);
    void (async () => {
      for (const file of files) await open(file);
    })().catch((error: unknown) => console.warn("[os-shell] dropped file opening failed", error));
  };
  target.addEventListener("dragover", over);
  target.addEventListener("drop", drop);
  return () => {
    target.removeEventListener("dragover", over);
    target.removeEventListener("drop", drop);
  };
}
