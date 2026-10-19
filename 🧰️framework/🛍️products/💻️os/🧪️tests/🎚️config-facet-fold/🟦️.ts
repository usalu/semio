/** 🎚️ LAW: the store worker is the only place an OS config facet (the local document catalog, the identity record) changes. A
 * shell sends mutation envelopes whose `diff.payload` is the facet's sparse diff; the worker folds each through the twin of the
 * central applier onto the value its folder holds, persists the resulting archive, and hands every client that archive as the
 * projection to render. A whole-archive put on a facet is refused, a payload that is not the facet's diff is refused, and an
 * envelope that arrives before the folder's first answer waits for it instead of overwriting what the folder holds.
 * The envelopes are the shell's own (`localCatalogMutationEnvelopeV1`) and cross the real wire codec before the worker sees them. */
import { decodeBackboneWorkerRequest, decodeDocumentArchiveBytes, decodePackValue, encodeBackboneWorkerRequest, encodeDocumentArchiveBytes, type ArtifactActorConfig, type BackboneWorkerRequest, type BackboneWorkerResponse } from "../../🟦️.ts";
import { localCatalogMutationEnvelopeV1, decodeLocalCatalogArchiveV1 } from "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗂️local-catalog/🟦️.ts";
import { admitLocalDocument, identityMutationDiffPair, LOCAL_CATALOG_CONFIG_SCHEMA, retireLocalDocument, signIn, signOut, type Identity, type LocalDocument } from "@semio-tech/framework/config/schema/mutations";
import { decodeIdentityValue } from "@semio-tech/framework/config/schema/mutations";

type Folder = Map<string, Uint8Array>;
const IDENTITY_SCHEMA = "os.config.identity";

const alpha: LocalDocument = { documentId: "studio-alpha", schema: "s.space", name: "Alpha Studio", storage: "folder", target: "/data/os/local-documents/studio-alpha", admittedAtMs: 1_790_590_000_000 };
const beta: LocalDocument = { documentId: "studio-beta", schema: "s.space", name: "Beta Studio", storage: "file", target: "/data/Documents/beta.os", admittedAtMs: 1_790_500_000_000 };
const grace: Identity = { userId: "grace", email: "grace@studio.example", displayName: "Grace Hopper", hubBaseUrl: "https://hub.studio.example", issuedAtMs: 1_756_286_400_000 };

