/** 📄️ A paged text scene can edit one complete value through its explicit artifact owner. */
import { splitUiTextChunks } from "@semio-tech/ui-contract/component";
import type { ActionDescriptor } from "@semio-tech/framework";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import schema from "../🧬️schema/📄️value-source/🔣️.json";

export type TextEditorValueSourceV1 = Readonly<{
  readOnly: boolean;
  commit: "change";
  editControllerId: string;
  editAction: string;
  editArgument: string;
  editEncoding?: "json-chunks";
  editMaximumBytes?: number;
  editArguments: Readonly<Record<string, unknown>>;
  editIdentityArguments?: Readonly<Record<string, unknown>>;
  label: string;
}> | Readonly<{ readOnly: true; label: string }>;

/** 🗂️ Admits only the schema-owned change-commit settings; the source itself stays in its buffer lane. */
export function parseTextEditorValueSourceV1(json: string | undefined): TextEditorValueSourceV1 | null {
  if (json === undefined) return null;
  try {
    const value: unknown = JSON.parse(json);
    return validateJsonSchemaSubset(schema, value).length === 0 ? value as TextEditorValueSourceV1 : null;
  } catch { return null; }
}

/** 🪪️ Names the semantic edit route independently of labels, appearance and caller-supplied baseline data. */
export function textEditorValueSourceEditIdentityV1(controllerId: string, surfaceId: string, settings: TextEditorValueSourceV1 | null): string {
  return JSON.stringify([controllerId, surfaceId, settings && "editControllerId" in settings ? [settings.editControllerId, settings.editAction, settings.editArgument, settings.editEncoding, Object.entries(settings.editIdentityArguments ?? settings.editArguments).sort(([a], [b]) => a.localeCompare(b))] : null]);
}

export type TextEditorValueSourceResultV1 = Readonly<{ kind: "dispatch"; action: ActionDescriptor }> | Readonly<{ kind: "local" }> | Readonly<{ kind: "refused"; reason: "source-too-large" }>;
export type TextEditorSourceDraftV1 = Readonly<{ text: string; refused: boolean; admitted: string | null }>;

/** ✍️ Keeps the latest local correction independent of an older in-flight parent edit. */
export function changeTextEditorSourceDraftV1(state: TextEditorSourceDraftV1 | null, text: string, published: string, refused: boolean, awaitingPublication = false): TextEditorSourceDraftV1 | null {
  return text === published && !awaitingPublication ? null : { text, refused: text === published ? false : refused || state?.refused === true, admitted: null };
}

/** ✅️ An older admission cannot clear a newer local size refusal or replace its text. */
export function admitTextEditorSourceDraftV1(state: TextEditorSourceDraftV1 | null, text: string): TextEditorSourceDraftV1 | null {
  return state?.text === text ? { ...state, refused: false, admitted: text } : state;
}

/** 📨️ Preflights complete encoded values before dispatch; programming faults remain visible to callers. */
export function textEditorValueSourceActionV1(settings: TextEditorValueSourceV1, action: ActionDescriptor): TextEditorValueSourceResultV1 {
  if (settings.readOnly) return { kind: "local" };
  if (action.action === "undo" || action.action === "redo") return { kind: "dispatch", action: { ...action, controllerId: settings.editControllerId } };
  const args = action.args;
  if (action.action !== "textEdit" || typeof args !== "object" || args === null || Array.isArray(args) || !("text" in args) || typeof args.text !== "string") return { kind: "local" };
  const encoded = JSON.stringify(args.text), maximum = settings.editMaximumBytes ?? 16_384;
  if (settings.editEncoding === "json-chunks" && new TextEncoder().encode(encoded).byteLength > maximum) return { kind: "refused", reason: "source-too-large" };
  return { kind: "dispatch", action: { controllerId: settings.editControllerId, action: settings.editAction, args: { ...settings.editArguments, [settings.editArgument]: settings.editEncoding === "json-chunks" ? splitUiTextChunks(encoded, maximum) : args.text } } };
}
