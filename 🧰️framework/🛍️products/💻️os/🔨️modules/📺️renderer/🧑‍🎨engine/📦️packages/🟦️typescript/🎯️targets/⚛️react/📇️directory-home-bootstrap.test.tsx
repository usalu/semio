import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import Ajv2020 from "ajv/dist/2020.js";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { AppDefinition } from "@semio-tech/framework";
import type { BackboneWorkerRequest, BackboneWorkerResponse } from "@semio-tech/framework-os";
import type { PluginWasmHandle } from "../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import {
  DIRECTORY_PROJECTION_RECEIPT_SCHEMA,
  DirectoryBootstrapStatusNotice,
  applyDirectoryEventPageBootstrapV1,
  closeDirectoryHomeOwnerV1,
  openDirectoryHomeOwnerV1,
  parseDirectoryProjectionReceiptV1,
} from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx";
import schema from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🧬️.schema.json";
import fixture from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🔣️.json";

afterEach(cleanup);

const app = {
  id: "s.space.home@v1/*#editor",
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

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (reason: unknown) => void;
  const promise = new Promise<T>((ok, bad) => { resolve = ok; reject = bad; });
  return { promise, resolve, reject };
}

function handle(output: Promise<unknown> | unknown, calls: string[] = []): PluginWasmHandle {
  return {
    pluginId: "s.space",
    manifest: { pluginId: "s.space", label: "Space", version: "1", apps: [app], examples: [] },
    createApp: async () => { calls.push("create"); return 41; },
    destroyApp: async () => { calls.push("destroy"); },
    handleAction: async (_instance: number, invocation: string) => { calls.push(`action:${invocation}`); return { output: await output }; },
  } as unknown as PluginWasmHandle;
}

async function ownerFor(plugin: PluginWasmHandle, posts: BackboneWorkerRequest[]) {
  return openDirectoryHomeOwnerV1({ plugin, app, baseUrl: "https://hub.example", bootstrapEpoch: 3, locale: "de-DE", terminology: "native", post: (message) => posts.push(message) });
}

describe("retained hidden Home directory bootstrap", () => {
  it("validates the language-neutral receipt and hostile vectors with AJV and the independent parser", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(parseDirectoryProjectionReceiptV1(fixture.receipt)).toEqual(fixture.receipt);
    for (const row of fixture.hostile) {
      const hostile = { ...structuredClone(fixture.receipt), ...row.patch };
      expect(parseDirectoryProjectionReceiptV1(hostile), row.id).toBeNull();
    }
  });

  it("opens the worker epoch only after the hidden Home instance exists", async () => {
    const calls: string[] = [];
    const posts: BackboneWorkerRequest[] = [];
    const owner = await ownerFor(handle(fixture.receipt, calls), posts);
    expect(calls).toEqual(["create"]);
    expect(posts).toEqual([{ kind: "directory-bootstrap-open", baseUrl: "https://hub.example", after: 0, bootstrapEpoch: 3 }]);
    expect(owner.viewState.locale).toBe("de-DE");
    await closeDirectoryHomeOwnerV1(owner, (message) => posts.push(message));
    expect(calls.at(-1)).toBe("destroy");
  });

  it("serializes pages and ACKs only after the typed terminal receipt resolves", async () => {
    const result = deferred<unknown>();
    const posts: BackboneWorkerRequest[] = [];
    const calls: string[] = [];
    const owner = await ownerFor(handle(result.promise, calls), posts);
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
    expect(posts).toEqual([]);
    expect(JSON.parse(calls.find((call) => call.startsWith("action:"))!.slice(7))).toMatchObject({ address: { actionId: "applyDirectoryEventPage" }, arguments: { pageJson: page.canonicalJson } });
    result.resolve(fixture.receipt);
    expect((await pending).receipt).toEqual(fixture.receipt);
    expect(posts).toEqual([{ kind: "directory-bootstrap-ack", bootstrapEpoch: 3, sessionBindingSha256: fixture.receipt.sessionBindingSha256, authorizationGeneration: 7, throughSeqInclusive: 11, receiptSha256: fixture.receipt.receiptSha256 }]);
  });

  it("rejects a recoverable Home action failure once without dropping the owner", async () => {
    const posts: BackboneWorkerRequest[] = [];
    const calls: string[] = [];
    const owner = await ownerFor(handle(Promise.reject(new Error("capacity")), calls), posts);
    posts.length = 0;
    expect((await applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message))).state).toEqual({ kind: "retrying", throughSeqInclusive: 11 });
    expect(posts).toEqual([{ kind: "directory-bootstrap-reject", bootstrapEpoch: 3, receiptSha256: fixture.receipt.receiptSha256 }]);
    expect(owner.abort.signal.aborted).toBe(false);
    expect(calls).not.toContain("destroy");
  });

  it("closes and destroys on an exact receipt mismatch", async () => {
    const posts: BackboneWorkerRequest[] = [];
    const calls: string[] = [];
    const owner = await ownerFor(handle({ ...fixture.receipt, throughSeqInclusive: 12 }, calls), posts);
    posts.length = 0;
    expect((await applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message))).state).toEqual({ kind: "fault", code: "directory-bootstrap.receipt-mismatch" });
    expect(posts).toEqual([{ kind: "directory-bootstrap-close", bootstrapEpoch: 3 }]);
    expect(calls.at(-1)).toBe("destroy");
  });

  it("suppresses a late receipt after cancellation", async () => {
    const result = deferred<unknown>();
    const posts: BackboneWorkerRequest[] = [];
    const owner = await ownerFor(handle(result.promise), posts);
    posts.length = 0;
    const pending = applyDirectoryEventPageBootstrapV1(owner, page, (message) => posts.push(message));
    await closeDirectoryHomeOwnerV1(owner, (message) => posts.push(message));
    result.resolve(fixture.receipt);
    expect((await pending).state).toEqual({ kind: "fault", code: "directory-bootstrap.cancelled" });
    expect(posts).toEqual([{ kind: "directory-bootstrap-close", bootstrapEpoch: 3 }]);
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
