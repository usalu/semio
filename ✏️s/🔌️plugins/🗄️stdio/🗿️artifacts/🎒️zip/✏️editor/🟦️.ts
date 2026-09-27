/** 🎒️ Canonical archive text edits shared by both ZIP dialects. */
import commandSchema from "./🎮️commands/✏️set-node/🔣️schema.json";
import { blake3Hex } from "../../../../../../🧰️framework/🔨️modules/🔏️hash/🟦️.ts";
import type { ZipSnapshot } from "../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import type { ZipMutation } from "../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts";

export interface ZipSetNode {
  nodeId: string;
  value: string;
  revision: string;
}

export const ZIP_COMMENT_NODE_ID = "comment" as const;
export const ZIP_ENTRY_NODE_PREFIX = "entry:" as const;
export const ZIP_EDIT_MAIN_WINDOW_KIND_ID = "framework.window.tree" as const;
export const ZIP_EDIT_MAIN_BODY_KEY = ZIP_EDIT_MAIN_WINDOW_KIND_ID;
export const ZIP_MAXIMUM_TEXT_BYTES = commandSchema.properties.value["x-semio-maxUtf8Bytes"];

/** 🔐️ The persisted text token carried by a local archive draft. */
export function archiveTextRevision(value: string): string {
  return blake3Hex(new TextEncoder().encode(value));
}

/** 🏷️ Reordering preserves the target; a rename invalidates its old address. */
export function archiveEntryNodeId(name: string): string {
  return `${ZIP_ENTRY_NODE_PREFIX}${archiveTextRevision(name)}`;
}

/** 📏️ Rejects oversized UTF-8 text before allocating an unbounded encoding buffer. */
function validateArchiveText(value: string): void {
  if (value.length > ZIP_MAXIMUM_TEXT_BYTES || new TextEncoder().encode(value).byteLength > ZIP_MAXIMUM_TEXT_BYTES) throw new Error("stdio.zip.text-too-large");
}

/** 🧵️ Resolves one bounded member name per turn against an immutable saved snapshot. */
export class ArchiveTextCursor {
  #cursor = 0;
  #matched: number | undefined;
  #collision = false;
  #complete = false;

  get scannedEntries(): number { return this.#cursor; }

  advance(snapshot: ZipSnapshot, event: ZipSetNode): ZipMutation[] | undefined {
    if (this.#complete) throw new Error("stdio.zip.work-complete");
    validateArchiveText(event.value);
    if (event.nodeId === ZIP_COMMENT_NODE_ID) {
      validateArchiveText(snapshot.comment);
      if (archiveTextRevision(snapshot.comment) !== event.revision) throw new Error("stdio.zip.draft-conflict");
      this.#complete = true;
      return snapshot.comment === event.value ? [] : [{ mutation: "setArchiveComment", comment: event.value }];
    }
    if (!event.nodeId.startsWith(ZIP_ENTRY_NODE_PREFIX)) throw new Error("stdio.zip.target-missing");
    const current = snapshot.entries[this.#cursor];
    if (current !== undefined) {
      validateArchiveText(current.name);
      if (archiveEntryNodeId(current.name) === event.nodeId) {
        if (this.#matched !== undefined) throw new Error("stdio.zip.target-ambiguous");
        this.#matched = this.#cursor;
      }
      this.#collision ||= current.name === event.value;
      this.#cursor++;
      return undefined;
    }
    if (this.#matched === undefined) throw new Error("stdio.zip.target-missing");
    const entry = snapshot.entries[this.#matched]!;
    if (archiveTextRevision(entry.name) !== event.revision) throw new Error("stdio.zip.draft-conflict");
    if (event.value.length === 0) throw new Error("stdio.zip.name-required");
    this.#complete = true;
    if (entry.name === event.value) return [];
    if (this.#collision) throw new Error("stdio.zip.name-exists");
    return [{ mutation: "renameEntry", name: entry.name, newName: event.value }];
  }
}

/** ✏️ Resolves a complete synchronous edit with the same retained cursor semantics. */
export function editArchiveText(snapshot: ZipSnapshot, event: ZipSetNode): ZipMutation[] {
  const cursor = new ArchiveTextCursor();
  for (;;) {
    const result = cursor.advance(snapshot, event);
    if (result !== undefined) return result;
  }
}
