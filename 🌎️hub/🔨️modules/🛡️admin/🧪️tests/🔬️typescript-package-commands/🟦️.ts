import type { ScriptInvocation } from "@semio-tech/process/routing";
import { scriptCompilerAdmissionControlV1 } from "@semio-tech/process/compiler-capacity";
import { runViteBunxDev } from "@semio-tech/repo-lib/dev";

async function warnWhenHubIsUnreachable(): Promise<void> {
  const hubUrl = process.env.OS_HUB_URL ?? "http://127.0.0.1:8787";
  try {
    const response = await fetch(`${hubUrl}/admin/api/overview`, { signal: AbortSignal.timeout(2_000) });
    if (response.ok) {
      console.log(`[admin] hub reachable at ${hubUrl}; use bun repo run os-hub:dev-secure-admin for authenticated administration`);
      return;
    }
    console.warn(`[admin] hub at ${hubUrl} answered ${response.status}; use bun repo run os-hub:dev-secure-admin for the authenticated relay.`);
  } catch {
    console.warn(`[admin] no hub reachable at ${hubUrl}; this Vite surface is static-only and has no administrator credential.`);
    console.warn(`[admin] start the protected surface with: bun repo run os-hub:dev-secure-admin`);
  }
}

/** 🏃️ Runs the `DevScript` command under the caller's roots and invocation. */
export async function runDevScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  await warnWhenHubIsUnreachable();
  await runViteBunxDev(context.invocation, scriptCompilerAdmissionControlV1(context.invocation,"vite:entry",0), context.root, segments, { runtimeChannels:[],
    config: "../../🏗️builder/🌐️vite/🟦️.ts",
    portEnv: "OS_HUB_ADMIN_DEV_PORT",
    defaultPort: "8790",
    fixedPort: true,
  });
}
