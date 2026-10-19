import { join } from "node:path";
import { runCargo } from "@semio-tech/process/execution";

/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if (true) {
    const [command, artifact, standard, subset, ...surface] = process.argv.slice(2);
    if (command !== "list-mutations" || !artifact || !standard || !subset || surface.length > 1) throw new Error("list-mutations <artifact> <standard> <subset> [<surface>]");
    await runCargo(["run", "--quiet", "--offline", "--manifest-path", join(import.meta.dir, "📦️packages/🦀️rust/Cargo.toml"), "--bin", "semio-hub-stdio-mutation-bridge", "--", command, artifact, standard, subset, ...surface], import.meta.dir);
  }
}
