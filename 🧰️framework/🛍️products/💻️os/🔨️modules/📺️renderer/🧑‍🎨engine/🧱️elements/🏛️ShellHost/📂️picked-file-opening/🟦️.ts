import vocabulary from "./🔣️.json" with { type: "json" };
import { localizedLabelTextV1 } from "../📣️replay-refusal/🟦️.ts";
import { FormatResolutionErrorV1, pickRevisionV1, resolveFormatV1, type FormatClaimsV1, type FormatResolutionV1, type RevisionSniffV1 } from "../../🛠️ShellHelpers/🧭️format-resolution/🟦️.ts";


export type PickedFileRefusalReasonV1 = keyof typeof vocabulary.reasons;

/** 🪝️ What the shell's Open file needs from its composition root: the registered format claims and the dialect coordinate (`kind@standard/subset`) whose owner opens each format. */
export type PickedFileFormatCatalogPortV1 = Readonly<{
  descriptors(): readonly FormatClaimsV1[];
  artifactRefOf(kindId: string): string | undefined;
  revisionsOf?(kindId: string): readonly RevisionSniffV1[];
}>;

/** 🔬️ How many leading bytes of a picked file the revision sniff may read. */
export const PICKED_FILE_SNIFF_BYTES_V1 = 4096;

export type PickedFileRouteV1 = Readonly<{ kind: "routed"; artifactRef: string; resolution: FormatResolutionV1 }> | Readonly<{ kind: "refused"; reason: PickedFileRefusalReasonV1 }>;

/** 📂️ Routes one picked file to the dialect whose owner editor opens it, picking among a format\'s revisions by sniffing the file head; every dead end is a named refusal, never a guess or a fallback format. */
export async function routePickedFileV1(catalog: PickedFileFormatCatalogPortV1 | undefined, file: Readonly<{ name: string; type?: string }>, readHead: (bytes: number) => Promise<Uint8Array>): Promise<PickedFileRouteV1> {
  if (catalog === undefined) return { kind: "refused", reason: "format-catalog-unavailable" };
  let resolution: FormatResolutionV1 | null;
  try {
    resolution = resolveFormatV1(catalog.descriptors(), { fileName: file.name, ...(file.type ? { mime: file.type } : {}) });
  } catch (error) {
    return { kind: "refused", reason: error instanceof FormatResolutionErrorV1 && error.code === "ambiguous" ? "format-ambiguous" : "format-unresolved" };
  }
  if (resolution === null) return { kind: "refused", reason: "format-unresolved" };
  const revisions = catalog.revisionsOf?.(resolution.format.kindId) ?? [];
  const kindId = revisions.length === 0 ? resolution.format.kindId : pickRevisionV1(resolution.format.kindId, revisions, await readHead(PICKED_FILE_SNIFF_BYTES_V1));
  const artifactRef = catalog.artifactRefOf(kindId);
  return artifactRef === undefined ? { kind: "refused", reason: "owner-unavailable" } : { kind: "routed", artifactRef, resolution };
}

/** 🗣️ Refusal notice text with `{file}` filled; an unknown locale renders every language. */
export function pickedFileRefusalTextV1(reason: PickedFileRefusalReasonV1, file: string, locale: string): string {
  return localizedLabelTextV1(vocabulary.reasons[reason], locale).replaceAll("{file}", file);
}

/** 🩺️ Fault code carried on the refusal notice. */
export function pickedFileRefusalCodeV1(reason: PickedFileRefusalReasonV1): string {
  return `${vocabulary.codePrefix}${reason}`;
}

/** 📖️ Reads the leading bytes of a picked blob; a cancel settles the wait at once even though the browser read itself cannot be aborted. */
export async function readPickedFileHeadV1(blob: Pick<Blob, "slice">, bytes: number, signal: AbortSignal): Promise<Uint8Array> {
  if (signal.aborted) throw signal.reason ?? new DOMException("The operation was aborted", "AbortError");
  let onAbort: (() => void) | undefined;
  const cancelled = new Promise<never>((_resolve, reject) => {
    onAbort = () => reject(signal.reason ?? new DOMException("The operation was aborted", "AbortError"));
    signal.addEventListener("abort", onAbort, { once: true });
  });
  try {
    return await Promise.race([blob.slice(0, bytes).arrayBuffer().then((buffer) => new Uint8Array(buffer)), cancelled]);
  } finally {
    if (onAbort !== undefined) signal.removeEventListener("abort", onAbort);
  }
}

