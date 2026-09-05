/** 🩺 Window-fault classification for the "app mounts, body stays empty" family. The plugin runtime
 * emits one discriminating `Fault.code` per cleanup-loop cause
 * (`🔌️plugin/🩺️runtime-fault-vectors.json`); this is the client half that stops throwing that code
 * away and turns it into a small typed set a smoke harness can read straight off the DOM. */

export type WindowFaultClass = "abi-mismatch" | "interactive-ceiling" | "clock" | "plugin-internal" | "install-failed" | "unknown";

/** 🩺 `data-semio-window-fault` — the DOM attribute a catalog smoke reads to name the cause. */
export const WINDOW_FAULT_ATTRIBUTE = "data-semio-window-fault";

const WINDOW_FAULT_CODE_CLASSES: readonly (readonly [string, WindowFaultClass])[] = [
  ["plugin.internal.abi-mismatch", "abi-mismatch"],
  ["plugin.internal.interactive-ceiling", "interactive-ceiling"],
  ["plugin.internal.clock", "clock"],
  ["plugin.internal", "plugin-internal"],
];

/** 🩺 One decoded wire fault reduced to what the shell can act on: the class, the raw code and
 * origin (never discarded any more), and the message the plugin already formatted for us. */
export type WindowFault = {
  readonly class: WindowFaultClass;
  readonly code: string | undefined;
  readonly origin: string | undefined;
  readonly message: string;
};

/** 🩺 A crashed/quarantined supervisor outranks every code: the module never installed, so no
 * cleanup-loop cause it might also report describes the real failure. */
export function classifyWindowFault(code: string | undefined, supervisor?: string): WindowFaultClass {
  if (supervisor === "crashed" || supervisor === "quarantined") return "install-failed";
  if (!code) return "unknown";
  return WINDOW_FAULT_CODE_CLASSES.find(([prefix]) => code === prefix || code.startsWith(prefix))?.[1] ?? "unknown";
}

/** 🩺 Reads `code`/`origin`/`message` off whatever the plugin bridge rejected with — a decoded
 * `Fault`, a `SemioFaultError` carrying one, or a bare `Error` — without ever losing the code. */
export function windowFaultFromError(error: unknown, supervisor?: string): WindowFault {
  const candidate = (error ?? {}) as { readonly code?: unknown; readonly origin?: unknown; readonly message?: unknown; readonly fault?: { readonly code?: unknown; readonly origin?: unknown; readonly message?: unknown } };
  const fault = candidate.fault ?? candidate;
  const code = typeof fault.code === "string" ? fault.code : undefined;
  const origin = typeof fault.origin === "string" ? fault.origin : undefined;
  const message = typeof candidate.message === "string" ? candidate.message : typeof fault.message === "string" ? fault.message : String(error);
  return { class: classifyWindowFault(code, supervisor), code, origin, message };
}
