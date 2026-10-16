/** 🧭️ Browser actor command windows: an app command is admitted in every window kind of the verified app. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerActorCommandWindowTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams } = dependencies;
  describe("browser actor command windows", () => {
    beforeEach(resetWorkerLaw);
    it("admits an app command in every window kind of the verified app and nowhere else", async () => {
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/🪟️browser-actor-command-windows-v1.json", source.url), "utf8")) as {
        readonly pluginId: string;
        readonly appId: string;
        readonly leaseWindow: string;
        readonly windows: readonly string[];
        readonly cases: readonly Readonly<{ name: string; address: Readonly<{ kind: "window" | "command"; appId?: string; windowKindId?: string; windowInstanceId?: string }>; view: Readonly<Record<string, string>>; admitted?: string; refused?: string }>[];
      };
      const { createBrowserActorAppCommandRequestV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/command");
      const { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION } = await import("@semio-tech/framework/browser-bundle/action-handoff");
      const fields = { package: { pluginId: corpus.pluginId }, surface: { appId: corpus.appId, windowKindId: corpus.leaseWindow } } as unknown as Parameters<typeof testSeams.browserActorAppCommandV1>[1];
      const windows = new Set(corpus.windows);
      expect(corpus.cases.filter((row) => row.admitted !== undefined).map((row) => row.admitted)).toEqual(expect.arrayContaining([...windows]));
      for (const [index, row] of corpus.cases.entries()) {
        const invocation = row.address.kind === "window"
          ? { address: { pluginId: corpus.pluginId, appId: row.address.appId ?? corpus.appId, modeId: "edit", windowKindId: row.address.windowKindId!, windowInstanceId: row.address.windowInstanceId!, actionId: "demoShellCommand" }, arguments: { commandId: "shell.windowActivate" } }
          : { address: { owner: { app: { pluginId: corpus.pluginId, appId: corpus.appId } }, commandId: "demoShellCommand" }, arguments: { commandId: "shell.windowActivate" } };
        const request = createBrowserActorAppCommandRequestV1(
          { scope: { spaceId: "space-a", documentId: "artifact-a" }, verifiedSurfaceId: "s.demo.demo@1/*#editor", appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION, activationGeneration: "1", instanceId: 0, surfaceRevision: 1, actionSequence: index + 1 },
          invocation as Parameters<typeof createBrowserActorAppCommandRequestV1>[1],
          row.view as Parameters<typeof createBrowserActorAppCommandRequestV1>[2],
        );
        if (row.refused !== undefined) expect(() => testSeams.browserActorAppCommandV1(request, fields, windows), row.name).toThrow(row.refused);
        else expect(testSeams.browserActorAppCommandV1(request, fields, windows).surfaceKey, row.name).toBe(row.admitted);
      }
    });
  });
}
