import { checkScriptInvocation, type ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🚀️ Prepares a browser before serving under the same issued invocation and cancellation signal. */
export async function withPreparedBrowserV1(invocation: ScriptInvocation, prepare: (invocation: ScriptInvocation) => Promise<void>, serve: (signal: AbortSignal) => Promise<void>): Promise<void> {
  checkScriptInvocation(invocation);
  invocation.control.signal.throwIfAborted();
  await prepare(invocation);
  invocation.control.signal.throwIfAborted();
  await serve(invocation.control.signal);
}
