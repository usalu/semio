/** 🔁️ GIS browser document peers refetch the same exact pair after a scoped rebootstrap. */
import { workerLawFakesV1, bytesFromHex, fixtureRequiredFrontier } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { TestSource } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../🧰️dependencies/🟦️.ts";

type BackboneWorkerResponse = import("@semio-tech/framework-os").BackboneWorkerResponse;
type DocumentScope = import("@semio-tech/framework/directory/schema").DocumentScope;
type PersistenceBinding = import("@semio-tech/framework-os").PersistenceBinding;
type UiNodeRecord = import("@semio-tech/framework/manifest").UiNodeRecord;
type WireFrontierSummary = import("@semio-tech/framework-replication").WireFrontierSummary;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerGisPeerRebootstrapTests(vitest: Vitest, dependencies: ServiceCompositionTestDependenciesV1, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw, artifactBootstrapFixture, decodeFixtureFrame, installFixture } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, artifacts, documentRuntimeKeyForConfig, handleHubFrame, encodePackValue, closeArtifactRuntime } = dependencies;
  //#region 🧪️ArtifactBootstrapRestore
  describe("artifact bootstrap atomic restore", () => {

    beforeEach(resetWorkerLaw);

    it("browser document peers refetch the same exact pair after scoped rebootstrap", async () => {
      const { readFile } = await import("node:fs/promises");
      type GisMapPeerRebootstrapFixtureV1 = Readonly<{
        schema: "semio.os.gis-map-peer-rebootstrap/v1";
        scope: DocumentScope;
        clients: readonly Readonly<{ clientInstanceId: string }>[];
        published: Readonly<{
          checkpointId: string;
          descriptorDigestV1: string;
          aggregateSha256: string;
          frontier: Readonly<{ documentId: string; headEditOrdinal: number; headEditId: string; lastCommitSeq: number; chainHash: readonly number[] }>;
          scene: Readonly<{ revision: number; nodeKind: "tiled-map"; region: Readonly<{ id: string; kind: "inference-bounds" }> }>;
        }>;
        order: readonly string[];
        sourceHostiles: readonly string[];
        nonclaims: readonly string[];
      }>;
      const parsed: unknown = JSON.parse(await readFile(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/🗺️gis-map-peer-rebootstrap-v1/🔣️.json", import.meta.url), "utf8"));
      const schema = JSON.parse(await readFile(new URL("../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🔣️.json", import.meta.url), "utf8")) as { $id: string };
      
      
      
      const { default: Ajv } = await import("ajv");
      const ref = (name: string) => ({ $ref: `${schema.$id}#/$defs/${name}` });
      const strings = { type: "array", items: { type: "string" } };
      const admit = new Ajv({ strict: false }).addSchema(schema).compile<GisMapPeerRebootstrapFixtureV1>({
        type: "object",
        required: ["schema", "scope", "clients", "published", "order", "sourceHostiles", "nonclaims"],
        properties: {
          schema: { const: "semio.os.gis-map-peer-rebootstrap/v1" },
          scope: ref("GisMapPeerRebootstrapScope"),
          clients: { type: "array", items: { type: "object", required: ["clientInstanceId"], properties: { clientInstanceId: { type: "string" } } } },
          published: {
            type: "object",
            required: ["checkpointId", "descriptorDigestV1", "aggregateSha256", "frontier", "scene"],
            properties: {
              checkpointId: ref("GisMapPeerRebootstrapHash"),
              descriptorDigestV1: ref("GisMapPeerRebootstrapHash"),
              aggregateSha256: ref("GisMapPeerRebootstrapHash"),
              frontier: ref("GisMapPeerRebootstrapFrontier"),
              scene: {
                type: "object",
                required: ["revision", "nodeKind", "region"],
                properties: {
                  revision: { type: "integer" },
                  nodeKind: { const: "tiled-map" },
                  region: { type: "object", required: ["id", "kind"], properties: { id: { type: "string" }, kind: { const: "inference-bounds" } } },
                },
              },
            },
          },
          order: strings,
          sourceHostiles: strings,
          nonclaims: strings,
        },
      });
      if (!admit(parsed)) throw new Error("peer rebootstrap fixture violates its schema");
      const corpus = parsed;
      const fixture = await artifactBootstrapFixture();
      const welcome = decodeFixtureFrame(fixture.wire.inlineWelcomeHex);
      if (!("Welcome" in welcome) || typeof welcome.Welcome.bootstrap !== "object" || !("ArtifactBootstrap" in welcome.Welcome.bootstrap)) throw new Error("peer bootstrap fixture");
      const bootstrap = welcome.Welcome.bootstrap.ArtifactBootstrap;
      const { UiDocumentStore } = await import("@semio-tech/framework-os/modules/renderer/engine/elements/UiDocumentStore");
      const priorPost = testSeams.workerPostTestSink;
      const results: { clientInstanceId: string; pack: number[]; spr: number[]; frontier: WireFrontierSummary; node: unknown; closeCount: number }[] = [];
      try {
        for (const client of corpus.clients) {
          const state = await installFixture(fixture, false);
          artifacts.delete(state.runtimeKey);
          const binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> = { kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: corpus.scope.spaceId, requestedSurfaceId: "s.gis.gismap@1/*/viewer" };
          state.config = { ...state.config, documentId: corpus.scope.documentId, bindings: [binding] };
          state.runtimeKey = documentRuntimeKeyForConfig(state.config);
          state.openClientInstanceId = client.clientInstanceId;
          artifacts.set(state.runtimeKey, state);
          let closeCount = 0;
          const staleSocket = { close: () => closeCount++ } as unknown as WebSocket;
          state.socket = staleSocket;
          const posted: BackboneWorkerResponse[] = [];
          testSeams.workerPostTestSink = (message) => posted.push(message);
          await handleHubFrame(
            state,
            {
              RebootstrapRequired: {
                control: {
                  space_id: corpus.scope.spaceId,
                  document_id: corpus.scope.documentId,
                  checkpoint_id: [...bytesFromHex(corpus.published.checkpointId)],
                  descriptor_hash: [...bootstrap.descriptor_hash],
                  baseline_frontier: { ...bootstrap.baseline_frontier, document_id: corpus.scope.documentId },
                },
              },
            },
            null,
            staleSocket,
          );
          expect(state.currentPack).toBeNull();
          expect(state.currentSpr).toBeNull();
          expect(state.frontier).toBeNull();
          expect(state.resumeToken).toBeNull();
          expect(closeCount).toBe(1);
          expect(posted.filter((message) => message.kind === "artifact-rebootstrap-required")).toEqual([
            { kind: "artifact-rebootstrap-required", documentId: corpus.scope.documentId, clientInstanceId: client.clientInstanceId, scope: corpus.scope, message: "rebootstrap-required", retryable: true },
          ]);
          const freshSocket = { close: () => closeCount++ } as unknown as WebSocket;
          state.socket = freshSocket;
          await handleHubFrame(state, welcome, null, freshSocket);
          const frontier = { ...fixtureRequiredFrontier(fixture), document_id: corpus.scope.documentId };
          await handleHubFrame(state, { Commands: { envelopes: [], origin: state.config.actor, frontier } }, null, freshSocket);
          expect(state.status.remote.kind).toBe("live");
          const store = new UiDocumentStore("gis-map-window");
          const node: UiNodeRecord = {
            id: 0,
            key: "map-root",
            component: { type: "surface", kind: corpus.published.scene.nodeKind, docSchema: "tiled-map@1", doc: { bytes: Array.from(encodePackValue({ regions: [corpus.published.scene.region] })) }, bindings: [] },
            layout: { kind: "leaf", width: "fill", height: "fill" },
            style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
            activity: "idle",
            disabled: false,
            transition: null,
            accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
            bindings: [],
            menu: null,
            children: [],
          };
          expect(store.applyPatch({ surface: "gis-map-window", baseRevision: 0, revision: corpus.published.scene.revision, ops: [{ type: "upsert", ...node }, { type: "setRoot", id: 0 }]})).toEqual({ ok: true });
          results.push({ clientInstanceId: client.clientInstanceId, pack: [...state.currentPack!], spr: [...state.currentSpr!], frontier: state.frontier!, node: store.getNodeSnapshot(0), closeCount });
          closeArtifactRuntime(state.runtimeKey);
        }
      } finally {
        testSeams.workerPostTestSink = priorPost;
      }
      expect(results).toHaveLength(2);
      expect(results[0]!.pack).toEqual(results[1]!.pack);
      expect(results[0]!.spr).toEqual(results[1]!.spr);
      expect(results[0]!.frontier).toEqual(results[1]!.frontier);
      expect(results[0]!.node).toEqual(results[1]!.node);
      expect(results.map(({ clientInstanceId, closeCount }) => ({ clientInstanceId, closeCount }))).toEqual(corpus.clients.map((client) => ({ clientInstanceId: client.clientInstanceId, closeCount: 1 })));
      console.log("gis-map-peer-rebootstrap: clients=2 production-bootstrap=1 scene=controlled simultaneous=0");
    });

  });
  //#endregion 🧪️ArtifactBootstrapRestore
}
