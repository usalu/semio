/** 🏷️ Text editor types: the grammar token, diagnostic, session, span, completion, rename and pick shapes, the explicit draft settings, pending and state types and the retained draft records. */
import { type ActionDescriptor, type ActionOperationControlV1, type ContextMenuItemSpec, type DraftChangeV1, type PluginContextMenuRequest, type TextEditorScene } from "@semio-tech/framework";
import { type LocalDocumentOwnerV1 } from "../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { type EditorWasmSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { type InputDiagnosticV1, type InputOutcomeV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";

//#region 🔖️TextEditorHost
//#region Types
export type GrammarToken = { readonly class: string; readonly start: number; readonly end: number };

export type EditorDiagnostic = { readonly start: number; readonly end: number; readonly severity?: string; readonly message: string };

export type FrameworkEditorSession = EditorWasmSession;

export type SpanRange = { readonly start: number; readonly end: number };

export type CompletionItem = { readonly label: string; readonly detail?: string; readonly insertText?: string };

export type RenameInfo = { readonly name: string; readonly occurrences: readonly SpanRange[] };

export type RenameDraft = { readonly occurrences: readonly SpanRange[]; readonly text: string };

export type PickTarget = { readonly domain: string; readonly id: string; readonly generality?: number; readonly label: string };

export type TextEditorExplicitDraftSettings = Readonly<{
  readOnly: false;
  editAction: string;
  editArgument: string;
  editArguments: Readonly<Record<string, unknown>>;
  commit: "explicit";
  applyLabel: string;
  discardLabel: string;
  conflictLabel: string;
  applyingLabel: string;
  cancelLabel: string;
  failedLabel: string;
  syntaxErrorLabel: string;
  validationErrorLabel: string;
  pathLabel: string;
  lineLabel: string;
  columnLabel: string;
  publicationRevision: string;
}>;

export type TextEditorExplicitDraftPending = Readonly<{ token: object; value: string; changes: readonly DraftChangeV1[]; revision: string; outcome: InputOutcomeV1 | null; settled: boolean }>;
export type TextEditorExplicitDraftState = Readonly<{ surfaceId: string; base: string; revision: string; draft: string; changes: readonly DraftChangeV1[]; dirty: boolean; conflicted: boolean; failed: boolean; diagnostic: InputDiagnosticV1 | null; pending: TextEditorExplicitDraftPending | null; operation: ActionOperationControlV1 | null; cancelling: boolean; cancellationAccepted: boolean }>;
export type RetainedTextEditorDraftV1 = { readonly owner: LocalDocumentOwnerV1; readonly key: string; readonly bucket: RetainedTextEditorDraftBucketV1; state: TextEditorExplicitDraftState; attached: boolean; readonly listeners: Set<(state: TextEditorExplicitDraftState) => void> };
export type RetainedTextEditorDraftBucketV1 = { readonly entries: Map<string, RetainedTextEditorDraftV1>; unsubscribe: () => void };
export type WasmEditorSurfaceProps = {
  readonly scene: TextEditorScene;
  readonly controllerId: string;
  readonly surfaceId: string;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly explicitDraft?: TextEditorExplicitDraftSettings | null;
  readonly explicitDraftDirty?: boolean;
  readonly descriptionId?: string;
  readonly invalid?: boolean;
  readonly onDraftChange?: (text: string) => void;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
};

//#endregion Types
//#endregion 🔖️TextEditorHost
