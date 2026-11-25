/** 🧩️ Package components: the language-neutral cases (`🧫️fixtures/🔣️.json`, shared with the Rust builder's registration law) decide the
 * component every activation loads, and every descriptor a host refuses; Ajv, a third-party validator, gives the same shape verdict on the
 * descriptor admission schema's `components` member as the repository's own subset validator.
 * @see ../🟦️.ts
 * @see ../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/📜️registration/🧪️tests/🦀️.rs */
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { OwnedResidentLedger } from "@semio-tech/value-ts/resident";
import { ShardClient, type ShardBudget } from "@semio-tech/framework-actor/shard-client";
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import cases from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import admission from "../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { ActivationRegistry } from "../../🎛️activation-registry/🟦️.ts";
import { descriptorPackageComponents, fetchDescriptorManifest, fetchPackageDescriptor } from "../../🛂️descriptor-admission/🟦️.ts";
import { PackageComponentError, packageComponentForApp, readPackageComponents } from "../🟦️.ts";

const BUDGET: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16, maxFrames: 8 };
const accepted = cases.registrations.filter(row => row.components !== undefined);
const shards = accepted.find(row => Array.isArray(row.components))!;

/** 📮️ A shard client whose one worker records every message and answers every request. */
function capturingShardClient(posted: { kind: string; component?: string | null; actorId?: string }[]): ShardClient {
  const createWorker = () => {
    const worker: { postMessage: (message: unknown) => void; terminate: () => void; onmessage: ((event: { readonly data: unknown }) => void) | null; onerror: ((event: unknown) => void) | null } = {
      postMessage: message => {
        posted.push(message as never);
        const requestId = (message as { readonly requestId?: string }).requestId;
        if (requestId) queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value: undefined } }));
      },
      terminate: () => {},
      onmessage: null,
      onerror: null,
    };
    return worker;
  };
  return new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker });
}

/** 🗂️ The app roster a registration case registers, in order. */
function roster(row: (typeof cases.registrations)[number]): string[] {
  return row.steps.flatMap(step => step.apps);
}

/** 🧾️ The descriptor a registration case describes: the `components` member only when the case declares components. */
function describedOf(row: (typeof cases.registrations)[number]): object {
  return row.components === null ? {} : { components: row.components };
}

