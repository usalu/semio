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
  readonly notice?: WindowFaultNoticeV1;
};

/** 🌐️ The localized text a fault carries for the user, one entry per supported UI language. */
export type WindowFaultNoticeV1 = Readonly<{ en: string; de: string }>;

/** 🌐️ The text the fault card shows: the fault's localized notice in `locale`, else its diagnostic message; no language is assumed. */
export function windowFaultTextV1(fault: WindowFault | null, message: string, locale: string): string {
  const notice = fault?.notice;
  return notice && (locale === "en" || locale === "de") ? notice[locale] : message;
}

/** 🔎️ Reads the localized notice an error carries, when it names one for every supported UI language. */
function windowFaultNotice(value: unknown): WindowFaultNoticeV1 | undefined {
  const notice = value as { readonly en?: unknown; readonly de?: unknown } | null | undefined;
  return typeof notice?.en === "string" && typeof notice.de === "string" ? { en: notice.en, de: notice.de } : undefined;
}

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
  const candidate = (error ?? {}) as { readonly code?: unknown; readonly origin?: unknown; readonly message?: unknown; readonly notice?: unknown; readonly fault?: { readonly code?: unknown; readonly origin?: unknown; readonly message?: unknown } };
  const fault = candidate.fault ?? candidate;
  const code = typeof fault.code === "string" ? fault.code : undefined;
  const origin = typeof fault.origin === "string" ? fault.origin : undefined;
  const message = typeof candidate.message === "string" ? candidate.message : typeof fault.message === "string" ? fault.message : String(error);
  const notice = windowFaultNotice(candidate.notice);
  return { class: classifyWindowFault(code, supervisor), code, origin, message, ...(notice ? { notice } : {}) };
}

/** 🪦 The window fault a failed pass raises for a LIVE instance, or `null` when `retired` says the failure only means the
 * instance it addressed was retired under it (a session switch, a hot swap, a revoked activation): that window is already
 * gone, and a fault raised for it stays on its successor's screen — measured as `no actor for instance 1` /
 * `actor-activation.revoked` painted over the Space app after a `/spaces/<id>` load (🎫️ 26/09/23 C10). */
export function liveInstanceWindowFaultV1(error: unknown, supervisor: string | undefined, retired: (error: unknown) => boolean): WindowFault | null {
  return retired(error) ? null : windowFaultFromError(error, supervisor);
}
