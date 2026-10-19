import {writeFileSync,existsSync} from "node:fs";

/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  writeFileSync(process.argv[2]!,JSON.stringify({parent:process.ppid,worker:process.pid}));
  while(!existsSync(process.argv[3]!))await Bun.sleep(10);
  process.stdout.write("paid-child");
}
