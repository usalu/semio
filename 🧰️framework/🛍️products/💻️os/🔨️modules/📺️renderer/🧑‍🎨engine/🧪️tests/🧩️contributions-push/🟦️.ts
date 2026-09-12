/** 🧩️ The host→guest contributions push as an OWNED unit, driven by the language-neutral fixture
 * `🛠️ShellHelpers/🧫️fixtures/🧩️contributions-push/🔣️.json`.
 *
 * 🏁️ The defect these laws encode was measured live at `http://127.0.0.1:6018/?plugin=generation3d`
 * on 2026-09-12: 45 s of console with ZERO `[DEBUG] contributions …` lines and a preview stuck at
 * `flow.extension-not-contributed`. The push lived inside `ShellHost.refreshUi`, which bumps a
 * refresh generation on every call and abandons itself when that generation moves under an await —
 * and the push's own await was a guest document read. A guest re-arming a faulting `flowEvalTick`
 * settles into a refresh every few seconds, so every push was superseded inside its document read
 * and the closure never crossed (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * ⚖️ The laws are about the UNIT, not about a widened race window: a superseded refresh must not
 * abort a resolving push, overlapping refreshes must join one run, and exactly one
 * `setContributions` must cross per `(instanceId, content)`. */

import { describe, expect, it } from "vitest";
import { createContributionsPublisher, type ContributionsOperatorScope, type ContributionsPublishOutcome, type ContributionsSessionKey } from "../../🧱️elements/🛠️ShellHelpers/🧩️contributions/🟦️.ts";
import fixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧩️contributions-push/🔣️.json";

type Scenario = (typeof fixture.scenarios)[number] & {
  readonly retireInstanceId?: number;
  readonly nextInstanceId?: number;
  readonly unresolvedReason?: string;
};

type Environment = { readonly generation: string };

/** 🧪️ One run of the unit against a document read the test opens and closes by hand, so "superseded
 * mid-read" is a state the law reaches deterministically instead of racing for. */
function harness(scenario: Scenario) {
  const reads: number[] = [];
  const pushes: { readonly instanceId: number; readonly json: string }[] = [];
  let releaseRead: (() => void) | undefined;
  const readGate = new Promise<void>((resolve) => {
    releaseRead = resolve;
  });
  const publisher = createContributionsPublisher<Environment>({
    registryGeneration: (environment) => environment.generation,
    resolveScope: async (session): Promise<ContributionsOperatorScope> => {
      reads.push(session.instanceId);
      if (scenario.documentReadResolves !== "immediately") await readGate;
      if (scenario.documentReadResolves === "unresolved") return { status: "unresolved", reason: scenario.unresolvedReason ?? "no-document-pack" };
      return { status: "resolved", kinds: fixture.scopeKinds };
    },
    buildPack: () => fixture.pack,
    install: async (session, json) => {
      pushes.push({ instanceId: session.instanceId, json });
    },
  });
  return { publisher, reads, pushes, releaseRead: () => releaseRead?.() };
}

const sessionKey = (instanceId: number): ContributionsSessionKey => ({ pluginId: fixture.session.pluginId, instanceId });
const environment: Environment = { generation: "boot" };
const installedKeyOf = (entry: { readonly instanceId: number; readonly json: string }): string => `${entry.instanceId}::PACK`;

describe("contributions push unit", () => {
  for (const scenario of fixture.scenarios as readonly Scenario[]) {
    it(scenario.id, async () => {
      const { publisher, reads, pushes, releaseRead } = harness(scenario);
      const outcomes: ContributionsPublishOutcome[] = [];
      if (scenario.supersedeEveryRefresh) {
        // 🔁️ Every refresh starts while the previous one is still inside its document read and is
        // then superseded — the shape `refreshUi` produced on every settle of the faulting guest.
        const runs = Array.from({ length: scenario.refreshes }, () => publisher.publish(sessionKey(fixture.session.instanceId), environment));
        releaseRead();
        outcomes.push(...(await Promise.all(runs)));
      } else if (scenario.documentReadResolves === "after-retire") {
        const first = publisher.publish(sessionKey(fixture.session.instanceId), environment);
        publisher.retire(scenario.retireInstanceId!);
        releaseRead();
        outcomes.push(await first);
        outcomes.push(await publisher.publish(sessionKey(scenario.nextInstanceId!), environment));
      } else {
        for (let index = 0; index < scenario.refreshes; index++) outcomes.push(await publisher.publish(sessionKey(fixture.session.instanceId), environment));
      }
      expect(reads.length, `document reads for ${scenario.id}`).toBe(scenario.expected.documentReads);
      expect(pushes.length, `setContributions crossings for ${scenario.id}`).toBe(scenario.expected.pushes);
      expect(pushes.map(installedKeyOf)).toEqual(scenario.expected.installedKeys);
      expect(outcomes.map((outcome) => outcome.status)).toEqual(scenario.expected.outcomes);
      for (const push of pushes) expect(push.json).toBe(fixture.pack);
    });
  }

  it("re-runs the unit when the loaded registry generation moves", async () => {
    const { publisher, pushes, releaseRead } = harness({ ...(fixture.scenarios[1] as Scenario), documentReadResolves: "immediately" });
    releaseRead();
    expect((await publisher.publish(sessionKey(1), { generation: "boot" })).status).toBe("installed");
    expect((await publisher.publish(sessionKey(1), { generation: "boot" })).status).toBe("unchanged");
    // 🔢 A plugin load or an extension toggle is a new closure: the unit re-runs and the content key
    // decides again — the pack is unchanged here, so nothing crosses twice.
    expect((await publisher.publish(sessionKey(1), { generation: "one-plugin-more" })).status).toBe("unchanged");
    expect(pushes.length).toBe(1);
    expect(publisher.installedKey()).toBe(`1::${fixture.pack}`);
  });

  it("restores the installed key when the guest crossing throws, so a failure can be retried", async () => {
    let attempts = 0;
    const publisher = createContributionsPublisher<Environment>({
      registryGeneration: (environment) => environment.generation,
      resolveScope: async () => ({ status: "resolved", kinds: fixture.scopeKinds }),
      buildPack: () => fixture.pack,
      install: async () => {
        attempts += 1;
        if (attempts === 1) throw new Error("guest instance busy");
      },
    });
    expect((await publisher.publish(sessionKey(3), environment)).status).toBe("failed");
    expect(publisher.installedKey()).toBe(null);
    expect((await publisher.publish(sessionKey(3), environment)).status).toBe("installed");
    expect(attempts).toBe(2);
  });
});