export type PickedFileTaskV1 = Readonly<{ signal: AbortSignal; finish: () => void; progress: (completed: number, total: number | null) => void }>;

/** 🔔️ Every outcome the Open file flow tells a person about. */
export type PickedFileOutcomeV1 =
  | Readonly<{ kind: "opened" | "cancelled" | "failed" | "busy"; name: string }>
  | Readonly<{ kind: "refused"; reason: PickedFileRefusalReasonV1; name: string }>;

/** 🪝️ What one Open file gesture needs from its host; the single-flight slot is claimed only after the picker answered, so a picker that never reports back cannot block later opens. */
export interface PickedFileFlowPortV1<TOwner, TPicked extends Readonly<{ name: string }>> {
  readonly select: () => Promise<TPicked | undefined>;
  readonly claim: () => boolean;
  readonly release: () => void;
  readonly track: (name: string) => PickedFileTaskV1;
  readonly route: (picked: TPicked, signal: AbortSignal) => Promise<PickedFileRouteV1>;
  readonly resolveOwner: (route: Extract<PickedFileRouteV1, { kind: "routed" }>) => Promise<TOwner | null>;
  readonly importInto: (owner: TOwner, picked: TPicked, task: PickedFileTaskV1) => Promise<number>;
  readonly commit: (owner: TOwner, instanceId: number, task: PickedFileTaskV1) => boolean | Promise<boolean>;
  readonly destroy: (owner: TOwner, instanceId: number) => Promise<void>;
  readonly report: (message: string, error: unknown) => void;
  readonly notify: (outcome: PickedFileOutcomeV1) => void;
}

/** 💾️ Awaits admission before publishing opened; cancellation retires the caller even if storage is still settling. */
async function commitPickedFileV1<TOwner, TPicked extends Readonly<{ name: string }>>(port: PickedFileFlowPortV1<TOwner, TPicked>, owner: TOwner, instanceId: number, task: PickedFileTaskV1): Promise<boolean> {
  task.signal.throwIfAborted();
  let abort!: () => void;
  const cancelled = new Promise<never>((_resolve, reject) => {
    abort = () => reject(task.signal.reason ?? new DOMException("The operation was aborted", "AbortError"));
    task.signal.addEventListener("abort", abort, { once: true });
  });
  try {
    const committed = await Promise.race([Promise.resolve().then(() => { task.signal.throwIfAborted(); return port.commit(owner, instanceId, task); }), cancelled]);
    task.signal.throwIfAborted();
    return committed;
  } finally {
    task.signal.removeEventListener("abort", abort);
  }
}

/** 📂️ One Open file gesture, every failure visible and every created instance released: selection, routing, head read, owner resolution, import and the panel commit all run inside one guarded, cancellable task. */
export async function openPickedFileV1<TOwner, TPicked extends Readonly<{ name: string }>>(port: PickedFileFlowPortV1<TOwner, TPicked>): Promise<void> {
  let picked: TPicked | undefined;
  try {
    picked = await port.select();
  } catch (error) {
    port.report("file selection failed", error);
    return port.notify({ kind: "failed", name: "" });
  }
  if (picked === undefined) return;
  if (!port.claim()) return port.notify({ kind: "busy", name: picked.name });
  const task = port.track(picked.name);
  let created: { owner: TOwner; instanceId: number } | null = null;
  try {
    task.signal.throwIfAborted();
    const route = await port.route(picked, task.signal);
    if (route.kind === "refused") return port.notify({ kind: "refused", reason: route.reason, name: picked.name });
    const owner = await port.resolveOwner(route);
    if (owner === null) return port.notify({ kind: "refused", reason: "owner-unavailable", name: picked.name });
    task.signal.throwIfAborted();
    const instanceId = await port.importInto(owner, picked, task);
    created = { owner, instanceId };
    if (!await commitPickedFileV1(port, owner, instanceId, task)) throw new Error("open-file.commit-unavailable");
    created = null;
    port.notify({ kind: "opened", name: picked.name });
  } catch (error) {
    port.report("file open failed", error);
    if (created !== null) await port.destroy(created.owner, created.instanceId).catch((destroyError: unknown) => port.report("opened instance could not be closed", destroyError));
    port.notify({ kind: task.signal.aborted ? "cancelled" : "failed", name: picked.name });
  } finally {
    task.finish();
    port.release();
  }
}
