import { readFileSync, rmSync } from "node:fs";
import { deserialize } from "node:v8";

/** 👷️ The worker process of the graph inference farm: loads the service module its parent names, opens one session and runs jobs one at a time, answering every job with plain data; a message that cannot be sent is answered as a failure, never swallowed. */
const MEGABYTE = 1048576;
let service;

const fail = (id, error) => ({ type: "failure", id, name: String(error?.name ?? "Error"), message: String(error?.message ?? error), code: typeof error?.code === "string" ? error.code : null });

process.on("disconnect", () => process.exit(0));
process.on("message", async message => {
  try {
    if (message?.type === "init") {
      const url = new URL(message.service);
      if (url.protocol !== "file:") throw Error("File-addressed farm service required");
      service = await import(message.service);
      const identity = await service.open(deserialize(readFileSync(message.sessionFile)), { slot: message.slot, generation: message.generation });
      process.send({ type: "ready", pid: process.pid, identity });
    } else if (message?.type === "run") {
      let answer;
      try { const job = JSON.parse(readFileSync(message.jobFile, "utf8")); rmSync(message.jobFile, { force: true }); answer = { type: "result", id: message.id, reply: await service.run(job), rssMegabytes: process.memoryUsage().rss / MEGABYTE }; }
      catch (error) { answer = fail(message.id, error); }
      try { process.send(answer); }
      catch (error) { process.send(fail(message.id, error)); }
    } else if (message?.type === "shutdown") {
      await service?.close?.();
      process.exit(0);
    }
  } catch (error) {
    try { process.send({ type: "fatal", name: String(error?.name ?? "Error"), message: String(error?.message ?? error) }); } finally { process.exit(1); }
  }
});
