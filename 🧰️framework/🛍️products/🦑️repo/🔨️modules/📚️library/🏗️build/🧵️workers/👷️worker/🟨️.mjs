/** 👷️ The worker thread of the build worker pool: resolves a file-addressed exported function and answers with plain data. */
import { parentPort } from "node:worker_threads";

parentPort.on("message", async (message) => {
  if (message?.type !== "run") return;
  const { id } = message;
  try {
    const url = new URL(message.module);
    if (url.protocol !== "file:") throw new Error("File-addressed worker task required");
    const task = (await import(message.module))[message.export];
    if (typeof task !== "function") throw new Error("Exported worker task function required");
    const value = await task(message.input, { progress: (completed, total) => parentPort.postMessage({ type: "progress", id, completed, total }) });
    parentPort.postMessage({ type: "result", id, value });
  } catch (error) {
    parentPort.postMessage({ type: "error", id, name: String(error?.name ?? "Error"), message: String(error?.message ?? error) });
  }
});
