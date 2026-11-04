import { describe, expect, it } from "vitest";
import { replayOpenArtifactV1, type OpenArtifactReplayPortV1 } from "../../🧱️elements/🏛️ShellHost/⚡️host-effects/🎯️replay-shell-command-effect/🚪️open-artifact/🟦️.ts";
import { runArtifactCreationReadyOpeningV1 } from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { REPLAY_REFUSAL_LABELS_V1 } from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts";

function port(overrides: Partial<OpenArtifactReplayPortV1<string>> = {}) {
  const log: string[] = [];
  const base: OpenArtifactReplayPortV1<string> = {
    resolve: async () => "opening",
    open: async (opening) => { log.push(`open:${opening}`); return "opened"; },
    refuse: (reason) => { log.push(`refuse:${reason}`); },
  };
  return { log, port: { ...base, ...overrides } };
}

describe("replayOpenArtifactV1", () => {
  it("opens the resolved artifact", async () => {
    const { log, port: p } = port();
    await replayOpenArtifactV1(p);
    expect(log).toEqual(["open:opening"]);
  });

  it("refuses with router-not-ready when nothing resolves", async () => {
    const { log, port: p } = port({ resolve: async () => null });
    await replayOpenArtifactV1(p);
    expect(log).toEqual(["refuse:router-not-ready"]);
  });

  it("refuses visibly when opening failed, install failure included", async () => {
    const { log, port: p } = port({ open: async () => "failed" });
    await replayOpenArtifactV1(p);
    expect(log).toEqual(["refuse:open-rejected"]);
  });

  it("stays silent when the effect owner retired", async () => {
    const { log, port: p } = port({ open: async () => "retired" });
    await replayOpenArtifactV1(p);
    expect(log).toEqual([]);
  });

  it("refuses when opening throws", async () => {
    const { log, port: p } = port({ open: async () => { throw new Error("boom"); } });
    await replayOpenArtifactV1(p);
    expect(log).toEqual(["refuse:open-rejected"]);
  });

  it("only uses reasons the shared refusal vocabulary words in en and de", () => {
    for (const reason of ["router-not-ready", "open-rejected"] as const) {
      expect(REPLAY_REFUSAL_LABELS_V1[reason].en.length).toBeGreaterThan(0);
      expect(REPLAY_REFUSAL_LABELS_V1[reason].de.length).toBeGreaterThan(0);
    }
  });
});

describe("publish-after-open ordering shared with creation", () => {
  const run = (open: () => Promise<{ committed: boolean } | null>, prepare: () => Promise<string | null> = async () => "target") => {
    const log: string[] = [];
    return runArtifactCreationReadyOpeningV1<string, { committed: boolean }>({
      current: () => true,
      prepare: async () => { const t = await prepare(); if (t) log.push("prepare"); return t; },
      open: async () => { log.push("open"); return open(); },
      publish: () => { log.push("publish"); },
      release: async () => { log.push("release"); },
      failed: () => { log.push("failed"); },
    }).then((outcome) => ({ outcome, log }));
  };

  it("publishes only after the document opened", async () => {
    expect(await run(async () => ({ committed: true }))).toEqual({ outcome: "opened", log: ["prepare", "open", "publish"] });
  });

  it("destroys the prepared instance and never publishes when the document fails", async () => {
    expect(await run(async () => { throw new Error("x"); })).toEqual({ outcome: "failed", log: ["prepare", "open", "release", "failed"] });
  });

  it("reports failure when the app cannot be prepared", async () => {
    expect((await run(async () => ({ committed: true }), async () => null)).outcome).toBe("failed");
  });
});