describe("package components", () => {
  it("validates its language-neutral cases against their schema with Ajv", () => {
    const ajv = new Ajv({ allErrors: true, strict: false });
    const valid = ajv.validate(schema, cases);
    expect(ajv.errors ?? []).toEqual([]);
    expect(valid).toBe(true);
  });

  it("loads the component that owns each activation's app, and the first for an activation of no app", () => {
    for (const row of accepted) {
      const components = readPackageComponents(describedOf(row), roster(row));
      expect(components?.declarations ?? null, row.id).toEqual(row.components);
      for (const activation of row.activations!) expect(packageComponentForApp(components, activation.app), `${row.id}: ${activation.app}`).toBe(activation.component);
    }
  });

  it("refuses an activation of an app no component serves", () => {
    const components = readPackageComponents(describedOf(shards), roster(shards));
    expect(() => packageComponentForApp(components, "s.stdio.pdf@2.0/*#editor")).toThrow("package-components.unknown-app");
  });

  it("refuses every descriptor whose components do not partition its roster, with the same shape verdict as Ajv on the admission schema", () => {
    const ajv = new Ajv({ allErrors: true, strict: false });
    const member = (admission as { $defs: { PackageDescriptorV1: { properties: { components: object } } } }).$defs.PackageDescriptorV1.properties.components;
    const shape = ajv.compile(member);
    for (const row of cases.descriptors) {
      let refusal: unknown;
      try { readPackageComponents({ components: row.components }, row.roster); } catch (error) { refusal = error; }
      expect(refusal, row.id).toBeInstanceOf(PackageComponentError);
      expect((refusal as PackageComponentError).code, row.id).toBe(row.code);
      expect(shape(row.components), row.id).toBe(row.shapeValid);
      expect(validateJsonSchemaSubset(member, row.components, admission).length === 0, row.id).toBe(row.shapeValid);
    }
    for (const row of accepted.filter(candidate => candidate.components !== null)) {
      expect(shape(row.components), row.id).toBe(true);
      expect(validateJsonSchemaSubset(member, row.components, admission), row.id).toEqual([]);
    }
  });

  it("activates, suspends and resumes each actor on the component of its app; no app runs on the first", async () => {
    const posted: { kind: string; component?: string | null; actorId?: string }[] = [];
    const registry = new ActivationRegistry({ shardClient: capturingShardClient(posted), defaultBudget: BUDGET, fetchAssets: async () => [], fetchPackageAssets: async () => [] });
    const components = readPackageComponents(describedOf(shards), roster(shards))!;
    registry.registerManifest({ pluginId: "stdio-pdf", moduleUrl: "https://x/stdio-pdf/bridge.js", caps: [], components });
    registry.registerManifest({ pluginId: "note", moduleUrl: "https://x/note/bridge.js", caps: [] });
    const activations = shards.activations!;
    for (const [index, activation] of activations.entries()) await registry.activate("stdio-pdf", `stdio-pdf#${index}`, "manual", activation.app);
    await registry.activate("note", "note#0", "manual", "s.note.note@1/*#editor");
    const activated = () => posted.filter(message => message.kind === "activate");
    expect(activated().map(message => [message.actorId, message.component])).toEqual([...activations.map((activation, index) => [`stdio-pdf#${index}`, activation.component]), ["note#0", null]]);
    await registry.suspend("stdio-pdf#0");
    await registry.resume("stdio-pdf#0");
    expect(activated().at(-1)).toMatchObject({ actorId: "stdio-pdf#0", component: activations[0]!.component });
    await expect(registry.activate("stdio-pdf", "stdio-pdf#lost", "manual", "s.stdio.pdf@2.0/*#editor")).rejects.toThrow("package-components.unknown-app");
  });

  it("hands the components a served descriptor declares to both loaders, and refuses a descriptor whose components do not partition its apps", async () => {
    const original = globalThis.fetch;
    const apps = roster(shards).map(id => ({ id }));
    const serve = (document: object) => { globalThis.fetch = Object.assign(async () => new Response(JSON.stringify(document), { headers: { "content-type": "application/json" } }), { preconnect: () => {} }) as typeof fetch; };
    try {
      serve({ manifest: { pluginId: "stdio-pdf", apps }, components: shards.components });
      const manifest = await fetchDescriptorManifest("stdio-pdf", "http://page.test/modules/stdio-pdf/bridge.js");
      expect(descriptorPackageComponents(manifest)?.declarations).toEqual(shards.components);
      serve({ manifest: { pluginId: "stdio-pdf", apps }, packageId: "semio:stdio-pdf", components: shards.components });
      const admitted = await fetchPackageDescriptor("stdio-pdf", "http://page.test/modules/stdio-pdf/bridge.js", { read: async () => ({ version: 1, pluginId: "stdio-pdf", packageId: "semio:stdio-pdf", componentSha256: "0".repeat(64) }) });
      expect(admitted.components?.declarations).toEqual(shards.components);
      serve({ manifest: { pluginId: "stdio-pdf", apps } });
      expect(descriptorPackageComponents(await fetchDescriptorManifest("stdio-pdf", "http://page.test/modules/stdio-pdf/bridge.js"))).toBeUndefined();
      serve({ manifest: { pluginId: "stdio-pdf", apps }, components: shards.components!.slice(1) });
      await expect(fetchDescriptorManifest("stdio-pdf", "http://page.test/modules/stdio-pdf/bridge.js")).rejects.toThrow("package-components.unowned-app");
    } finally { globalThis.fetch = original; }
  });
});
