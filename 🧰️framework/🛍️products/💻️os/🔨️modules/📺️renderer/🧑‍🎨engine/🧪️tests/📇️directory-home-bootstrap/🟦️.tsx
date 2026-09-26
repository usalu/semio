import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import Ajv from "ajv";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SemioFaultError, type AppDefinition, type Fault } from "@semio-tech/framework";
import type { ViewModel } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import type { BackboneWorkerRequest, BackboneWorkerResponse } from "@semio-tech/framework-os";
import type { PluginOperationCompletion, PluginWasmHandle } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import {
  DIRECTORY_PROJECTION_RECEIPT_SCHEMA,
  DirectoryBootstrapStatusNotice,
  applyDirectoryEventPageBootstrapV1,
  closeDirectoryHomeOwnerV1,
  directoryHomeOwnerAppV1,
  openDirectoryHomeOwnerV1,
  parseDirectoryProjectionReceiptV1,
  startedDirectoryOperationIdV1,
} from "../../🧱️elements/🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx";
import directorySchema from "../../../../📇️directory/🧬️schema/🔣️.json" with { type: "json" };
import fixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/📇️directory-bootstrap/🔣️.json";

afterEach(cleanup);

const app = {
  id: "s.space.home@1/*#editor",
  controllerId: "home",
  defaultModeId: "explore",
  modes: [{ id: "explore" }],
  windowKinds: [{ id: "main", actions: [{ id: "applyDirectoryEventPage" }] }],
} as unknown as AppDefinition;

const page = {
  kind: "directory-event-page",
  canonicalJson: "{\"schema\":\"semio.directory.event-page.v1\"}",
  bootstrapEpoch: 3,
  sessionBindingSha256: fixture.receipt.sessionBindingSha256,
  authorizationGeneration: fixture.receipt.authorizationGeneration,
  afterSeqExclusive: 0,
  throughSeqInclusive: fixture.receipt.throughSeqInclusive,
  hasMore: false,
  receiptSha256: fixture.receipt.receiptSha256,
} as Extract<BackboneWorkerResponse, { readonly kind: "directory-event-page" }>;

/** 🎟️ What the migrated guest really answers a dispatched `applyDirectoryEventPage` with: the typed
 * operation's `{ operationId, generation }` handle, both decimal strings, and NOT the verb's result. */
const ADMISSION_OPERATION = 64;
const ADMISSION = { operationId: String(ADMISSION_OPERATION), generation: "0" };

function terminal(output: unknown) {
  return { output, mutations: [], inverseGroup: { invocationId: "fixture", mutations: [], inverseMutations: [] } };
}

function fault(code: string, retryable: boolean): SemioFaultError {
  return new SemioFaultError({ origin: "plugin", code, severity: "error", message: code, scope: {}, retryable } as unknown as Fault);
}

/** ⏭️ Drains the microtask queue so an awaited `handleAction` and the settle registration behind it
 * have both run — the ordering this whole lane is about. */
async function flush(): Promise<void> {
  for (let tick = 0; tick < 8; tick += 1) await Promise.resolve();
}

type HomeHandleV1 = Readonly<{
  plugin: PluginWasmHandle;
  publish(completion: Readonly<{ operation?: number; terminalOutput: unknown }>): void;
  subscribers(): number;
}>;

/** 🏠️ A Home handle that behaves like the job-routed guest: `handleAction` answers the admission and
 * the receipt arrives later, only through the operation-completion subscription. */
function handle(
  options: Readonly<{ admission?: unknown; refuse?: unknown; calls?: string[]; viewStates?: ViewModel[] }> = {},
): HomeHandleV1 {
  const calls = options.calls ?? [];
  const viewStates = options.viewStates ?? [];
  const listeners = new Set<(completion: PluginOperationCompletion) => void>();
  const plugin = {
    pluginId: "space",
    manifest: { pluginId: "space", label: "Space", version: "1", apps: [app], examples: [] },
    createApp: async () => { calls.push("create"); return 41; },
    destroyApp: async () => { calls.push("destroy"); },
    handleAction: async (_instance: number, invocation: string, viewState: ViewModel) => {
      const actionId = JSON.parse(invocation).address.actionId as string;
      calls.push(`action:${actionId}:${invocation}`);
      viewStates.push(structuredClone(viewState));
      if (options.refuse !== undefined) throw options.refuse;
      return terminal("admission" in options ? options.admission : ADMISSION);
    },
    subscribeOperationCompletions: (instanceId: number, listener: (completion: PluginOperationCompletion) => void) => {
      calls.push(`subscribe:${instanceId}`);
      listeners.add(listener);
      return () => { listeners.delete(listener); };
    },
  } as unknown as PluginWasmHandle;
  return {
    plugin,
    publish: (completion) => {
      for (const listener of [...listeners]) {
        listener({
          instanceId: 41,
          operation: completion.operation ?? ADMISSION_OPERATION,
          revision: 1n,
          uiScope: undefined,
          historyPatch: undefined,
          requestedEffects: [],
          terminalOutput: completion.terminalOutput,
        });
      }
    },
    subscribers: () => listeners.size,
  };
}

