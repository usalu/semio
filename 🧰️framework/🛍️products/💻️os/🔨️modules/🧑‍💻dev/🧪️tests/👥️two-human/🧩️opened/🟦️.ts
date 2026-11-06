/** 🪟 One rendered element observed through the owned test port. */
export type OpenedElementV1 = Readonly<{ text: string; visible: boolean; attributes: Readonly<Record<string, string | null>> }>;

/** 🧭 Human interactions exposed without a browser implementation type. */
export interface OpenedUiV1 {
  observe(selector: string, attributes: readonly string[], readText?: boolean): Promise<readonly OpenedElementV1[]>;
  canvas(selector: string): Promise<Readonly<{ width: number; height: number; visiblePixels: number; colors: number }> | null>;
  click(selector: string): Promise<void>;
  press(selector: string, key: string): Promise<void>;
  chooseFiles(selector: string, paths: readonly string[]): Promise<void>;
  download(selector: string): Promise<Readonly<{ name: string; mime: string | null; bytes: Uint8Array }>>;
}

/** 📋 One check backed by browser-observed evidence. */
export type OpenedCheckV1 = Readonly<{ id: string; status: "passed" | "failed" | "unsupported" | "unexecuted"; evidence: readonly string[] }>;

export type OpenedPhaseV1 = "editor" | "viewer" | "cold";
export type OpenedPhaseCheckV1 = OpenedCheckV1 & Readonly<{ phase: OpenedPhaseV1; spaceId: string; documentId: string; expectedActors: Readonly<{ a: "editor"; b: "editor" | "viewer" }> }>;

/** 🪟 Requires both visible surfaces to own the same artifact and excludes a conflicting B editor. */
function openedScopeV1(a: readonly Readonly<{ role: string; space: string; document: string; visible: boolean; instance: string; window: string }>[], b: typeof a, space: string, document: string, role: "viewer" | "editor"): boolean {
  const scoped = (row: typeof a[number]) => row.visible && row.space === space && row.document === document;
  const owned = (row: typeof a[number], role: string) => row.role === role && !!row.instance.trim() && !!row.window.trim();
  const scopedB = b.filter(scoped);
  return !!space.trim() && !!document.trim() && a.some(row => scoped(row) && owned(row, "editor")) && scopedB.length > 0 && scopedB.every(row => owned(row, role));
}

/** 👁️ Requires B's scoped surfaces to be viewer surfaces. */
export function openedViewerScopeV1(a: Parameters<typeof openedScopeV1>[0], b: Parameters<typeof openedScopeV1>[1], space: string, document: string): boolean { return openedScopeV1(a, b, space, document, "viewer"); }

/** ✏️ Requires B's scoped surfaces to be editor surfaces. */
export function openedEditorsScopeV1(a: Parameters<typeof openedScopeV1>[0], b: Parameters<typeof openedScopeV1>[1], space: string, document: string): boolean { return openedScopeV1(a, b, space, document, "editor"); }

/** 🧾 Requires every phase receipt exactly once for the same actual artifact. */
export function openedPhasesPassV1(required: readonly Readonly<{ id: string; phase: OpenedPhaseV1 }>[], receipts: readonly OpenedPhaseCheckV1[], spaceId: string, documentId: string): boolean {
  if (!spaceId.trim() || !documentId.trim() || required.some(row => !["editor", "viewer", "cold"].includes(row.phase)) || !openedChecksPassV1(required.map(row => row.id), receipts)) return false;
  const phases = new Map(required.map(row => [row.id, row.phase]));
  return receipts.every(row => row.phase === phases.get(row.id) && row.spaceId === spaceId && row.documentId === documentId && row.expectedActors?.a === "editor" && row.expectedActors.b === (row.phase === "viewer" ? "viewer" : "editor"));
}

/** 🔗 The exact shared document opened by both real sessions. */
export type OpenedArtifactV1 = Readonly<{ spaceId: string; documentId: string; locale: "en" | "de"; signal: AbortSignal; a: OpenedUiV1; b: OpenedUiV1 }>;

/** ✅ Requires each declared check once, successful, with nonempty evidence. */
export function openedChecksPassV1(required: readonly string[], checks: readonly OpenedCheckV1[]): boolean {
  const remaining = new Set(required);
  return required.every(id => id.trim().length > 0) && remaining.size > 0 && remaining.size === required.length && checks.length === required.length && checks.every(check => remaining.delete(check.id) && check.status === "passed" && check.evidence.some(value => value.trim().length > 0)) && remaining.size === 0;
}

/** 🧪 A product-owned set of checks on the shared opened artifact. */
export type OpenedArtifactChecksV1 = Readonly<{ required: readonly string[]; run(context: OpenedArtifactV1): Promise<readonly OpenedCheckV1[]>; viewer?: Readonly<{ required: readonly string[]; run(context: OpenedArtifactV1): Promise<readonly OpenedCheckV1[]> }>; cold?: Readonly<{ required: readonly string[]; run(context: OpenedArtifactV1): Promise<readonly OpenedCheckV1[]> }> }>;

/** 🧾 Records all checks as unexecuted when opening, cancellation or the callback fails. */
export async function runOpenedChecksV1(opened: OpenedArtifactChecksV1, context: OpenedArtifactV1 | null, signal: AbortSignal): Promise<readonly OpenedCheckV1[]> {
  try {
    if (context === null) throw Error("Opened artifact checks require both peers");
    signal.throwIfAborted();
    if (context.signal !== signal || !context.spaceId || !context.documentId) throw Error("Opened artifact identity or signal is invalid");
    const checks = await new Promise<readonly OpenedCheckV1[]>((resolve, reject) => {
      const abort = () => { signal.removeEventListener("abort", abort); reject(signal.reason ?? Error("Cancelled")); };
      signal.addEventListener("abort", abort, { once: true });
      if (signal.aborted) abort();
      else void Promise.resolve().then(() => { signal.throwIfAborted(); return opened.run(context); }).then(resolve, reject).finally(() => signal.removeEventListener("abort", abort));
    });
    signal.throwIfAborted();
    return checks;
  } catch (error) {
    return opened.required.map(id => ({ id, status: "unexecuted" as const, evidence: [String(error).slice(0, 500)] }));
  }
}

/** 🏷️ Separates a product check from the framework's overall receipt. */
export function openedCheckNameV1(id: string): string { return `opened: ${id}`; }
export const OPENED_CHECKS_COMPLETE_V1 = "opened checks complete";
