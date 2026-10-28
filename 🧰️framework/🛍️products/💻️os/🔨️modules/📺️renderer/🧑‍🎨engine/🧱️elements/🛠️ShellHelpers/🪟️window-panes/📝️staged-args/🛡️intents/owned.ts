/** 🪢️ Synchronous staged edits borrow original issued controls without projecting an owned surface. */
import type { UiNodeRecord, UiValue } from "../../../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts";
import type { UiControlPort } from "../../../../🗣️Interpreter/🎛️control-port/🟦️.ts";
import { STAGED_FORM_STATUS_EXTENSION, STAGED_FORM_EVENT_ACTION, STAGED_FORM_PAGE_ACTION, stagedFormToken, type StagedFormBinding, type StagedFormPageBinding, type StagedFormStatus } from "@semio-tech/framework/manifest/staging";
import { StagedFormSession, type StagedControlSource, type StagedIntentResult, type StagedResidentControl } from "./🟦️.ts";

export interface StagedOwnedSnapshot { readonly version: number; readonly record: unknown }
export interface StagedOwnedSubscription { readonly snapshot: StagedOwnedSnapshot | null }
export interface StagedOwnedView { readonly revision: number; readonly root: number | null; readonly hash: string | null }
export interface StagedOwnedSurface {
  readonly view: StagedOwnedView;
  subscribeView(notify: () => void): StagedOwnedSubscription;
  subscribeNode(id: number, notify: () => void): StagedOwnedSubscription;
  acknowledgeRead(subscription: StagedOwnedSubscription, snapshot: StagedOwnedSnapshot): void;
  unsubscribeNode(subscription: StagedOwnedSubscription): void;
}

const maximum = (1n << 64n) - 1n;
const readLimit = 64;
type Data = Record<string, unknown>;
function data(value: unknown): value is Data { return !!value && typeof value === "object" && [Object.prototype, null].includes(Object.getPrototypeOf(value)); }
function closed(value: unknown, keys: readonly string[]): value is Data {
  return data(value) && Reflect.ownKeys(value).length === keys.length && keys.every(key => { const field = Object.getOwnPropertyDescriptor(value, key); return !!field?.enumerable && "value" in field; });
}
function counter(value: unknown): value is string { return typeof value === "string" && /^(0|[1-9][0-9]{0,19})$/u.test(value) && BigInt(value) <= maximum; }
function path(value: unknown): boolean { return Array.isArray(value) && value.length >= 1 && value.length <= 64 && value.every(part => typeof part === "string" && part.length > 0); }
function label(value: unknown): boolean { return closed(value, ["en", "de"]) && typeof value.en === "string" && typeof value.de === "string"; }
function status(value: unknown): value is StagedFormStatus {
  if (!closed(value, ["token", "ready", "diagnostics", "diagnosticOffset", "diagnosticTotal"]) || !stagedFormToken(value.token) || typeof value.ready !== "boolean" || !Array.isArray(value.diagnostics) || value.diagnostics.length > 16 || !counter(value.diagnosticOffset) || !counter(value.diagnosticTotal)) return false;
  if (BigInt(value.diagnosticOffset) + BigInt(value.diagnostics.length) > BigInt(value.diagnosticTotal)) return false;
  return value.diagnostics.every(item => closed(item, ["path", "code", "label"]) && Array.isArray(item.path) && item.path.length <= 64 && item.path.every(part => typeof part === "string" && part.length > 0) && typeof item.code === "string" && label(item.label));
}
function edits(value: unknown): boolean {
  return Array.isArray(value) && value.length >= 1 && value.length <= 8 && value.every(edit => {
    if (!data(edit)) return false;
    if (edit.source === "text") return closed(edit, ["source", "path"]) && path(edit.path);
    if (edit.source === "choice") return closed(edit, ["source", "kind", "path", "options"]) && ["set", "append"].includes(edit.kind as string) && path(edit.path) && Array.isArray(edit.options) && edit.options.length >= 1 && edit.options.length <= 32 && edit.options.every(option => closed(option, ["key", "value"]) && typeof option.key === "string" && option.key.length > 0);
    if (edit.source !== "fixed" || !closed(edit, ["source", "edit"]) || !data(edit.edit)) return false;
    return edit.edit.kind === "remove" ? closed(edit.edit, ["kind", "path"]) && path(edit.edit.path) : closed(edit.edit, ["kind", "path", "value"]) && ["set", "append"].includes(edit.edit.kind as string) && path(edit.edit.path);
  });
}
function resident(record: UiNodeRecord, trigger: string): StagedResidentControl<number> | undefined {
  const candidates = record.bindings?.filter(binding => binding.trigger === trigger);
  if (candidates?.length !== 1) return;
  const binding = candidates[0]!;
  if (binding.capability != null || !data(binding.action) || !data(binding.args) || !stagedFormToken(binding.args.token)) return;
  const page = binding.action.name === STAGED_FORM_PAGE_ACTION;
  if (page ? !closed(binding.args, ["token", "page"]) : binding.action.name !== STAGED_FORM_EVENT_ACTION || !closed(binding.args, ["token", "layout", "edits"]) || !["preserve", "replace"].includes(binding.args.layout as string) || !edits(binding.args.edits)) return;
  const input = record.component.type === "input" && ["text", "longText"].includes(record.component.kind) && record.component.commit !== "blur" ? "text" : record.component.type === "select" ? "choice" : record.component.type === "button" ? "none" : undefined;
  if (input === undefined) return;
  return { node: record.id, key: record.key, trigger, disabled: record.disabled, editable: true, input, action: binding.action, binding: binding.args as unknown as StagedFormBinding | StagedFormPageBinding } as StagedResidentControl<number>;
}

