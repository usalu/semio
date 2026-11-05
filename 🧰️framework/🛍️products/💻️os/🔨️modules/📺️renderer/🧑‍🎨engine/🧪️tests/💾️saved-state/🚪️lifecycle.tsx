import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { useCloseDocumentState } from "../../🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🟠️close-document/🟦️.ts";
import { commitLocalFoldersConfigMutationV1, readLocalFolderBindingsV1 } from "../../🧱️elements/🏛️ShellHost/📎️local-folders/🟦️.tsx";

afterEach(() => { cleanup(); vi.useRealTimers(); });

function setup() {
  const storage = createMemoryStoragePort();
  const entry = (id: string) => ({ documentId: id, clientInstanceId: id, persistenceBindings: [{ kind: "folder" }], session: { pluginId: "stdio", app: { id: "txt" }, instanceId: 1 } });
  const documents = new Map([["a", entry("a")], ["b", entry("b")]]);
  const shellState = { sync: { syncBackboneUri: "actor://a", syncStatusByDocumentId: { a: { persisted: false, pendingMutations: 1 }, b: { persisted: false, pendingMutations: 1 } } } };
  const options = {
    scope: { storage }, openDocumentSessionsRef: { current: documents }, shellStateRef: { current: shellState },
    resolveSyncTargetSession: () => null, loadedPlugins: [], syncBackboneUri: "actor://a",
    closeDocumentRef: { current: null }, localDocumentOwnerRegistry: {}, core: { admitProgramRetirementRef: { current: null } }, dispatch: vi.fn(),
  } as unknown as Parameters<typeof useCloseDocumentState>[0];
  const hook = renderHook(() => useCloseDocumentState(options));
  return { ...hook, storage, documents, shellState, entry };
}

describe("document close lifecycle", () => {
  it("keeps the remembered folder when the person cancels detach during saving", async () => {
    const view = setup();
    commitLocalFoldersConfigMutationV1(view.storage, { mutation: "attachLocalFolder", documentId: "a", pluginId: "stdio", appId: "txt", folder: { kind: "path", path: "/documents" } });
    act(() => view.result.current.detachSyncBackbone());
    expect(readLocalFolderBindingsV1(view.storage).bindings).toHaveLength(1);
    await act(async () => view.result.current.closePrompt!.onKeep());
    expect(view.result.current.closePrompt).toBeNull();
    expect(readLocalFolderBindingsV1(view.storage).bindings).toHaveLength(1);
    expect(view.documents.has("a")).toBe(true);
  });

  it("does not let an older close clear the newer close prompt", async () => {
    const view = setup();
    let first!: Promise<"close" | "stay">, second!: Promise<"close" | "stay">;
    act(() => { first = view.result.current.requestCloseDocument("a"); });
    await act(async () => { second = view.result.current.requestCloseDocument("b"); await first; });
    expect(await first).toBe("stay");
    expect(view.result.current.closePrompt?.name).toBe("b");
    await act(async () => { view.result.current.closePrompt!.onKeep(); await second; });
    expect(await second).toBe("stay");
    expect(view.result.current.closePrompt).toBeNull();
    expect(view.documents.size).toBe(2);
  });

  it("keeps a replacement generation and clears the retired generation's prompt", async () => {
    vi.useFakeTimers();
    const view = setup();
    let closing!: Promise<"close" | "stay">;
    act(() => { closing = view.result.current.requestCloseDocument("a"); });
    const replacement = view.entry("replacement");
    view.documents.set("a", replacement);
    await act(async () => { await vi.advanceTimersByTimeAsync(100); });
    expect(await closing).toBe("stay");
    expect(view.documents.get("a")).toBe(replacement);
    expect(view.result.current.closePrompt).toBeNull();
  });
});

it("does not treat a bound document with missing status as safe to detach", async () => {
  const view = setup(); delete (view.shellState.sync.syncStatusByDocumentId as Record<string, unknown>).a;
  let pending!: Promise<string>;
  await act(async () => { pending = view.result.current.requestCloseDocument("a").catch(() => "error"); });
  expect(view.result.current.closePrompt?.phase).toBe("saving");
  await act(async () => { view.result.current.closePrompt!.onKeep(); await pending; });
  expect(await pending).toBe("stay"); expect(view.documents.has("a")).toBe(true);
});
