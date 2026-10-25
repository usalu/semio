import { fileURLToPath } from "node:url";
import { scriptInvocationEnvironment, type ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { captureOwnedCommandResultV1, writeOwnedCommandOutputV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/📥️capture/🟦️.ts";
import { resolveTypescriptCompilerV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../📚️library/📦️packages/🟦️typescript/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { runtimeTypeEnvironmentV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🧪️ Executes the typed SDK law only from the existing CLI native test route and its exact Original. */
export async function runInstalledRepoScopeTestsV1(root: string, invocation: ScriptInvocation): Promise<void> {
  const source = fileURLToPath(new URL("../🟦️.ts", import.meta.url)), policy = schema.$defs.InstalledProtocol.const;
  const environment = scriptInvocationEnvironment(invocation, repoTestArtifactEnvironment(root, "installed-repo-mcp-scope", { ...process.env }));
  for (const [name, args] of [
    ["types", [resolveTypescriptCompilerV1(root), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", runtimeTypeEnvironmentV1(), source]],
    ["sdk", ["test", "--timeout", String(policy.maximumBunMilliseconds), source]]
  ] as const) {
    const control = scriptCompilerAdmissionControlV1(invocation, "test:installed-repo-mcp:" + name, policy.maximumTestMilliseconds);
    const result = await captureOwnedCommandResultV1(process.execPath, args, root, { control, env: environment, output: { maximumBytes: 4194304, maximumLines: 8192 } });
    await writeOwnedCommandOutputV1(process.stdout, result.stdout, control); await writeOwnedCommandOutputV1(process.stderr, result.stderr, control);
    if (result.status !== 0 || result.signal !== null) throw Error("Installed repo MCP " + name + " gate failed: " + JSON.stringify({ status: result.status, signal: result.signal }));
  }
}
