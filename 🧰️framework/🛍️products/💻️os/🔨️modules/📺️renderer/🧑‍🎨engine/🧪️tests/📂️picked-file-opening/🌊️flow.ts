import { describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import commitFixture from "./🧫️fixtures/🔣️.json";
import { openPickedFileV1, readPickedFileHeadV1, type PickedFileFlowPortV1, type PickedFileOutcomeV1 } from "../../🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts";

type Owner = { id: string };
const picked = { name: "a.png" };

function harness(overrides: Partial<PickedFileFlowPortV1<Owner, typeof picked>> = {}) {
  const log: string[] = [];
  const outcomes: PickedFileOutcomeV1[] = [];
  const abort = new AbortController();
  let slot = false;
  const port: PickedFileFlowPortV1<Owner, typeof picked> = {
    select: async () => picked,
    claim: () => { if (slot) return false; slot = true; log.push("claim"); return true; },
    release: () => { slot = false; log.push("release"); },
    track: () => ({ signal: abort.signal, progress: () => undefined, finish: () => { log.push("finish"); } }),
    route: async () => ({ kind: "routed", artifactRef: "ref", resolution: {} as never }),
    resolveOwner: async () => ({ id: "o" }),
    importInto: async () => 7,
    commit: () => true,
    destroy: async (_owner, id) => { log.push(`destroy:${id}`); },
    report: () => undefined,
    notify: (outcome) => { outcomes.push(outcome); },
    ...overrides,
  };
  return { port, log, outcomes, abort, slot: () => slot };
}

describe("openPickedFileV1", () => {
  it("opens, releases the slot and tells the person", async () => {
    const h = harness();
    await openPickedFileV1(h.port);
    expect(h.outcomes).toEqual([{ kind: "opened", name: "a.png" }]);
    expect(h.slot()).toBe(false);
    expect(h.log).not.toContain("destroy:7");
  });

  it("turns a rejecting picker into a visible failure", async () => {
    const h = harness({ select: async () => { throw new Error("picker"); } });
    await openPickedFileV1(h.port);
    expect(h.outcomes).toEqual([{ kind: "failed", name: "" }]);
  });

  it("does not claim the slot while the picker waits and stays silent when nothing is picked", async () => {
    const h = harness({ select: async () => undefined });
    await openPickedFileV1(h.port);
    expect(h.log).toEqual([]);
    expect(h.outcomes).toEqual([]);
  });

  it("reports busy when another open holds the slot", async () => {
    const h = harness();
    h.port.claim();
    await openPickedFileV1(h.port);
    expect(h.outcomes).toEqual([{ kind: "busy", name: "a.png" }]);
  });

  it("turns rejections of routing, owner resolution and import into a failure and releases", async () => {
    for (const patch of [{ route: async () => { throw new Error("head"); } }, { resolveOwner: async () => { throw new Error("owner"); } }, { importInto: async () => { throw new Error("import"); } }] as const) {
      const h = harness(patch);
      await openPickedFileV1(h.port);
      expect(h.outcomes).toEqual([{ kind: "failed", name: "a.png" }]);
      expect(h.slot()).toBe(false);
      expect(h.log).toContain("finish");
    }
  });

  it("destroys the created instance and fails visibly when the panel commit has no session", async () => {
    const h = harness({ commit: () => false });
    await openPickedFileV1(h.port);
    expect(h.log).toContain("destroy:7");
    expect(h.outcomes).toEqual([{ kind: "failed", name: "a.png" }]);
  });

  it("reports a distinct cancelled outcome when the task was aborted", async () => {
    const h = harness({ importInto: async () => { h.abort.abort(); throw new Error("aborted"); } });
    await openPickedFileV1(h.port);
    expect(h.outcomes).toEqual([{ kind: "cancelled", name: "a.png" }]);
  });

  it("names an unresolvable owner and a refused route", async () => {
    expect((await (async () => { const h = harness({ resolveOwner: async () => null }); await openPickedFileV1(h.port); return h.outcomes; })())).toEqual([{ kind: "refused", reason: "owner-unavailable", name: "a.png" }]);
    const h = harness({ route: async () => ({ kind: "refused", reason: "format-unresolved" }) });
    await openPickedFileV1(h.port);
    expect(h.outcomes).toEqual([{ kind: "refused", reason: "format-unresolved", name: "a.png" }]);
  });
});

describe("durable picked-file commit", () => {
  it("validates neutral outcomes independently", () => { expect(new Ajv().validate(commitFixture.schema, commitFixture.cases)).toBe(true); });
  for (const row of commitFixture.cases) it(row.name, async () => {
    let settle!: (value: boolean) => void;
    const commit = vi.fn((_owner: Owner, _id: number, task: { signal: AbortSignal }) => {
      expect(task.signal).toBe(h.abort.signal);
      return new Promise<boolean>(resolve => { settle = resolve; });
    });
    const h = harness({ commit });
    const pending = openPickedFileV1(h.port);
    await vi.waitFor(() => expect(commit).toHaveBeenCalledTimes(1));
    expect(h.outcomes).toEqual([]);
    expect(h.slot()).toBe(true);
    if (row.commit === "cancelled") h.abort.abort(new Error("cancelled"));
    else settle(row.commit === "accepted");
    await pending;
    expect(h.outcomes).toEqual([{ kind: row.outcome, name: "a.png" }]);
    expect(h.log.includes("destroy:7")).toBe(row.destroy);
    expect(h.slot()).toBe(false);
    if (row.commit === "cancelled") { settle(true); await Promise.resolve(); expect(h.outcomes).toEqual([{ kind: "cancelled", name: "a.png" }]); }
  });
  it("releases a created instance when commit rejects", async () => {
    const h = harness({ commit: async () => { throw new Error("storage refused"); } });
    await openPickedFileV1(h.port);
    expect(h.log).toContain("destroy:7");
    expect(h.outcomes).toEqual([{ kind: "failed", name: "a.png" }]);
  });
  it("does not commit or report opened when import returns after cancellation", async () => {
    const commit = vi.fn(() => true);
    const h = harness({ commit, importInto: async () => { h.abort.abort(); return 7; } });
    await openPickedFileV1(h.port);
    expect(commit).not.toHaveBeenCalled();
    expect(h.log).toContain("destroy:7");
    expect(h.outcomes).toEqual([{ kind: "cancelled", name: "a.png" }]);
  });
});

describe("readPickedFileHeadV1", () => {
  it("reads the leading bytes", async () => {
    const bytes = Uint8Array.from([1, 2, 3, 4]);
    const blob = { slice: (start: number, end: number) => ({ arrayBuffer: async () => bytes.slice(start, end).buffer }) } as unknown as Blob;
    expect([...await readPickedFileHeadV1(blob, 2, new AbortController().signal)]).toEqual([1, 2]);
  });
  it("settles at once when cancelled, even while the read is pending", async () => {
    const abort = new AbortController();
    const never = { slice: () => ({ arrayBuffer: () => new Promise<ArrayBuffer>(() => undefined) }) } as unknown as Blob;
    const pending = readPickedFileHeadV1(never, 4, abort.signal);
    abort.abort(new Error("cancelled"));
    await expect(pending).rejects.toThrow("cancelled");
  });
});
