import { dirname, resolve } from "node:path";

/** 🔒️ Resource lease admission control: the access descriptor, the awaited wait progress, the caller control and the materialized path bound every lease owner shares; a leaf, so leases, retirement and routing never import each other for these. */
export type ResourceAccess = { readonly resource: string; readonly mode: "shared" | "exclusive" };
export type LeaseWait = ResourceAccess & { readonly elapsedMs: number };
/** 🎛️ Preserves an admitted caller's existing signal, deadline and awaited wait transitions. */
export interface ResourceLeaseControl {
  readonly signal: AbortSignal;
  remainingMilliseconds(): number | null;
  publish(progress: LeaseWait): void | Promise<void>;
  yieldContinuation(): Promise<void>;
}

/** ⏳️ Refuses expired caller authority without replacing its cancellation signal. */
export function checkResourceLeaseControl(control: ResourceLeaseControl): number | null {
  if (!control || !(control.signal instanceof AbortSignal) || typeof control.remainingMilliseconds !== "function" || typeof control.publish !== "function" || typeof control.yieldContinuation !== "function") throw Error("Complete resource admission control required");
  control.signal.throwIfAborted();
  const remaining = control.remainingMilliseconds();
  if (remaining !== null && (!Number.isFinite(remaining) || remaining <= 0)) throw Error("Original compiler admission deadline exhausted");
  return remaining;
}

/** ⌛️ Calls `expire` once with the refusal when the control's authority ends. The control is read again every time its end is believed due, so a renewable (idle-window) deadline moves the expiry and a fixed one is met exactly; one delay never exceeds the native timer range. Returns the call that retires the watch. */
export function watchResourceLeaseControlDeadlineV1(control: ResourceLeaseControl, expire: (reason: unknown) => void): () => void {
  let active = true, timer: ReturnType<typeof setTimeout> | undefined;
  const arm = (): void => {
    if (!active) return;
    let remaining: number | null;
    try { remaining = checkResourceLeaseControl(control); } catch (error) { active = false; expire(error); return; }
    if (remaining === null) return;
    timer = setTimeout(arm, Math.max(1, Math.min(2147483647, Math.ceil(remaining))));
  };
  arm();
  return () => { active = false; if (timer !== undefined) clearTimeout(timer); };
}

/** 📏️ Checks the complete resolved materialized path in both portable string measures. */
export function requireMaterializedPathV1(path: string): string {
  const absolute = resolve(path);
  if (absolute.includes("\0") || absolute.length > 256 || [...absolute].length > 256) throw Error("Materialized absolute path exceeds 256 characters");
  return absolute;
}

/** 🔗️ Whether an ancestor symlink belongs to the operating system rather than to the store: a link directly below the filesystem root that only root could create (macOS `/var`, `/tmp`, `/etc`), or on Windows any link directly below the drive root. Such a link cannot be substituted by the store's users, so the strict no-symlink rule stands for every other ancestor. */
export function isSystemLeaseLinkV1(current: string, stat: Readonly<{ uid: number }>): boolean {
  const parent = dirname(current);
  if (process.platform === "win32") return dirname(parent) === parent;
  return stat.uid === 0 && (process.getuid?.() !== 0 || dirname(parent) === parent);
}
