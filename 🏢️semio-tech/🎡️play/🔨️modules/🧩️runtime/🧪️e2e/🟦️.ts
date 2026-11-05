import { join } from "node:path";

export const PLAY_E2E_OWNER = "@semio-tech/semio-tech-play:e2e";

/** 🗂️ Locates ephemeral E2E service records outside cacheable application outputs. */
export function playE2eSessionRoot(workspace: string): string {
  return join(workspace, "🏢️semio-tech/🎡️play/dist/services/e2e");
}

/** 🎟️ Keeps scoped and full services independent even inside one multi-target build invocation. */
export function playE2eServiceV1(workspace: string, pane?: string): { readonly root: string; readonly owner: string } {
  if (pane === undefined) return { root: playE2eSessionRoot(workspace), owner: PLAY_E2E_OWNER };
  if (pane !== "forms") throw new Error(`Unknown Play E2E pane: ${pane}`);
  return { root: join(workspace, "🏢️semio-tech/🎡️play/dist/services/forms-e2e"), owner: "@semio-tech/semio-tech-play:forms-e2e" };
}

/** 🧭️ Requires the selected backend’s shared invocation identity before preparing a fresh service generation. */
export function playE2eInvocationPid(environment: Readonly<Record<string, string | undefined>>): number {
  const value = environment.SEMIO_BUILD_INVOCATION_PID;
  if (!value || !/^[1-9][0-9]*$/.test(value) || !Number.isSafeInteger(Number(value))) throw new Error("Play E2E requires a shared repository build invocation");
  return Number(value);
}
