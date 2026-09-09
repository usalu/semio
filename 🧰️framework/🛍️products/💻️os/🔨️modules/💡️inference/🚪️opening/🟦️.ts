/** 🚪️ Exact worker admission for the Shell's single inference opening. */
export type InferencePortOpeningOwnerV1 = { readonly operationEpoch: number; readonly scope: { readonly spaceId: string; readonly documentId: string } };
export type InferencePortOpeningRequestV1 = InferencePortOpeningOwnerV1 & { readonly kind: "inference-open" };
export type InferencePortClosedV1 = InferencePortOpeningOwnerV1 & { readonly kind: "inference-port-closed" };
export type InferencePortOpeningResultV1 = InferencePortOpeningOwnerV1 & {
  readonly kind: "inference-port-opened";
  readonly outcome: "opened" | "refused" | "indeterminate";
  readonly code: null | "inference.capacity" | "inference.denied" | "inference.invalid" | "inference.lease-unverified" | "inference.transport";
};

function exact(value: unknown, fields: readonly string[]): Readonly<Record<string, unknown>> {
  if (value === null || typeof value !== "object" || Array.isArray(value) || Object.getPrototypeOf(value) !== Object.prototype) throw new Error("inference-opening: invalid object");
  const record = value as Readonly<Record<string, unknown>>;
  if (Object.keys(record).length !== fields.length || fields.some((field) => !Object.hasOwn(record, field))) throw new Error("inference-opening: invalid fields");
  return record;
}

function text(value: unknown): string {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > 256 || /[\u0000-\u001f\u007f]/u.test(value)) throw new Error("inference-opening: invalid identifier");
  return value;
}

function owner(value: Readonly<Record<string, unknown>>): InferencePortOpeningOwnerV1 {
  if (typeof value.operationEpoch !== "number" || !Number.isSafeInteger(value.operationEpoch) || value.operationEpoch < 1) throw new Error("inference-opening: invalid epoch");
  const scope = exact(value.scope, ["spaceId", "documentId"]);
  return { operationEpoch: value.operationEpoch, scope: { spaceId: text(scope.spaceId), documentId: text(scope.documentId) } };
}

/** 🛂️ Parses an opening without accepting job, principal or credential authority. */
export function parseInferencePortOpeningRequestV1(value: unknown): InferencePortOpeningRequestV1 {
  const record = exact(value, ["kind", "operationEpoch", "scope"]);
  if (record.kind !== "inference-open") throw new Error("inference-opening: invalid kind");
  return { kind: "inference-open", ...owner(record) };
}

/** 📬️ An exact refusal cannot masquerade as successful worker admission. */
export function parseInferencePortClosedV1(value: unknown): InferencePortClosedV1 {
  const record = exact(value, ["kind", "operationEpoch", "scope"]);
  if (record.kind !== "inference-port-closed") throw new Error("inference-closing: invalid kind");
  return { kind: "inference-port-closed", ...owner(record) };
}

/** 📨️ An exact refusal cannot masquerade as successful worker admission. */
export function parseInferencePortOpeningResultV1(value: unknown): InferencePortOpeningResultV1 {
  const record = exact(value, ["kind", "operationEpoch", "scope", "outcome", "code"]);
  if (record.kind !== "inference-port-opened" || (record.outcome !== "opened" && record.outcome !== "refused" && record.outcome !== "indeterminate")) throw new Error("inference-opening: invalid disposition");
  const codes: Readonly<Record<InferencePortOpeningResultV1["outcome"], readonly unknown[]>> = { opened: [null], refused: ["inference.capacity", "inference.denied", "inference.invalid", "inference.lease-unverified"], indeterminate: ["inference.transport"] };
  if (!codes[record.outcome].includes(record.code)) throw new Error("inference-opening: invalid code");
  return { kind: "inference-port-opened", ...owner(record), outcome: record.outcome, code: record.code as InferencePortOpeningResultV1["code"] };
}

/** 📮️ A single pending opening, settled only by its exact private worker reply. */
export class InferencePortOpeningMailboxV1 {
  private pending: { readonly request: InferencePortOpeningRequestV1; readonly timer: ReturnType<typeof setTimeout>; readonly resolve: (result: InferencePortOpeningResultV1) => void; readonly reject: (error: Error) => void } | null = null;
  private closed = false;
  private lastEpoch = 0;

  constructor(private readonly send: (request: InferencePortOpeningRequestV1) => void, private readonly timeoutMs = 30_000) {
    if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > 300_000) throw new Error("inference-opening: invalid timeout");
  }

  /** 📨️ Installs the waiter before sending, so a synchronous receipt cannot be lost. */
  async open(value: InferencePortOpeningRequestV1): Promise<InferencePortOpeningResultV1> {
    if (this.closed) throw new Error("inference-opening: closed");
    if (this.pending !== null) throw new Error("inference-opening: pending");
    const request = parseInferencePortOpeningRequestV1(value);
    if (request.operationEpoch <= this.lastEpoch) throw new Error("inference-opening: replayed epoch");
    this.lastEpoch = request.operationEpoch;
    return await new Promise<InferencePortOpeningResultV1>((resolve, reject) => {
      const timer = setTimeout(() => this.close("inference-opening: admission unconfirmed"), this.timeoutMs);
      const pending = { request, timer, resolve, reject };
      this.pending = pending;
      try { this.send(request); }
      catch (error) {
        if (this.pending !== pending) return;
        this.pending = null;
        clearTimeout(timer);
        reject(error instanceof Error ? error : new Error("inference-opening: transport failed"));
      }
    });
  }

  /** 🪪️ Stale, malformed and replayed dispositions never move the retained owner. */
  settle(value: unknown): boolean {
    if (this.closed || this.pending === null) return false;
    let result: InferencePortOpeningResultV1;
    try { result = parseInferencePortOpeningResultV1(value); } catch { return false; }
    const pending = this.pending;
    if (result.operationEpoch !== pending.request.operationEpoch || result.scope.spaceId !== pending.request.scope.spaceId || result.scope.documentId !== pending.request.scope.documentId) return false;
    this.pending = null;
    clearTimeout(pending.timer);
    if (result.outcome === "opened") pending.resolve(result);
    else pending.reject(new Error(result.code!));
    return true;
  }

  /** 🛑️ Closing refuses admission locally without claiming that a Hub job was cancelled. */
  close(reason: string): void {
    this.closed = true;
    const pending = this.pending;
    this.pending = null;
    if (pending === null) return;
    clearTimeout(pending.timer);
    pending.reject(new Error(reason));
  }
}
