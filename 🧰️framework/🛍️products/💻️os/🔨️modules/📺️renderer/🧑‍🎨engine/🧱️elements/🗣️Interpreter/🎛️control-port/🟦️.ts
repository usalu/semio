/** 🎛️ The source authority ordinary controls use without owning a document projection. */
import type { ActionBinding, SurfaceId, UiIntent, UiNodeId, UiNodeRecord, Trigger as UiTrigger, UiValue } from "../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts";
import type { LocalDocumentOwnerV1 } from "../🧭️local-document-owner/🟦️.ts";
import type { InputOutcomeV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";

export type UiControlCompletion = void | Promise<void | InputOutcomeV1>;

export interface UiControlPort {
  readonly identity: object;
  surface(): SurfaceId;
  read(id: UiNodeId): UiNodeRecord | undefined;
  subscribe(listener: () => void): () => void;
  disabled(record: UiNodeRecord): boolean;
  value(record: UiNodeRecord): string | undefined;
  dispatch(record: UiNodeRecord, trigger: UiTrigger, input?: UiValue): UiControlCompletion;
}

export interface UiControlContext {
  readonly controls: UiControlPort;
  readonly localDocumentOwner?: LocalDocumentOwnerV1 | null;
  readonly domScope?: string;
  readonly field?: UiControlField;
}

export interface UiControlField {
  readonly label?: string;
  readonly describedBy?: string;
  readonly error?: string;
  readonly required?: boolean;
  readonly invalid?: boolean;
}

export interface UiControlDocument {
  getState(): { readonly surface: SurfaceId; readonly revision: UiIntent["revision"] };
  getNodeSnapshot(id: UiNodeId): UiNodeRecord | undefined;
  subscribeRevision(listener: () => void): () => void;
  buildIntent(record: UiNodeRecord, binding: ActionBinding, input?: UiValue): UiIntent;
}

/** 📃️ Adapts the real document's reads, publication and intent mint without copying its nodes. */
export function documentControlContext(context: { readonly store: UiControlDocument; readonly onIntent: (intent: UiIntent) => UiControlCompletion; readonly localDocumentOwner?: LocalDocumentOwnerV1 | null; readonly domScope?: string }): UiControlContext {
  const source = context.store;
  const disabled = (record: UiNodeRecord) => record.disabled === true || source.getNodeSnapshot(record.id) !== record;
  return {
    localDocumentOwner: context.localDocumentOwner,
    domScope: context.domScope,
    controls: {
      identity: source,
      surface: () => source.getState().surface,
      read: (id) => source.getNodeSnapshot(id),
      subscribe: (listener) => source.subscribeRevision(listener),
      disabled,
      value: () => undefined,
      dispatch: (record, trigger, input) => {
        if (disabled(record)) return;
        let binding: ActionBinding | undefined;
        for (const candidate of record.bindings ?? []) {
          if (candidate.trigger !== trigger) continue;
          if (binding) return;
          binding = candidate;
        }
        if (binding) return context.onIntent(source.buildIntent(record, binding, input));
      },
    },
  };
}

/** ⚡️ Consumes a control trigger through its original source authority. */
export function dispatchControlTrigger(context: UiControlContext, record: UiNodeRecord, trigger: UiTrigger, input?: UiValue): UiControlCompletion {
  return context.controls.dispatch(record, trigger, input);
}
