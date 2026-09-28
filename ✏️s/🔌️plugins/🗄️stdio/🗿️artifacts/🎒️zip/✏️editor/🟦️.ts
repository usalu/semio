/** 🎒️ Canonical archive text edits shared by both ZIP dialects. */
import commandSchema from "./🎮️commands/✏️set-node/🔣️schema.json";
import checkpointSchema from "./🧵️retained/🔣️schema.json";
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
const ZIP_CP437_HIGH = "ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■ ";

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

/** 🔤️ Mirrors the native EOCD edit policy: preserve unambiguous CP437 and promote otherwise. */
export function archiveCommentUtf8AfterEdit(currentUtf8: boolean, replacement: string): boolean {
  if (currentUtf8) return true;
  const bytes: number[] = [];
  for (const character of replacement) {
    const code = character.codePointAt(0)!;
    if (code < 0x80) bytes.push(code);
    else {
      const index = ZIP_CP437_HIGH.indexOf(character);
      if (index < 0) return true;
      bytes.push(index + 0x80);
    }
  }
  try {
    new TextDecoder("utf-8", { fatal: true }).decode(Uint8Array.from(bytes));
    return true;
  } catch {
    return false;
  }
}

/** 🧵️ Resolves one bounded member name per turn against an immutable saved snapshot. */
export class ArchiveTextCursor {
  #cursor = 0;
  #matched: number | undefined;
  #collision = false;
  #complete = false;
  #binding: { digest: string; entries: number } | undefined;
  #event: ZipSetNode | undefined;

  get scannedEntries(): number { return this.#cursor; }

  checkpoint(): Uint8Array {
    const bytes = new Uint8Array(checkpointSchema.minItems);
    const view = new DataView(bytes.buffer);
    bytes.set([90, 65, 84, 49, 1, Number(this.#matched !== undefined) | Number(this.#collision) << 1 | Number(this.#complete) << 2 | Number(this.#binding !== undefined) << 3]);
    view.setBigUint64(8, BigInt(this.#cursor), true);
    view.setBigUint64(16, BigInt(this.#matched ?? 0), true);
    if (this.#binding) {
      view.setBigUint64(24, BigInt(this.#binding.entries), true);
      bytes.set(new TextEncoder().encode(this.#binding.digest), 32);
    }
    return bytes;
  }

  restore(bytes: Uint8Array): void {
    const invalid = () => new Error("stdio.zip.checkpoint-invalid");
    if (bytes.length !== checkpointSchema.minItems || bytes[0] !== 90 || bytes[1] !== 65 || bytes[2] !== 84 || bytes[3] !== 49 || bytes[4] !== 1 || bytes[5]! > 15 || bytes[6] !== 0 || bytes[7] !== 0) throw invalid();
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const cursor = Number(view.getBigUint64(8, true));
    const matched = Number(view.getBigUint64(16, true));
    const entries = Number(view.getBigUint64(24, true));
    const flags = bytes[5]!;
    const bound = (flags & 8) !== 0;
    const digest = new TextDecoder().decode(bytes.subarray(32));
    if (![cursor, matched, entries].every(Number.isSafeInteger) || cursor > entries || ((flags & 1) !== 0 && matched >= cursor) || ((flags & 1) === 0 && matched !== 0) || (!bound && (flags !== 0 || cursor !== 0 || entries !== 0 || bytes.subarray(32).some(byte => byte !== 0))) || (bound && !/^[0-9a-f]{64}$/.test(digest))) throw invalid();
    this.#cursor = cursor;
    this.#matched = (flags & 1) !== 0 ? matched : undefined;
    this.#collision = (flags & 2) !== 0;
    this.#complete = (flags & 4) !== 0;
    this.#binding = bound ? { digest, entries } : undefined;
    this.#event = undefined;
  }

  advance(snapshot: ZipSnapshot, event: ZipSetNode): ZipMutation[] | undefined {
    if (this.#complete) throw new Error("stdio.zip.work-complete");
    if (this.#event) {
      if (this.#event.nodeId !== event.nodeId || this.#event.value !== event.value || this.#event.revision !== event.revision || this.#binding!.entries !== snapshot.entries.length) throw new Error("stdio.zip.checkpoint-context");
    } else {
      validateArchiveText(event.value);
      if (event.nodeId.length > 70 || event.revision.length > 64) throw new Error("stdio.zip.argument-invalid");
      const binding = { digest: archiveTextRevision(`${event.nodeId}\0${event.revision}\0${event.value}`), entries: snapshot.entries.length };
      if (this.#binding && (this.#binding.digest !== binding.digest || this.#binding.entries !== binding.entries)) throw new Error("stdio.zip.checkpoint-context");
      this.#binding = binding;
      this.#event = { ...event };
    }
    if (event.nodeId === ZIP_COMMENT_NODE_ID) {
      validateArchiveText(snapshot.comment);
      if (archiveTextRevision(snapshot.comment) !== event.revision) throw new Error("stdio.zip.draft-conflict");
      this.#complete = true;
      return snapshot.comment === event.value
        ? []
        : [{ mutation: "setArchiveComment", comment: event.value, commentUtf8: archiveCommentUtf8AfterEdit(snapshot.commentUtf8, event.value) }];
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
