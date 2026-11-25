import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { runOwnedCommand } from "../../🟦️.ts";
import { ProgressDeadlineExpiredV1 } from "../../../⏱️budget/🟦️.ts";
import { ResourceLoanDeadlineExhaustedV1 } from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type { ResourceLeaseControl } from "../../../🔒️leases/🟦️.ts";

type Row = Readonly<{ name: string; idleMilliseconds: number; lineEveryMilliseconds: number; lines: number; renewOnLine: boolean; expected: string }>;
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🔣️.json"), "utf8")) as Readonly<{ cases: readonly Row[] }>;

/** ⏳️ A caller control whose idle window is renewed by the observer's progress, as a renewable invocation control is. */
function renewableControl(row: Row, signal: AbortSignal): Readonly<{ control: ResourceLeaseControl; renew: () => void }> {
  let progressed = performance.now();
  const control: ResourceLeaseControl = {
    signal,
    remainingMilliseconds: () => {
      const left = row.idleMilliseconds - (performance.now() - progressed);
      if (left <= 0) {
        if (row.expected === "execution.progress-expired") throw new ProgressDeadlineExpiredV1("idle", "no progress for the idle window / kein Fortschritt im Leerlauffenster");
        return 0;
      }
      return left;
    },
    publish: async () => {},
    yieldContinuation: async () => {},
  };
  return { control, renew: () => { if (row.renewOnLine) progressed = performance.now(); } };
}

for (const row of fixture.cases) {
  test(`owned command deadline: ${row.name}`, async () => {
    const source = row.lines === 0
      ? "setTimeout(() => {}, 5000)"
      : `let n=0;const t=setInterval(()=>{console.error("progress "+(++n));if(n>=${row.lines}){clearInterval(t);}},${row.lineEveryMilliseconds})`;
    const { control, renew } = renewableControl(row, new AbortController().signal);
    const run = runOwnedCommand(process.execPath, ["-e", source], process.cwd(), "deadline-fixture", 0, { deadline: control, stdout: "ignore", onLine: renew });
    if (row.expected === "completed") await run;
    else await expect(run).rejects.toThrow(new RegExp(row.expected.replace(/\./gu, "\\.")));
  }, 20_000);
}

test("owned command deadline: cancellation still wins over a live deadline", async () => {
  const controller = new AbortController();
  const row: Row = { name: "cancel", idleMilliseconds: 60_000, lineEveryMilliseconds: 0, lines: 0, renewOnLine: true, expected: "cancelled" };
  const { control } = renewableControl(row, controller.signal);
  setTimeout(() => controller.abort(), 300);
  await expect(runOwnedCommand(process.execPath, ["-e", "setTimeout(() => {}, 5000)"], process.cwd(), "deadline-cancel", 0, { deadline: control, signal: controller.signal, stdout: "ignore" })).rejects.toThrow(/cancelled/);
});

test("owned command deadline: a caller signal ended by an exhausted loan stops the command with that deadline, never as a cancellation", async () => {
  for (const delay of [0, 300]) {
    const controller = new AbortController(), reason = new ResourceLoanDeadlineExhaustedV1("Original physical body cutoff reached");
    if (delay === 0) controller.abort(reason); else setTimeout(() => controller.abort(reason), delay);
    const failure = await runOwnedCommand(process.execPath, ["-e", "setTimeout(() => {}, 5000)"], process.cwd(), "deadline-loan", 0, { signal: controller.signal, stdout: "ignore" }).then(() => undefined, (error: unknown) => error);
    expect(failure).toBe(reason);
  }
});
