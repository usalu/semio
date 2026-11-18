import{machinePathV1}from"../../../🏗️build/🏠️machine-state/🟦️.ts";
import { join, resolve } from "node:path";
import { attachedDiskImagesV1 } from "../🖼️images/🟦️.ts";
import { classifyAttachedImagesV1, sweepLeakedSandboxImagesV1 } from "./🟦️.ts";

/** 🚪️ Sweeps the disk images of dead test sandboxes: `[--root=<checkout>] [--dry-run]`; prints the report as JSON, exits 1 on failure. */
function main(argv: readonly string[]): void {
  const option = (name: string): string | undefined => argv.find(argument => argument.startsWith(name + "="))?.slice(name.length + 1);
  const root = resolve(option("--root") ?? process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd()), parent = machinePathV1(root, ".cache", "test-sandbox");
  if (argv.includes("--dry-run")) {
    const rows = classifyAttachedImagesV1(attachedDiskImagesV1());
    console.log(JSON.stringify({ dryRun: true, total: rows.length, ours: rows.filter(row => row.kind === "ours").length, leaked: rows.filter(row => row.kind === "leaked").length, foreign: rows.filter(row => row.kind === "foreign").length }));
    return;
  }
  console.log(JSON.stringify(sweepLeakedSandboxImagesV1({ parents: [parent] })));
}

try { main(process.argv.slice(2)); } catch (error) { console.error(error instanceof Error ? error.message : String(error)); process.exit(1); }
