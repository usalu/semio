/** 🧪️ Catalog boot and issued-profile requests agree with independent URL and JSON Schema oracles. */
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { playgroundCatalog } from "@semio-tech/framework-plugin/build/plan";
import { timeTravelServeVariantV1, timeTravelServeRequestV1, type TimeTravelServeSelectionV1, type TimeTravelPlaygroundV1 } from "../🟦️.ts";
import { playgroundActivationTargetV1 } from "../../../../♻️activation/🎯️target/🟦️.ts";
import { ensureDevServe, devServeCommandV1 } from "../../../../🚀️local-hub/🏃️execution/🟦️.ts";
import schema from "../../../../🧬️schema/🔣️.json";
const { JSDOM } = createRequire(import.meta.url)("jsdom");
const read = (relative: string) => JSON.parse(readFileSync(fileURLToPath(new URL(relative, import.meta.url)), "utf8"));
const catalog = playgroundCatalog();
const [scenarioRow, familyRow] = catalog;
const tokens: Record<string, string> = { $scenario: scenarioRow!.variant, $family: familyRow!.variant, $familyWgpuPort: String(familyRow!.ports.wgpu) };
const resolved = (value: unknown) => JSON.parse(JSON.stringify(value).replace(/\$(?:scenario|familyWgpuPort|family)/gu, (token) => tokens[token]!));
const fixture = resolved(read("../🧫️fixtures/🔣️.json"));

function selectedCatalog(variant: string, composition: string | undefined): readonly TimeTravelPlaygroundV1[] {
  if (composition === undefined || composition === "catalog") return catalog;
  const values: Record<string, string | undefined> = {
    missing: undefined, empty: "", control: "bad\npath",
    oversized: "x".repeat(schema.$defs.DevServeSpawnRequestV1.properties.compositionConfigPath.maxLength + 1),
  };
  if (!(composition in values)) throw Error("Unknown authored composition case");
  return catalog.map(row => row.variant === variant ? {...row, compositionConfigPath: values[composition]} : row);
}

describe("universal history serve boot", () => {
  for (const law of fixture.cases) it(law.name, () => {
    const selection = law as TimeTravelServeSelectionV1;
    if (law.error) expect(() => timeTravelServeVariantV1(selection, catalog)).toThrow();
    else {
      const variant = timeTravelServeVariantV1(selection, catalog);
      expect(variant).toBe(law.expected);
      expect(catalog.some(row => row.variant === variant)).toBe(true);
      const dom = new JSDOM();
      const url = new dom.window.URL(law.url);
      const requested = law.explicit ?? url.searchParams.get("plugin");
      const oracle = !law.universal ? law.scenarioVariant : requested ? catalog.find(row => row.variant === requested || row.aliases.includes(requested))?.variant : catalog.find(row => row.ports[law.renderer as "react" | "wgpu"] === Number(url.port))?.variant;
      expect(variant).toBe(oracle);
      dom.window.close();
    }
  });
});

describe("catalog history serve request", () => {
  const ajv = new Ajv({strict:true,allErrors:true});
  ajv.addSchema(schema);
  const validate = ajv.getSchema(schema.$id + "#/$defs/DevServeSpawnRequestV1")!;
  for (const law of fixture.requests) it(law.name, () => {
    const admitted = selectedCatalog(scenarioRow!.variant, law.composition);
    const row = admitted.find(row => row.variant === scenarioRow!.variant)!;
    const selection = {universal:true,url:`http://127.0.0.1:6901/?plugin=${row.variant}`,renderer:law.renderer,profile:law.profile,port:6901,logPath:"fixture/serve.log"};
    const oracle = {variant:row.variant,renderer:law.renderer,profile:law.profile,port:6901,hubUrl:null,logPath:"fixture/serve.log",...(row.compositionConfigPath === undefined ? {} : {compositionConfigPath:row.compositionConfigPath})};
    expect(validate(oracle)).toBe(law.accepted);
    if (!law.accepted) expect(() => timeTravelServeRequestV1(selection, admitted)).toThrow();
    else {
      const request = timeTravelServeRequestV1(selection, admitted);
      expect(request).toEqual(oracle);
      const command = devServeCommandV1(request);
      expect(command.args).toContain(law.profile);
      if (law.renderer === "wgpu") expect(command.args.slice(-2)).toEqual(["--composition",row.compositionConfigPath]);
    }
  });
});

describe("owned history journey startup", () => {
  for (const law of fixture.startups) it(law.name, async () => {
    const controller = new AbortController();
    if (law.cancel === "before") controller.abort();
    const trace: string[] = [];
    let spawned = false;
    let activated = false;
    const admitted = selectedCatalog(law.variant, law.composition);
    const result = await (async () => {
      const request = timeTravelServeRequestV1({universal:true,url:`http://127.0.0.1:${law.port}/?plugin=${law.variant}`,renderer:law.renderer,profile:law.profile,port:law.port,logPath:"fixture/serve.log"}, admitted);
      return ensureDevServe({
        ...request, hubUrl: request.hubUrl ?? undefined, repoRoot: "fixture", signal: controller.signal,
        beforeSpawn: async () => {
          trace.push(playgroundActivationTargetV1(request.variant, request.profile, request.renderer, catalog));
          activated = true;
          if (law.activationFails) throw Error("activation refused");
          if (law.cancel === "during") controller.abort();
        },
        world: {
          answers: async () => law.answers || Boolean(activated && law.afterActivationAnswers) || spawned,
          portInUse: async () => law.occupied || Boolean(activated && law.afterActivationOccupied) || spawned,
          spawnServe: actual => {
            expect(actual.variant).toBe(request.variant);
            expect(actual.renderer).toBe(request.renderer);
            expect(actual.profile).toBe(request.profile);
            expect(actual.compositionConfigPath).toBe(request.compositionConfigPath);
            const command = devServeCommandV1(actual);
            expect(command.args).toContain(request.profile);
            if (actual.renderer === "wgpu") expect(command.args.slice(-2)).toEqual(["--composition", request.compositionConfigPath]);
            trace.push("spawn"); spawned = true; return { pid: 17, exited: () => false };
          },
          terminate: () => { trace.push("stop"); spawned = false; },
          now: () => 0, sleep: async () => undefined,
        },
      });
    })().catch(() => null);
    expect(result === null ? "error" : result.reused ? "reused" : "ready").toBe(law.outcome);
    await result?.stop();
    expect(JSON.parse(JSON.stringify(trace))).toEqual(law.trace);
  });
});
