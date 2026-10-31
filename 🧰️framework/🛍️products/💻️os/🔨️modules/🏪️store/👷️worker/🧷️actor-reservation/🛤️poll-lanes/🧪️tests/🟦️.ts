import { expect, test } from "bun:test";
import { ReservationPollLanes } from "../🟦️.ts";

/** 🧪️ Spawned-job drive laws of the poll lane: concurrent jobs, progress, cancellation of whatever a drive leaves running. */
const running = { tag: "running", val: undefined };
const done = (value: number[]) => ({ tag: "done", val: value });

function fixture(script: (job: bigint, step: number) => unknown) {
  const session = { pollTail: Promise.resolve(), closed: false, child: null as unknown, abort: new AbortController() };
  const log: string[] = [];
  const counts = new Map<bigint, number>();
  const child = {
    invoke: async ([owner, name]: string[], args: unknown[]) => {
      const job = args[0] as bigint;
      log.push(`${owner}.${name}:${job}`);
      if (name !== "stepJob") return undefined;
      const step = counts.get(job) ?? 0;
      counts.set(job, step + 1);
      await Promise.resolve();
      return script(job, step);
    },
  };
  session.child = child;
  const lanes = new ReservationPollLanes({ session } as never);
  return { lanes, session, child: child as never, log };
}

const spawn = (job: bigint) => ({ job, kind: "k", input: new Uint8Array() });

test("independent jobs interleave step by step and complete in spawn order", async () => {
  const { lanes, child, log } = fixture((job, step) => (step === 2 ? done([Number(job)]) : running));
  const events = await lanes.driveSpawnedJobs(child, [spawn(1n), spawn(2n)], () => {});
  expect(events.map((event) => (event as never as { val: { job: bigint } }).val.job)).toEqual([1n, 2n]);
  expect(log.filter((entry) => entry.includes("stepJob"))).toEqual(["jobs.stepJob:1", "jobs.stepJob:2", "jobs.stepJob:1", "jobs.stepJob:2", "jobs.stepJob:1", "jobs.stepJob:2"]);
  expect(log.some((entry) => entry.includes("cancelJob"))).toBe(false);
});

test("every observed step is reported as progress", async () => {
  const { lanes, child } = fixture((_job, step) => (step === 1 ? done([1]) : running));
  const seen: string[] = [];
  await lanes.driveSpawnedJobs(child, [spawn(7n)], () => {}, (step) => seen.push(`${step.job}:${step.index}:${step.status}`));
  expect(seen).toEqual(["7:0:running", "7:1:done"]);
});

test("a session abort stops the drive and cancels the job it leaves running", async () => {
  const { lanes, child, log, session } = fixture((_job, step) => {
    if (step === 1) session.abort.abort();
    return running;
  });
  await expect(lanes.driveSpawnedJobs(child, [spawn(3n)], () => {})).rejects.toThrow();
  expect(log).toEqual(["jobs.startJob:3", "jobs.stepJob:3", "jobs.stepJob:3", "jobs.cancelJob:3"]);
});

test("the step ceiling cancels a job that never settles", async () => {
  const { lanes, child, log } = fixture(() => running);
  await expect(lanes.driveSpawnedJobs(child, [spawn(4n)], () => {})).rejects.toThrow();
  expect(log.at(-1)).toBe("jobs.cancelJob:4");
});

test("a failing drive cancels the sibling already running and the first failure is the one thrown", async () => {
  const { lanes, child, log } = fixture(() => running);
  let calls = 0;
  const failing = () => { if (++calls === 3) throw new Error("current-owner-lost"); };
  await expect(lanes.driveSpawnedJobs(child, [spawn(5n), spawn(6n)], failing)).rejects.toThrow("current-owner-lost");
  expect(log).toContain("jobs.startJob:5");
  expect(log.filter((entry) => entry.includes("cancelJob"))).toEqual(["jobs.cancelJob:5"]);
  expect(log).not.toContain("jobs.startJob:6");
});
