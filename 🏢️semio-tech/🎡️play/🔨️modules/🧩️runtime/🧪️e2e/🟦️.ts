import { join } from "node:path";

export const PLAY_E2E_OWNER = "@semio-tech/semio-tech-play:e2e";

/** @emoji 🗂️ Locates ephemeral E2E service records outside cacheable application outputs. */
export function playE2eSessionRoot(workspace: string): string {
  return join(workspace, "🏢️semio-tech/🎡️play/dist/services/e2e");
}

/** @emoji 🧭️ Requires Nx's shared invocation identity before preparing a fresh service generation. */
export function playE2eInvocationPid(environment: Readonly<Record<string, string | undefined>>): number {
  const value = environment.NX_INVOCATION_ROOT_PID;
  if (!value || !/^[1-9][0-9]*$/.test(value) || !Number.isSafeInteger(Number(value))) throw new Error("Play E2E must run through Nx");
  return Number(value);
}
