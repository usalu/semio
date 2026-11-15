/** 🛑️ The exit status of a script process the user cancelled with SIGINT: the runtime contract for a SIGINT-cancelled script. */
export const SCRIPT_INTERRUPTED_EXIT_CODE_V1 = 130;

/** 🛑️ The typed cancellation of a script process by SIGINT; it is recognised by its code, never by its message. */
export class ScriptProcessInterruptedV1 extends Error {
  readonly code = "SCRIPT_PROCESS_INTERRUPTED";
  constructor() {
    super("Original Script process interrupted");
    this.name = "ScriptProcessInterruptedV1";
  }
}

/** 🔎️ Whether an error is the user's cancellation of a script process, whichever module instance created it. */
export const isScriptProcessInterruptedV1 = (error: unknown): boolean =>
  typeof error === "object" && error !== null && (error as { code?: unknown }).code === "SCRIPT_PROCESS_INTERRUPTED";
