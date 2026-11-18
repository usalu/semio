import { readFileSync, rmSync } from "node:fs";
import { deserialize } from "node:v8";
import { Worker } from "node:worker_threads";

/** 👷️ The worker process of the graph inference farm: it opens the session its parent names in its environment at once (the parent's event loop is never needed to start it), announces itself, then runs jobs one at a time, answering every job with plain data; a message that cannot be sent is answered as a failure, never swallowed. */
const MEGABYTE = 1048576, INIT = "SEMIO_GRAPH_WORKER_INIT";
let service, opened;

const fail = (id, error) => ({ type: "failure", id, name: String(error?.name ?? "Error"), message: String(error?.message ?? error), code: typeof error?.code === "string" ? error.code : null, rssMegabytes: process.memoryUsage().rss / MEGABYTE });

const open = async text => {
  const init = JSON.parse(text), url = new URL(init.service);
  if (url.protocol !== "file:") throw Error("File-addressed farm service required");
  service = await import(init.service);
  const identity = await service.open(deserialize(readFileSync(init.sessionFile)), { slot: init.slot, generation: init.generation, megabytes: init.megabytes });
  process.send({ type: "ready", pid: process.pid, identity });
};

const queue = { tail: Promise.resolve() };
const serve = async message => {
  try {
    await opened;
    if (message?.type === "run") {
      let answer;
      try { const job = JSON.parse(readFileSync(message.jobFile, "utf8")); rmSync(message.jobFile, { force: true }); const reply = await service.run(job); try { globalThis.Bun?.gc?.(true); if (service.trimRetainedV1?.() === true) globalThis.Bun?.gc?.(true); } catch {} answer = { type: "result", id: message.id, reply, rssMegabytes: process.memoryUsage().rss / MEGABYTE }; }
      catch (error) { answer = fail(message.id, error); }
      try { process.send(answer); }
      catch (error) { process.send(fail(message.id, error)); }
    }
  } catch (error) {
    try { process.send({ type: "fatal", name: String(error?.name ?? "Error"), message: String(error?.message ?? error) }); } finally { process.exit(1); }
  }
};

process.on("disconnect", () => process.exit(0));
try { new Worker(new URL("../🐕️watchdog/🟨️.mjs", import.meta.url), { workerData: { parentPid: process.ppid } }).unref(); } catch {}
process.on("message", message => {
  if (message?.type === "shutdown") { Promise.resolve(service?.close?.()).finally(() => process.exit(0)); return; }
  queue.tail = queue.tail.then(() => serve(message));
});
const text = process.env[INIT];
delete process.env[INIT];
opened = text === undefined ? Promise.reject(Error("Graph farm worker init required: " + INIT)) : open(text);
opened.catch(error => { try { process.send({ type: "fatal", name: String(error?.name ?? "Error"), message: String(error?.message ?? error) }); } finally { process.exit(1); } });
