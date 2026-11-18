import { createInterface } from "node:readline";
import { warmOwnerV1 } from "../../🟨️.mjs";

const [directory, recordPath, spin] = process.argv.slice(2), busy = Number(spin);
const ports = { keyText: job => "key:" + job.name, read: () => ({ header: { digest: "x", observations: [] }, value: {} }), settle: async () => { const until = performance.now() + busy; while (performance.now() < until); } };
for await (const line of createInterface({ input: process.stdin })) {
  const reply = await warmOwnerV1({ name: line }, { directory, cachePath: recordPath, ports });
  process.stdout.write(JSON.stringify({ name: line, outcome: reply.outcome, rss: Math.round(process.memoryUsage().rss / 1048576) }) + "\n");
}
