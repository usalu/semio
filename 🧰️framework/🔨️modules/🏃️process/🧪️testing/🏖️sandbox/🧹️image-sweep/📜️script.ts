import{machinePathV1}from"../../../🏗️build/🏠️machine-state/🟦️.ts";
import { resolve } from "node:path";
import { attachedDiskImagesV1 } from "../🖼️images/🟦️.ts";
import { classifyAttachedImagesV1, sweepLeakedSandboxImagesV1 } from "./🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type:"json" };

/** 🚪️ Sweeps dead test sandbox images and prints a JSON report. Workspace-only mode confines image selection to the specified checkout and skips sandbox-directory removal. */
function main(argv: readonly string[]): void {
  const option = (name: string): string | undefined => argv.find(argument => argument.startsWith(name + "="))?.slice(name.length + 1);
  const root = resolve(option("--root") ?? process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd()), workspaceOnly = argv.includes(schema.definitions.Protocol.const.workspaceOnlyFlag);
  if (argv.includes("--dry-run")) {
    const rows = classifyAttachedImagesV1(attachedDiskImagesV1().filter(row=>!workspaceOnly||row.image.startsWith(root+"/")));
    console.log(JSON.stringify({ dryRun: true, total: rows.length, ours: rows.filter(row => row.kind === "ours").length, leaked: rows.filter(row => row.kind === "leaked").length, foreign: rows.filter(row => row.kind === "foreign").length }));
    return;
  }
  console.log(JSON.stringify(sweepLeakedSandboxImagesV1(workspaceOnly?{under:root}:{parents:[machinePathV1(root,".cache","test-sandbox")]})));
}

try { main(process.argv.slice(2)); } catch (error) { console.error(error instanceof Error ? error.message : String(error)); process.exit(1); }