async function ownerFor(
  plugin: PluginWasmHandle,
  posts: BackboneWorkerRequest[],
  options: Readonly<{ identity?: typeof fixture.identities.a; instance?: NonNullable<Parameters<typeof openDirectoryHomeOwnerV1>[0]["instance"]>; signal?: AbortSignal; beforeBootstrap?(): Promise<void> }> = {},
) {
  return openDirectoryHomeOwnerV1({
    plugin,
    app,
    identity: options.identity ?? fixture.identities.a,
    instance: options.instance,
    baseUrl: "https://hub.example",
    bootstrapEpoch: 3,
    locale: "de-DE",
    terminology: "native",
    signal: options.signal,
    beforeBootstrap: options.beforeBootstrap,
    post: (message) => posts.push(message),
  });
}

describe("retained visible Home directory bootstrap", () => {
  it("validates the language-neutral receipt and hostile vectors with AJV and the independent parser", () => {
    const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(directorySchema);
    const receipt = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/DirectoryProjectionReceiptV1` });
    const identity = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/DirectoryHomeIdentityV1` });
    const step = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/DirectoryHomeBootstrapStepV1` });
    const labels = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/DirectoryHomeBootstrapLabelsV1` });
    const validate = (value: typeof fixture): boolean =>
      receipt(value.receipt) && Object.values(value.identities).every((row) => identity(row)) && value.lifecycle.every((row) => step(row)) && labels(value.labels);
    expect(validate(fixture)).toBe(true);
    expect(parseDirectoryProjectionReceiptV1(fixture.receipt)).toEqual(fixture.receipt);
    for (const row of fixture.hostile) {
      const hostile = { ...structuredClone(fixture.receipt), ...row.patch };
      expect(parseDirectoryProjectionReceiptV1(hostile), row.id).toBeNull();
    }
  });

  it("reads only the typed-operation handle out of an admitting reply", () => {
    expect(startedDirectoryOperationIdV1(ADMISSION)).toBe(ADMISSION_OPERATION);
    expect(startedDirectoryOperationIdV1({ operationId: 7, generation: "0" })).toBe(7);
    expect(startedDirectoryOperationIdV1(fixture.receipt)).toBeNull();
    for (const refused of [null, undefined, "64", 64, [], { generation: "0" }, { operationId: "-1" }, { operationId: "1.5" }, { operationId: "" }, { operationId: Number.NaN }])
      expect(startedDirectoryOperationIdV1(refused), JSON.stringify(refused ?? null)).toBeNull();
  });

  it("opens the worker epoch with the exact host identity on the owned Home instance", async () => {
    const calls: string[] = [];
    const posts: BackboneWorkerRequest[] = [];
    const owner = await ownerFor(handle({ calls }).plugin, posts);
    expect(calls).toEqual(["create"]);
    expect(posts).toEqual([{ kind: "directory-bootstrap-open", baseUrl: "https://hub.example", after: 0, bootstrapEpoch: 3 }]);
    expect(owner.viewState.locale).toBe("de-DE");
    expect(owner.viewState.sessionIdentity).toEqual({ userId: "user-a", displayName: "Ada Author" });
    await closeDirectoryHomeOwnerV1(owner, (message) => posts.push(message));
    expect(calls.at(-1)).toBe("destroy");
  });

  it("binds and refreshes the same visible Home instance before opening without destroying it", async () => {
    const calls: string[] = [];
    const posts: BackboneWorkerRequest[] = [];
    const order: string[] = [];
    const plugin = handle({ calls }).plugin;
    const owner = await ownerFor(plugin, posts, {
      instance: { instanceId: 77, viewState: { activeModeId: "explore", panelJson: "visible" } },
      beforeBootstrap: async () => { order.push("refresh"); },
    });
    order.push(posts[0]?.kind ?? "missing");
    expect(owner.instanceId).toBe(77);
    expect(owner.ownsInstance).toBe(false);
    expect(calls.some((call) => call === "create")).toBe(false);
    expect(owner.viewState.sessionIdentity).toEqual({ userId: "user-a", displayName: "Ada Author" });
    expect(order).toEqual(["refresh", "directory-bootstrap-open"]);
    await closeDirectoryHomeOwnerV1(owner, (message) => posts.push(message));
    expect(calls).not.toContain("destroy");
  });

  it("suppresses an obsolete owner before replacing its host identity on the same visible instance", async () => {
    let releaseFirstRefresh!: () => void;
    const firstRefresh = new Promise<void>((resolve) => { releaseFirstRefresh = resolve; });
    const calls: string[] = [];
    const plugin = handle({ calls }).plugin;
    const firstPosts: BackboneWorkerRequest[] = [];
    const firstAbort = new AbortController();
    const first = ownerFor(plugin, firstPosts, { instance: { instanceId: 77, viewState: {} }, signal: firstAbort.signal, beforeBootstrap: () => firstRefresh });
    await Promise.resolve();
    firstAbort.abort("identity-replaced");
    releaseFirstRefresh();
    await expect(first).rejects.toThrow("directory-bootstrap.stale-owner");
    expect(firstPosts).toEqual([]);

    const secondPosts: BackboneWorkerRequest[] = [];
    const secondPlugin = handle({ calls }).plugin;
    const second = await ownerFor(secondPlugin, secondPosts, { identity: fixture.identities.b, instance: { instanceId: 77, viewState: {} } });
    expect(calls).toEqual([]);
    expect(secondPosts).toHaveLength(1);
    expect(second.identity).toEqual(fixture.identities.b);
    expect(second.viewState.sessionIdentity).toEqual({ userId: "user-b", displayName: "Bert Spectator" });
    await closeDirectoryHomeOwnerV1(second, (message) => secondPosts.push(message));
  });

  it("never ACKs the admitting reply and ACKs only the settled operation's terminal receipt", async () => {
    const posts: BackboneWorkerRequest[] = [];
    const calls: string[] = [];
    const viewStates: ViewModel[] = [];
    const home = handle({ calls, viewStates });
    const owner = await ownerFor(home.plugin, posts);
    posts.length = 0;
    const pending = applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message));
    const duplicate = await applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message));
    expect(duplicate.state).toEqual({ kind: "pending", throughSeqInclusive: 11, cancellable: true });
    expect(owner.pending).toEqual({
      canonicalJson: page.canonicalJson,
      sessionBindingSha256: page.sessionBindingSha256,
      authorizationGeneration: page.authorizationGeneration,
      receiptSha256: page.receiptSha256,
      throughSeqInclusive: page.throughSeqInclusive,
    });
    // 🧾️ The admitting reply has already resolved here — its `{ operationId, generation }` handle is
    // NOT a receipt, so nothing may be acknowledged yet. Reading it as one is the whole defect.
    await flush();
    expect(calls.filter((call) => call.startsWith("action:applyDirectoryEventPage:"))).toHaveLength(1);
    expect(posts).toEqual([]);
    // 🏁️ The subscription opened BEFORE the dispatch, or a completion racing the reply is lost.
    expect(calls.indexOf("subscribe:41")).toBeLessThan(calls.findIndex((call) => call.startsWith("action:applyDirectoryEventPage:")));
    expect(JSON.parse(calls.find((call) => call.startsWith("action:applyDirectoryEventPage:"))!.split(":").slice(2).join(":"))).toMatchObject({ address: { actionId: "applyDirectoryEventPage" }, arguments: { pageJson: page.canonicalJson } });
    expect(viewStates).toHaveLength(1);
    expect(viewStates[0]?.sessionIdentity).toEqual({ userId: "user-a", displayName: "Ada Author" });
    // 🧾️ A completion for ANOTHER operation on the same instance settles nothing.
    home.publish({ operation: ADMISSION_OPERATION + 1, terminalOutput: fixture.receipt });
    await flush();
    expect(posts).toEqual([]);
    home.publish({ terminalOutput: fixture.receipt });
    expect((await pending).receipt).toEqual(fixture.receipt);
    expect(posts).toEqual([{ kind: "directory-bootstrap-ack", bootstrapEpoch: 3, sessionBindingSha256: fixture.receipt.sessionBindingSha256, authorizationGeneration: 7, throughSeqInclusive: 11, receiptSha256: fixture.receipt.receiptSha256 }]);
    expect(home.subscribers()).toBe(0);
  });

  it("settles a completion that wins the race against its own admitting reply", async () => {
    const posts: BackboneWorkerRequest[] = [];
    const home = handle({});
    const owner = await ownerFor(home.plugin, posts);
    posts.length = 0;
    const pending = applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message));
    home.publish({ terminalOutput: fixture.receipt });
    const result = await pending;
    expect(result.state).toEqual({ kind: "idle" });
    expect(result.receipt).toEqual(fixture.receipt);
  });

  it("refreshes the visible projection before publishing the ACK", async () => {
    const order: string[] = [];
    const home = handle({});
    const owner = await ownerFor(home.plugin, []);
    const pending = applyDirectoryEventPageBootstrapV1(
      owner,
      page,
      (message) => order.push(message.kind),
      async () => { order.push("refresh"); },
    );
    home.publish({ terminalOutput: fixture.receipt });
    expect((await pending).state).toEqual({ kind: "idle" });
    expect(order).toEqual(["refresh", "directory-bootstrap-ack"]);
  });

  it("re-offers the page only on a typed transient refusal and stops on a permanent one", async () => {
    const transientPosts: BackboneWorkerRequest[] = [];
    const transientCalls: string[] = [];
    const transient = await ownerFor(handle({ calls: transientCalls, refuse: fault("s.home.config-lane-busy", true) }).plugin, transientPosts);
    transientPosts.length = 0;
    expect((await applyDirectoryEventPageBootstrapV1(transient, page, (message) => transientPosts.push(message))).state).toEqual({ kind: "retrying", throughSeqInclusive: 11 });
    expect(transientPosts).toEqual([{ kind: "directory-bootstrap-reject", bootstrapEpoch: 3, receiptSha256: fixture.receipt.receiptSha256 }]);
    expect(transient.abort.signal.aborted).toBe(false);
    expect(transientCalls).not.toContain("destroy");

    // 🚫️ A page the guest REFUSES is permanent for this page: re-offering it is the retry storm
    // S4 measured (31 `event-page?after=0` in a 60 s window), and the code is what the notice shows.
    const refusedPosts: BackboneWorkerRequest[] = [];
    const refusedCalls: string[] = [];
    const refused = await ownerFor(handle({ calls: refusedCalls, refuse: fault("s.home.directory-event-page-invalid", false) }).plugin, refusedPosts);
    refusedPosts.length = 0;
    expect((await applyDirectoryEventPageBootstrapV1(refused, page, (message) => refusedPosts.push(message))).state).toEqual({ kind: "fault", code: "s.home.directory-event-page-invalid" });
    expect(refusedPosts).toEqual([{ kind: "directory-bootstrap-close", bootstrapEpoch: 3 }]);
    expect(refusedCalls.at(-1)).toBe("destroy");
  });

  it("re-offers the page when the terminal publication never arrives", async () => {
    const posts: BackboneWorkerRequest[] = [];
    const calls: string[] = [];
    const home = handle({ calls });
    const owner = await ownerFor(home.plugin, posts);
    posts.length = 0;
    const result = await applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message), undefined, 1);
    expect(result.state).toEqual({ kind: "retrying", throughSeqInclusive: 11 });
    expect(posts).toEqual([{ kind: "directory-bootstrap-reject", bootstrapEpoch: 3, receiptSha256: fixture.receipt.receiptSha256 }]);
    expect(owner.abort.signal.aborted).toBe(false);
    expect(calls).not.toContain("destroy");
    expect(home.subscribers()).toBe(0);
  });

  it("stops when the admitting reply started no typed operation at all", async () => {
    const posts: BackboneWorkerRequest[] = [];
    const calls: string[] = [];
    const owner = await ownerFor(handle({ calls, admission: fixture.receipt }).plugin, posts);
    posts.length = 0;
    expect((await applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message))).state).toEqual({ kind: "fault", code: "directory-bootstrap.operation-unstarted" });
    expect(posts).toEqual([{ kind: "directory-bootstrap-close", bootstrapEpoch: 3 }]);
    expect(calls.at(-1)).toBe("destroy");
  });

  it("closes and destroys on an exact receipt mismatch", async () => {
    const posts: BackboneWorkerRequest[] = [];
    const calls: string[] = [];
    const home = handle({ calls });
    const owner = await ownerFor(home.plugin, posts);
    posts.length = 0;
    const pending = applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message));
    home.publish({ terminalOutput: { ...fixture.receipt, throughSeqInclusive: 12 } });
    expect((await pending).state).toEqual({ kind: "fault", code: "directory-bootstrap.receipt-mismatch" });
    expect(posts).toEqual([{ kind: "directory-bootstrap-close", bootstrapEpoch: 3 }]);
    expect(calls.at(-1)).toBe("destroy");
  });

  it("suppresses a late receipt after cancellation", async () => {
    const posts: BackboneWorkerRequest[] = [];
    const home = handle({});
    const owner = await ownerFor(home.plugin, posts);
    posts.length = 0;
    const pending = applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message));
    await flush();
    await closeDirectoryHomeOwnerV1(owner, (message) => posts.push(message));
    home.publish({ terminalOutput: fixture.receipt });
    expect((await pending).state).toEqual({ kind: "fault", code: "directory-bootstrap.cancelled" });
    expect(posts).toEqual([{ kind: "directory-bootstrap-close", bootstrapEpoch: 3 }]);
    expect(home.subscribers()).toBe(0);
  });

  it("feeds the visible Home in either role and nothing else, equal to an AJV contains-oracle", () => {
    const surface = (row: (typeof fixture.ownerSurfaces)[number]["visible"]) =>
      ({ id: row.id, controllerId: row.id, dialect: { artifactKind: row.dialect, standard: "1", subset: "*" }, defaultModeId: "view", modes: [{ id: "view" }], windowKinds: [{ id: "main", actions: row.actions.map((id) => ({ id })) }] }) as unknown as AppDefinition;
    const landing = surface({ id: "s.space.home@1/*#editor", dialect: "s.space.home", actions: ["applyDirectoryEventPage"] });
    const oracle = new Ajv({ strict: true }).compile({
      type: "object",
      required: ["dialect", "actions"],
      properties: { dialect: { const: "s.space.home" }, actions: { type: "array", contains: { const: "applyDirectoryEventPage" } } },
    });
    expect(fixture.ownerSurfaces.map((row) => row.id)).toEqual(["home-editor", "home-viewer", "stale-home-viewer-without-feed", "space-index", "foreign-app-declaring-the-verb"]);
    for (const row of fixture.ownerSurfaces) {
      const visible = surface(row.visible);
      const owner = directoryHomeOwnerAppV1(landing, visible);
      expect(owner === visible, row.id).toBe(row.owner);
      expect(oracle({ dialect: row.visible.dialect, actions: row.visible.actions }), row.id).toBe(row.owner);
    }
  });

  it("renders explicit accessible EN and DE status without a fallback locale", () => {
    const cancel = vi.fn();
    render(<DirectoryBootstrapStatusNotice state={{ kind: "pending", throughSeqInclusive: 11, cancellable: true }} locale="de-DE" onCancel={cancel} />);
    expect(screen.getByRole("status").getAttribute("aria-live")).toBe("polite");
    fireEvent.click(screen.getByRole("button", { name: "Verzeichnisaktualisierung abbrechen" }));
    expect(cancel).toHaveBeenCalledOnce();
    cleanup();
    render(<DirectoryBootstrapStatusNotice state={{ kind: "retrying", throughSeqInclusive: 11 }} locale="en-US" onCancel={cancel} />);
    expect(screen.getByRole("status").textContent).toContain("Retrying directory update");
    cleanup();
    render(<DirectoryBootstrapStatusNotice state={{ kind: "fault", code: "x" }} locale="fr-FR" onCancel={cancel} />);
    expect(screen.getByRole("alert").getAttribute("data-directory-bootstrap")).toBe("locale-missing");
  });
});
