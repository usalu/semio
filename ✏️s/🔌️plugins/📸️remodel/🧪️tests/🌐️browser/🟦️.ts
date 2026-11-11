import { checkScriptInvocation, type ScriptInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTwoHumanCli } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts";
import scenarios from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { inspectRemodelOpenedV1, inspectRemodelViewerV1, inspectRemodelColdV1, inspectRemodelWindowsV1 } from "./🧭️journey/🟦️.ts";
import { validateRemodelTwoHumanArgumentsV1 } from "./📋️report/🟦️.ts";

/** 📸️ Runs the plugin checks inside the existing two-human shared-artifact journey. */
export async function runRemodelBrowserAcceptanceV1(invocation: ScriptInvocation, repoRoot: string, segments: readonly string[]): Promise<void> {
  checkScriptInvocation(invocation);
  invocation.control.signal.throwIfAborted();
  validateRemodelTwoHumanArgumentsV1(segments);
  const deferred = new Set(["windows-and-viewer", "cold-reopen"]);
  let windows: readonly string[] = [];
  await runTwoHumanCli(invocation, repoRoot, "", segments, { required: scenarios.filter(row => !deferred.has(row.id)).map(row => row.id), run: async context => { const checks = await inspectRemodelOpenedV1(context, repoRoot); windows = await inspectRemodelWindowsV1(context); return checks.filter(row => !deferred.has(row.id)); }, viewer: { required: ["windows-and-viewer"], run: async context => (await inspectRemodelViewerV1(context)).map(row => ({ ...row, evidence: [...windows, ...row.evidence] })) }, cold: { required: ["cold-reopen"], run: inspectRemodelColdV1 } });
}
