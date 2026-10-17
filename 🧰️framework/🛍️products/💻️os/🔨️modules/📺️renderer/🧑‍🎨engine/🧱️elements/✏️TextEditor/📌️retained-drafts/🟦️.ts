/** 📌️ Text editor retained drafts: the bounded retained draft store keyed by surface, its bucket, key, acquire, publish and count helpers. */
import { type LocalDocumentOwnerV1 } from "../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import type { RetainedTextEditorDraftBucketV1, RetainedTextEditorDraftV1, TextEditorExplicitDraftState } from "../🏷️editor-types/🟦️.ts";

//#region 🔖️TextEditorHost
export const TEXT_EDITOR_RETAINED_DRAFT_LIMIT_V1 = 64;
const retainedTextEditorDraftsV1 = new WeakMap<LocalDocumentOwnerV1, RetainedTextEditorDraftBucketV1>();

function retainedTextEditorDraftBucketV1(owner: LocalDocumentOwnerV1): RetainedTextEditorDraftBucketV1 {
  const current = retainedTextEditorDraftsV1.get(owner);
  if (current) return current;
  const bucket: RetainedTextEditorDraftBucketV1 = { entries: new Map(), unsubscribe: () => {} };
  if (!owner.active) return bucket;
  retainedTextEditorDraftsV1.set(owner, bucket);
  bucket.unsubscribe = owner.subscribeRetirement(() => {
    bucket.entries.clear();
    bucket.unsubscribe();
    retainedTextEditorDraftsV1.delete(owner);
  });
  return bucket;
}

export function retainedTextEditorDraftKeyV1(windowId: string, draftScope: string): string { return `${windowId.length}:${windowId}${draftScope.length}:${draftScope}`; }

export function acquireRetainedTextEditorDraftV1(owner: LocalDocumentOwnerV1, key: string, initial: TextEditorExplicitDraftState): RetainedTextEditorDraftV1 {
  const bucket = retainedTextEditorDraftBucketV1(owner);
  return bucket.entries.get(key) ?? { owner, key, bucket, state: initial, attached: false, listeners: new Set() };
}

function meaningfulRetainedTextEditorDraftV1(state: TextEditorExplicitDraftState): boolean { return state.dirty || state.pending !== null || state.conflicted || state.failed; }

export function publishRetainedTextEditorDraftV1(entry: RetainedTextEditorDraftV1, state: TextEditorExplicitDraftState): void {
  if (!entry.owner.active) return;
  entry.state = state;
  if (meaningfulRetainedTextEditorDraftV1(state)) {
    if (!entry.attached) {
      while (entry.bucket.entries.size >= TEXT_EDITOR_RETAINED_DRAFT_LIMIT_V1) {
        const clean = [...entry.bucket.entries].find(([, candidate]) => !meaningfulRetainedTextEditorDraftV1(candidate.state));
        if (!clean) break;
        clean[1].attached = false;
        entry.bucket.entries.delete(clean[0]);
      }
      if (entry.bucket.entries.size < TEXT_EDITOR_RETAINED_DRAFT_LIMIT_V1) {
        entry.bucket.entries.set(entry.key, entry);
        entry.attached = true;
      }
    }
  } else if (entry.attached) {
    entry.bucket.entries.delete(entry.key);
    entry.attached = false;
  }
  for (const listener of entry.listeners) listener(state);
}

/** 🧮️ Reports the bounded dirty/pending Source drafts held by one exact live document owner. */
export function retainedTextEditorDraftCountV1(owner: LocalDocumentOwnerV1): number { return retainedTextEditorDraftsV1.get(owner)?.entries.size ?? 0; }
//#endregion 🔖️TextEditorHost
