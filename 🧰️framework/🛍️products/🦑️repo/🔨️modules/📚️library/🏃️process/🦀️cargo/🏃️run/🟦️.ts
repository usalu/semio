import { runCmd } from "../../🟦️.ts";
import { selectedCargoArguments } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";

/** 🦀️Runs `cargo` through [[runCmd]] — on POSIX via the cargo relay, which pipes and forwards its output ([[cargoStreamingStatus]]). */
export function runCargo(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  runCmd("cargo", selectedCargoArguments(getWorkspaceRoot(), args), { cwd, env });
}