type StatusRead = { readonly subscription: StagedOwnedSubscription; readonly snapshot: StagedOwnedSnapshot; readonly view: StagedOwnedView };

/** 🧶️ One stable surface/session lifetime owns reads, presentation notifications and a checked event sequence. */
export class OwnedStagedControls {
  readonly reads: Omit<StagedOwnedSurface, "view">;
  readonly #live = new Set<StagedOwnedSubscription>();
  readonly #listeners = new Set<() => void>();
  readonly #adapter: StagedControlSource<number, number>;
  #status: StatusRead | undefined;
  #borrow: { readonly record: UiNodeRecord; readonly trigger: string } | undefined;
  #sequence = 0n;
  #closed = false;
  #presentation: object = Object.freeze({});
  constructor(private readonly source: StagedOwnedSurface, private readonly surface: string, private readonly session: StagedFormSession<number, number>, private readonly staged: (result: Extract<StagedIntentResult, { kind: "staged" }>) => void) {
    if (!surface) throw new Error("staging.surface");
    this.reads = {
      subscribeView: (notify) => {
        if (this.#closed || this.#live.size >= readLimit) throw new Error("staging.read-capacity");
        const read = source.subscribeView(() => { this.#notify(); notify(); });
        this.#live.add(read);
        return read;
      },
      subscribeNode: (id, notify) => {
        if (this.#closed || this.#live.size >= readLimit) throw new Error("staging.read-capacity");
        const read = source.subscribeNode(id, () => { this.#notify(); notify(); });
        this.#live.add(read);
        return read;
      },
      acknowledgeRead: (read, snapshot) => { if (this.#live.has(read)) source.acknowledgeRead(read, snapshot); },
      unsubscribeNode: (read) => { if (this.#live.has(read)) { source.unsubscribeNode(read); this.#live.delete(read); this.#notify(); } },
    };
    this.#adapter = {
      view: () => ({ surface: this.surface, revision: this.#view()?.revision ?? -1 }),
      status: () => this.#currentStatus(),
      read: (node, trigger) => this.#borrow?.record.id === node && this.#borrow.trigger === trigger ? resident(this.#borrow.record, trigger) : undefined,
    };
  }
  #view(): StagedOwnedView | undefined { if (this.#closed) return; try { return this.source.view; } catch { return; } }
  readonly view = (): StagedOwnedView | undefined => this.#view();
  #record(read: StagedOwnedSubscription, snapshot: StagedOwnedSnapshot, view: StagedOwnedView): UiNodeRecord | undefined {
    if (!this.#live.has(read) || this.#view() !== view || snapshot.version !== view.revision) return;
    try { if (read.snapshot !== snapshot) return; const record = snapshot.record as UiNodeRecord | undefined; return record?.component.type !== "surface" ? record : undefined; } catch { return; }
  }
  #currentStatus(): StagedFormStatus | undefined {
    const read = this.#status;
    if (!read) return;
    const record = this.#record(read.subscription, read.snapshot, read.view);
    if (record?.component.type !== "extension" || record.component.extension !== STAGED_FORM_STATUS_EXTENSION || !status(record.component.props)) return;
    return record.component.props;
  }
  #notify(): void { this.#presentation = Object.freeze({}); for (const listener of this.#listeners) { try { listener(); } catch {} } }
  readonly presentation = (): object => this.#presentation;
  readonly subscribe = (listener: () => void): (() => void) => {
    if (this.#closed) return () => {};
    if (this.#listeners.size >= readLimit) throw new Error("staging.listener-capacity");
    this.#listeners.add(listener);
    return () => { this.#listeners.delete(listener); };
  };
  publishStatus(read: StagedOwnedSubscription, snapshot: StagedOwnedSnapshot, view: StagedOwnedView): boolean {
    this.#status = { subscription: read, snapshot, view };
    const accepted = !!this.#currentStatus() && this.session.publish(this.#adapter);
    if (!accepted) this.#status = undefined;
    this.#notify();
    return accepted;
  }
  control(read: StagedOwnedSubscription, snapshot: StagedOwnedSnapshot, view: StagedOwnedView): UiControlPort {
    const current = (record: UiNodeRecord) => this.#record(read, snapshot, view) === record;
    const borrow = <T>(record: UiNodeRecord, trigger: string, consume: () => T): T | undefined => {
      if (this.#borrow || !current(record)) return;
      this.#borrow = { record, trigger };
      try { return consume(); } finally { this.#borrow = undefined; }
    };
    const trigger = (record: UiNodeRecord): "activate" | "change" => record.component.type === "button" ? "activate" : "change";
    return {
      identity: this,
      surface: () => this.surface,
      read: id => { const record = this.#record(read, snapshot, view); return record?.id === id ? record : undefined; },
      subscribe: this.subscribe,
      disabled: record => borrow(record, trigger(record), () => this.session.canInteract(record.id, trigger(record))) !== true,
      value: record => borrow(record, trigger(record), () => this.session.display(record.id, trigger(record))),
      dispatch: (record, event, input?: UiValue) => {
        let result: StagedIntentResult | undefined;
        borrow(record, event, () => {
          if (this.#sequence === maximum || !this.session.canInteract(record.id, event)) return;
          const control = resident(record, event);
          if (!control) return;
          result = this.session.handle({ surface: this.surface, revision: view.revision, node: record.id, nodeKey: record.key, trigger: event, action: control.action, args: null, input, seq: ++this.#sequence });
        });
        if (result?.kind === "staged") {
          this.#notify();
          const request = this.session.request();
          if (!this.#closed && current(record) && this.#currentStatus() && request?.token === result.token && request.args === result.args) this.staged(result);
        }
      },
    };
  }
  close(): void {
    if (this.#closed && this.#live.size === 0) return;
    this.#closed = true;
    this.#status = undefined;
    this.#borrow = undefined;
    this.session.close();
    let failure: { readonly error: unknown } | undefined;
    for (const read of this.#live) { try { this.source.unsubscribeNode(read); this.#live.delete(read); } catch (error) { failure ??= { error }; } }
    this.#notify();
    this.#listeners.clear();
    if (failure) throw failure.error;
  }
}
