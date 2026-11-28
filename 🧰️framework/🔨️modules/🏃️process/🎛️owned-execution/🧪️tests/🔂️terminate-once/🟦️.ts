import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { runOwnedCommand } from "../../🟦️.ts";

/** 🔂️ Termination law of an owned command: however many times its output sink fails, the process tree is stopped once (one wave of signals), and the default kill writer also appends to the ledger the environment names (`SEMIO_KILL_LEDGER`). A sink that fails on every write used to start a new process-table read and a new wave per error, which starved the event loop until nothing was reaped.
 * @see ../../../🪓️termination/🧬️schema/🔣️.json */
test.skipIf(process.platform === "win32")("a sink that fails on every write stops the command once and the ledger holds that one wave", async () => {
  const directory = mkdtempSync(join(tmpdir(), "terminate-once-")), ledger = join(directory, "kills.log"), previous = process.env.SEMIO_KILL_LEDGER;
  process.env.SEMIO_KILL_LEDGER = ledger;
  const sink = process.stdout, write = sink.write.bind(sink);
  sink.write = ((chunk: string | Uint8Array, ...rest: unknown[]) => {
    if (String(chunk).includes("flood-line")) { queueMicrotask(() => sink.emit("error", Error("output-sink"))); return true; }
    return (write as (...values: unknown[]) => boolean)(chunk, ...rest);
  }) as typeof sink.write;
  const started = Date.now();
  try {
    await expect(runOwnedCommand(process.execPath, ["-e", 'const{writeSync}=await import("node:fs");for(;;){writeSync(1,"flood-line\\n");await new Promise(done=>setImmediate(done));}'], process.cwd(), "flood", 60000)).rejects.toBeDefined();
    expect(Date.now() - started, "the stop must not starve the loop").toBeLessThan(20000);
    const lines = readFileSync(ledger, "utf8").split("\n").filter(Boolean), terms = lines.filter((line) => line.includes("received SIGTERM"));
    expect(terms.length).toBeGreaterThanOrEqual(1);
    expect(terms.length, "one wave, not one wave per error").toBeLessThanOrEqual(3);
    expect(terms[0]).toMatch(/^\d{4}-\d\d-\d\dT[\d:.]+Z Process \d+ received SIGTERM from owned execution flood \(killing process \d+\): output delivery refused/u);
    expect(terms[0]).toContain(" / Prozess ");
  } finally {
    sink.write = write;
    if (previous === undefined) delete process.env.SEMIO_KILL_LEDGER; else process.env.SEMIO_KILL_LEDGER = previous;
    rmSync(directory, { recursive: true, force: true });
  }
}, 40000);
