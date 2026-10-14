import { existsSync, readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import { isMainThread, threadId } from "node:worker_threads";

if (isMainThread) {
const directory = process.env.SEMIO_JOURNAL_CPU_PROBE_DIRECTORY;
if (!directory || !isAbsolute(directory) || directory.length > 220) throw Error("Owned daemon CPU probe directory required");
const request = join(directory, "cpu-request.json"), started = join(directory, "cpu-started.json"), finished = join(directory, "cpu-finished.json");
setInterval(() => {
  if (!existsSync(request)) return;
  const value = JSON.parse(readFileSync(request, "utf8"));
  if (Object.keys(value).join(",") !== "version,milliseconds" || value.version !== 1 || !Number.isInteger(value.milliseconds) || value.milliseconds < 1000 || value.milliseconds > 5000) throw Error("Closed finite CPU probe request required");
  unlinkSync(request); writeFileSync(started, JSON.stringify({ processId: process.pid, threadId }));
  const end = performance.now() + value.milliseconds;
  while (performance.now() < end) Math.sqrt(performance.now());
  writeFileSync(finished, JSON.stringify({ processId: process.pid, threadId }));
}, 10).unref();
}
