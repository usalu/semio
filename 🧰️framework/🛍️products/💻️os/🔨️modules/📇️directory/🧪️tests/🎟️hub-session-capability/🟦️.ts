/** 🎟️ The hub session capability lane: the human's own bearer on every hub call, refusal without a capability and the private session port. */
import type { BackboneWorkerTestDependencies } from "../../../🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerHubSessionCapabilityTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, installHubSessionCapability, hubSessionFetch, clearHubSessionCapability } = dependencies;
  describe("hub session capability lane", () => {
    beforeEach(resetWorkerLaw);
    const CAPABILITY_ONE = `session.v1.${"a".repeat(32)}.${"1".repeat(64)}`;
    const CAPABILITY_TWO = `session.v1.${"b".repeat(32)}.${"2".repeat(64)}`;

    it("sends the human's own bearer on every hub call and never any other credential", async () => {
      const originalFetch = globalThis.fetch;
      const requests: Array<{ url: string; headers: Headers }> = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: unknown, init?: RequestInit) => {
        requests.push({ url: String(input), headers: new Headers(init?.headers) });
        return new Response("{}", { status: 200 });
      };
      try {
        installHubSessionCapability(CAPABILITY_ONE);
        await hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 });
        await hubSessionFetch("/directory/spaces", { method: "GET" }, { timeoutMs: 1_000 });
        expect(requests.map((request) => request.url)).toEqual(["/_semio/hub/auth/sessions/me", "/_semio/hub/directory/spaces"]);
        for (const request of requests) {
          expect(request.headers.get("authorization")).toBe(`Bearer ${CAPABILITY_ONE}`);
          expect(request.headers.get("x-semio-browser-broker")).toBeNull();
          expect(request.headers.get("x-semio-browser-broker-next")).toBeNull();
        }
      } finally {
        clearHubSessionCapability();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("refuses every hub call without a capability, and drops the capability the hub answers 401 for", async () => {
      const originalFetch = globalThis.fetch;
      let calls = 0;
      try {
        clearHubSessionCapability();
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          calls += 1;
          return new Response("{}", { status: 200 });
        };
        await expect(hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("hub session rebootstrap required");
        expect(calls).toBe(0);

        installHubSessionCapability(CAPABILITY_ONE);
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          calls += 1;
          return new Response("unauthorized", { status: 401 });
        };
        await expect(hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("hub session rebootstrap required");
        expect(calls).toBe(1);
        await expect(hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("hub session rebootstrap required");
        expect(calls).toBe(1);

        installHubSessionCapability(CAPABILITY_TWO);
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          calls += 1;
          throw new Error("transport detail must be redacted");
        };
        await expect(hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("hub session unreachable");
        expect(calls).toBe(2);
        await hubSessionFetchSucceeds();
      } finally {
        clearHubSessionCapability();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }

      async function hubSessionFetchSucceeds(): Promise<void> {
        (globalThis as unknown as { fetch: unknown }).fetch = async () => new Response("{}", { status: 200 });
        const response = await hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 });
        expect(response.status).toBe(200);
      }
    });

    it("refuses a malformed capability, is idempotent for the same one, and bounds its own queue", async () => {
      const originalFetch = globalThis.fetch;
      const originalQueued = testSeams.hubSessionQueued;
      let sentAuthorization = "";
      (globalThis as unknown as { fetch: unknown }).fetch = async (_input: unknown, init?: RequestInit) => {
        sentAuthorization = new Headers(init?.headers).get("authorization") ?? "";
        return new Response("{}", { status: 200 });
      };
      try {
        clearHubSessionCapability();
        for (const rejected of ["", "session.v1", "4".repeat(64), `session.v1.${"a".repeat(32)}.${"1".repeat(63)}`, `Bearer session.v1.${"a".repeat(32)}.${"1".repeat(64)}`]) {
          expect(installHubSessionCapability(rejected)).toBe(false);
        }
        await expect(hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("hub session rebootstrap required");

        expect(installHubSessionCapability(CAPABILITY_ONE)).toBe(true);
        expect(installHubSessionCapability(CAPABILITY_ONE)).toBe(true);
        await hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 });
        expect(sentAuthorization).toBe(`Bearer ${CAPABILITY_ONE}`);

        testSeams.hubSessionQueued = 64;
        await expect(hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("hub session capacity exceeded");
        expect(sentAuthorization).toBe(`Bearer ${CAPABILITY_ONE}`);
      } finally {
        testSeams.hubSessionQueued = originalQueued;
        clearHubSessionCapability();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("keeps the private session port out of malicious plugin shard source, and leaves no broker-proof lane behind", async () => {
      const { readFile } = await import("node:fs/promises");
      const materialization = ["🟦️.ts", "🧾️contract/🟦️.ts", "🔤️guestslim-fonts/🟦️.ts", "🩹️preview2-shim-patches/🟦️.ts", "🧵️shard-worker-source/🟦️.ts", "🌉️component-bridge-source/🟦️.ts", "🪶️wasm-opt/🟦️.ts", "🧬️jco-module-rewrites/🟦️.ts", "🪢️host-shim-source/🟦️.ts"];
      const pluginShard = (await Promise.all(materialization.map((leaf) => readFile(new URL(`./🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/${leaf}`, source.url), "utf8")))).join("\n");
      const shellHostFiles = [
        "🟦️.tsx",
        "⚡️host-effects/🟦️.ts",
        "✅️check-in-sync/🟦️.tsx",
        "🎚️shell-model/🟦️.tsx",
        "🎥️tutorial-orchestration/🟦️.ts",
        "🎨️theme-driver-settings/🟦️.tsx",
        "🎬️tutorial-recorder/🟦️.tsx",
        "🎮️focus-and-input/🟦️.tsx",
        "🏁️operation-settle/🟦️.ts",
        "🏗️dock-assembly/🟦️.ts",
        "🏠️space-administration/🟦️.ts",
        "📋️panel-hosting/🟦️.tsx",
        "📐️layout-hotkeys/🟦️.ts",
        "📤️document-transfer-tabs/🟦️.tsx",
        "📨️window-invocation/🟦️.ts",
        "📬️artifact-opening/🟦️.ts",
        "🔁️contributions-refresh/🟦️.ts",
        "🔌️extension-runtime/🟦️.ts",
        "🔎️command-bar/🟦️.tsx",
        "🔐️identity-binding/🟦️.ts",
        "🔒️identity-document-ports/🟦️.ts",
        "🖱️shell-actions/🟦️.ts",
        "🖼️canvas-footer/🟦️.tsx",
        "🗄️shell-uri-documents/🟦️.ts",
        "🚀️plugin-lifecycle/🟦️.ts",
        "🛂️directory-route/🟦️.ts",
        "🛫️spawned-session/🟦️.ts",
        "🧩️shell-core-state/🟦️.ts",
        "🧱️built-node-stores/🟦️.ts",
        "🧵️backbone-worker/🟦️.ts",
        "🧾️space-artifact-creation/🟦️.ts",
      ];
      const shellHost = (await Promise.all(shellHostFiles.map((file) => readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/" + file, source.url), "utf8")))).join("\n");
      const worker = (await import("@semio-tech/framework-compiler-ts/syntax/module-tree")).readModuleTreeSource(new URL("./🔨️modules/🏪️store/👷️worker/🟦️.ts", source.url));
      expect(pluginShard).not.toContain("semio-hub-session-port");
      expect(pluginShard).not.toContain("authorization");
      for (const retired of ["semio-browser-broker-port", "x-semio-browser-broker", "#semio-broker=", "localBrowserBrokerProof"]) {
        expect(shellHost).not.toContain(retired);
        expect(worker).not.toContain(retired);
      }
    });
  });
}