export async function registerConfigFacetFoldTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  worker: {
    readonly testSeams: { workerPostTestSink: ((message: BackboneWorkerResponse) => void) | null };
    readonly artifactState: (documentId: string, spaceId?: string) => { readonly pendingMutations: readonly unknown[]; readonly revalidateFolder: () => Promise<void> } | undefined;
    readonly closeArtifact: (documentId: string, spaceId: string | undefined, clientInstanceId: string) => void;
    readonly handleTsRequest: (request: BackboneWorkerRequest) => void;
    readonly installStreamMuxEndpoint: (endpoint: null) => void;
    readonly configFacetArchiveV1: (value: unknown) => Uint8Array;
  },
): Promise<void> {
  const { describe, it, expect } = vitest;
  const binding = { kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/config-facet-fold" } as const;
  const open = (documentId: string, clientInstanceId: string): ArtifactActorConfig & { readonly kind: "open"; readonly clientInstanceId: string } =>
    ({ kind: "open", actorIdentityGrant: { maximumItems: 1024, maximumCopyBytes: 65536, maximumCapacityBytes: 65536, maximumReleaseBytes: 65536, maximumDepth: 64 }, clientInstanceId, documentId, schema: documentId, bindings: [binding], watchExternal: false, actor: "actor-1" });
  const wire = (request: BackboneWorkerRequest): BackboneWorkerRequest => decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request));
  const settle = async (done: () => boolean): Promise<void> => {
    for (let turn = 0; turn < 400 && !done(); turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
    expect(done()).toBe(true);
  };
  const harness = () => {
    const folder: Folder = new Map();
    const held: { seed: Uint8Array | null } = { seed: null };
    const counts = { reads: 0, writes: 0 };
    const gate: { release: () => void; held: Promise<void> | null } = { release: () => {}, held: null };
    const fetchStub = async (url: string, init?: RequestInit) => {
      if (init?.method === "PUT") {
        counts.writes += 1;
        folder.set(url, new Uint8Array(init.body as Uint8Array));
        return new Response(null, { status: 200 });
      }
      counts.reads += 1;
      if (gate.held !== null) await gate.held;
      const bytes = folder.get(url) ?? held.seed;
      return bytes === null ? new Response(null, { status: 204 }) : new Response(bytes.slice(), { status: 200 });
    };
    const posted: BackboneWorkerResponse[] = [];
    const events = (kind: string) => posted.flatMap((message) => (message.kind === "event" && message.event.kind === kind ? [message.event] : []));
    const original = globalThis.fetch;
    (globalThis as unknown as { fetch: unknown }).fetch = fetchStub;
    worker.installStreamMuxEndpoint(null);
    worker.testSeams.workerPostTestSink = (message) => posted.push(message);
    const hold = () => {
      gate.held = new Promise<void>((resolve) => {
        gate.release = () => {
          gate.held = null;
          resolve();
        };
      });
    };
    const restore = () => {
      worker.testSeams.workerPostTestSink = null;
      (globalThis as unknown as { fetch: unknown }).fetch = original;
    };
    const only = () => {
      const [bytes] = [...folder.values()];
      return bytes;
    };
    return { folder, held, counts, posted, events, hold, release: () => gate.release(), restore, only };
  };

  describe("config facet fold", () => {
    it("folds a catalog admission and retirement through the twin applier, persists the archive and answers with it as the projection", async () => {
      const h = harness();
      const client = crypto.randomUUID();
      try {
        worker.handleTsRequest(wire(open(LOCAL_CATALOG_CONFIG_SCHEMA, client)));
        await settle(() => h.counts.reads === 1 && h.events("documentArchiveAbsent").length === 1);
        const admit = localCatalogMutationEnvelopeV1("actor-1", admitLocalDocument(alpha), { documents: [] });
        expect(admit.diff.payload).toEqual({ documents: [{ key: "studio-alpha", value: alpha }] });
        worker.handleTsRequest(wire({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId: client, message: { kind: "localMutations", envelopes: [admit] } }));
        await settle(() => h.counts.writes === 1 && worker.artifactState(LOCAL_CATALOG_CONFIG_SCHEMA)?.pendingMutations.length === 0);
        expect(decodeLocalCatalogArchiveV1(h.only()!)).toEqual({ documents: [alpha] });
        const replaced = h.events("documentArchiveReplaced") as { readonly archive: readonly number[] }[];
        expect(replaced).toHaveLength(1);
        expect(decodeLocalCatalogArchiveV1(Uint8Array.from(replaced[0]!.archive))).toEqual({ documents: [alpha] });

        const second = localCatalogMutationEnvelopeV1("actor-1", admitLocalDocument(beta), { documents: [alpha] });
        const retire = localCatalogMutationEnvelopeV1("actor-1", retireLocalDocument("studio-alpha"), { documents: [alpha, beta] });
        expect(retire.diff.payload).toEqual({ documents: [{ key: "studio-alpha" }] });
        expect(retire.inverse.inverseDiff.payload).toEqual({ documents: [{ key: "studio-alpha", value: alpha }] });
        worker.handleTsRequest(wire({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId: client, message: { kind: "localMutations", envelopes: [second, retire] } }));
        await settle(() => h.counts.writes === 2);
        expect(decodeLocalCatalogArchiveV1(h.only()!)).toEqual({ documents: [beta] });
        expect(h.events("commandOutcome")).toEqual([]);
      } finally {
        worker.closeArtifact(LOCAL_CATALOG_CONFIG_SCHEMA, undefined, client);
        h.restore();
      }
    });

    it("holds an envelope that arrives before the folder's first answer and folds it onto what the folder holds", async () => {
      const h = harness();
      const client = crypto.randomUUID();
      try {
        h.held.seed = worker.configFacetArchiveV1({ documents: [beta] });
        h.hold();
        worker.handleTsRequest(wire(open(LOCAL_CATALOG_CONFIG_SCHEMA, client)));
        await settle(() => h.counts.reads === 1);
        const admit = localCatalogMutationEnvelopeV1("actor-1", admitLocalDocument(alpha), { documents: [] });
        worker.handleTsRequest(wire({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId: client, message: { kind: "localMutations", envelopes: [admit] } }));
        await new Promise((resolve) => setTimeout(resolve, 20));
        expect(h.counts.writes).toBe(0);
        h.release();
        await settle(() => h.counts.writes === 1);
        expect(decodeLocalCatalogArchiveV1(h.only()!)).toEqual({ documents: [alpha, beta] });
      } finally {
        worker.closeArtifact(LOCAL_CATALOG_CONFIG_SCHEMA, undefined, client);
        h.restore();
      }
    });

    it("refuses a whole-archive put and a payload that is not the facet's diff, and leaves the folder as it was", async () => {
      const h = harness();
      const client = crypto.randomUUID();
      try {
        worker.handleTsRequest(wire(open(LOCAL_CATALOG_CONFIG_SCHEMA, client)));
        await settle(() => h.events("documentArchiveAbsent").length === 1);
        const archive = encodeDocumentArchiveBytes({ parent_pack: Array.from(worker.configFacetArchiveV1({ documents: [alpha] })), parent_spr: [], members: [] });
        worker.handleTsRequest(wire({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId: client, message: { kind: "localDocumentArchive", archive: Array.from(archive) } }));
        const malformed = { ...localCatalogMutationEnvelopeV1("actor-1", admitLocalDocument(alpha), { documents: [] }), diff: { schemaId: LOCAL_CATALOG_CONFIG_SCHEMA, payload: { documents: "everything" } } };
        worker.handleTsRequest(wire({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId: client, message: { kind: "localMutations", envelopes: [malformed] } }));
        await settle(() => h.events("commandOutcome").length === 2);
        for (const outcome of h.events("commandOutcome") as { readonly outcome: { readonly kind: string; readonly code: string } }[]) expect(outcome.outcome).toMatchObject({ kind: "rejected", code: "local.backbone-malformed" });
        expect(h.counts.writes).toBe(0);
        expect(h.folder.size).toBe(0);
        expect(h.events("documentArchiveReplaced")).toEqual([]);
      } finally {
        worker.closeArtifact(LOCAL_CATALOG_CONFIG_SCHEMA, undefined, client);
        h.restore();
      }
    });

    it("folds sign-in and sign-out of the identity facet and restores the persisted session on the next open", async () => {
      const h = harness();
      const first = crypto.randomUUID();
      const second = crypto.randomUUID();
      const envelope = (mutation: ReturnType<typeof signIn>, base: Identity | null) => {
        const { diff, inverse } = identityMutationDiffPair(mutation, base);
        return { id: `identity-${Math.random()}`, actor: "actor-1", document: IDENTITY_SCHEMA, schemaVersion: IDENTITY_SCHEMA, payloadHash: "", diff: { schemaId: IDENTITY_SCHEMA, payload: diff }, inverse: { targetOperation: mutation.mutation, inverseDiff: { schemaId: IDENTITY_SCHEMA, payload: inverse }, baseVersion: 0, undoPolicy: "exactBaseOnly" as const } };
      };
      const sessionOf = (bytes: Uint8Array): Identity | null | undefined => decodeIdentityValue(decodePackValue(new Uint8Array(decodeDocumentArchiveBytes(bytes).parent_pack)));
      try {
        worker.handleTsRequest(wire(open(IDENTITY_SCHEMA, first)));
        await settle(() => h.events("documentArchiveAbsent").length === 1);
        worker.handleTsRequest(wire({ kind: "send", documentId: IDENTITY_SCHEMA, clientInstanceId: first, message: { kind: "localMutations", envelopes: [envelope(signIn(grace), null)] } }));
        await settle(() => h.counts.writes === 1);
        expect(sessionOf(h.only()!)).toEqual(grace);
        worker.handleTsRequest(wire({ kind: "send", documentId: IDENTITY_SCHEMA, clientInstanceId: first, message: { kind: "localMutations", envelopes: [envelope(signIn(grace), grace)] } }));
        worker.handleTsRequest(wire({ kind: "send", documentId: IDENTITY_SCHEMA, clientInstanceId: first, message: { kind: "localMutations", envelopes: [envelope(signOut(), grace)] } }));
        await settle(() => h.counts.writes === 2);
        expect(sessionOf(h.only()!)).toBeNull();
        worker.closeArtifact(IDENTITY_SCHEMA, undefined, first);

        h.posted.length = 0;
        worker.handleTsRequest(wire(open(IDENTITY_SCHEMA, second)));
        await settle(() => h.events("documentArchiveReplaced").length === 1);
        const replaced = h.events("documentArchiveReplaced")[0] as { readonly archive: readonly number[] };
        expect(sessionOf(Uint8Array.from(replaced.archive))).toBeNull();
      } finally {
        worker.closeArtifact(IDENTITY_SCHEMA, undefined, second);
        worker.closeArtifact(IDENTITY_SCHEMA, undefined, first);
        h.restore();
      }
    });
  });
}
