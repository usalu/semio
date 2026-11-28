import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { defaultKillWriterV1, ownedKillLedgerWriterV1, ownedKillRecorderV1 } from "../../../../../../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { readyKillRowsV1, readyStopSenderV1 } from "../../🌲️process-tree/🟦️.ts";

/** 🔗️ One record format: what the owned executions and the reaper append to `kills.log` is read by the stop-sender reader like the stage runner's own rows, and a stage that ends by one of those signals names its sender.
 * @see ../../🌲️process-tree/🟦️.ts (`readyKillRowsV1`) */
test("the rows of owned execution are read back with victim, signal, sender and label, and name the sender of a stage", () => {
  const directory = mkdtempSync(join(tmpdir(), "one-record-")), file = join(directory, "kills.log"), at = Date.parse("2026-11-27T14:00:00.000Z");
  try {
    const record = ownedKillRecorderV1("shared cargo build", () => "the last requester a left the shared child k", ownedKillLedgerWriterV1(file, () => at));
    record({ pid: 4242, startMs: 1767225600000 }, "SIGTERM");
    record({ pid: 4243 }, "SIGKILL");
    ownedKillLedgerWriterV1(file, () => at)("garbage that is no record");
    const rows = readyKillRowsV1(readFileSync(file, "utf8"), 0);
    expect(rows.map((row) => ({ pid: row.pid, signal: row.signal, stage: row.stage, killer: row.killer }))).toEqual([
      { pid: 4242, signal: "SIGTERM", stage: "shared cargo build", killer: process.pid },
      { pid: 4243, signal: "SIGKILL", stage: "shared cargo build", killer: process.pid },
    ]);
    expect(rows[0]!.atMs).toBe(at);
    const sender = readyStopSenderV1({ ended: "SIGTERM", stagePid: 4242, endedMs: at + 1000, observer: process.pid, kills: rows });
    expect(sender.kind).toBe("runner");
    expect(sender.line).toContain(String(process.pid));
    expect(readyKillRowsV1(readFileSync(file, "utf8"), at + 1)).toEqual([]);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test("the default writer appends to the ledger the environment names and to nothing else", () => {
  const directory = mkdtempSync(join(tmpdir(), "one-record-")), file = join(directory, "kills.log");
  try {
    const stderr = process.stderr, write = stderr.write.bind(stderr);
    stderr.write = (() => true) as typeof stderr.write;
    try {
      defaultKillWriterV1("line one", { SEMIO_KILL_LEDGER: file });
      defaultKillWriterV1("line two", {});
    } finally { stderr.write = write; }
    expect(readFileSync(file, "utf8").split("\n").filter(Boolean).map((line) => line.replace(/^\S+ /u, ""))).toEqual(["line one"]);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
