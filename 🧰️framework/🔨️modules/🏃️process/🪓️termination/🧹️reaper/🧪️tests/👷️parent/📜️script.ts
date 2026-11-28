import { spawn } from "node:child_process";
import { reapWithParentV1 } from "../../🟦️.ts";

const kinds = ["polite", "stubborn", "released"] as const;
const shell: Record<(typeof kinds)[number], string> = { polite: "sleep 300 & wait", stubborn: 'trap "" TERM; (trap "" TERM; sleep 300) & while :; do sleep 1; done', released: "sleep 300 & wait" };
const pids: Record<string, number> = {};
for (const kind of kinds) {
  const child = spawn("sh", ["-c", shell[kind]], { detached: true, stdio: "ignore" });
  pids[kind] = child.pid!;
  const unregister = reapWithParentV1(child.pid);
  if (kind === "released") unregister();
}
console.log(JSON.stringify({ parent: process.pid, pids }));
setInterval(() => {}, 1 << 30);
