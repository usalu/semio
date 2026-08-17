// #region 🧲️Header
/** @emoji 🧪️ Component tests for `@semio-tech/hub-admin` — mocks at the transport boundary (`fetch`
 * for `AdminClient`, a fake `WebSocket` for `DirectoryClient.stream`), matching this repo's own
 * `DirectoryClient` test style (`🧰️framework/…/💻️os/🟦️component.ts`'s `FakeDirectoryWebSocket`)
 * rather than reaching into React context internals. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { DirectoryStreamMessage } from "@semio-tech/framework-os";
import { AdminLocaleProvider } from "../../🧱️elements/📚️I18n/🟦️component.tsx";
import { AdminSessionProvider } from "../../🧱️elements/🔑️AdminSession/🟦️component.tsx";
import { SpacesPage } from "../../🧱️elements/🏛️SpacesPage/🟦️component.tsx";
import { ConnectionsPage } from "../../🧱️elements/🔴️ConnectionsPage/🟦️component.tsx";
// #endregion 🔌️Adapters

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

//#region 🔖️FetchRouter
const OVERVIEW_BODY = { counts: { spaces: 1, users: 0, connections: 0 }, backends: { sqlite: true, postgres: false, neo4j: false }, dataDirBytes: 0, headSeq: 3, openArtifacts: 0 };

/** 🧵️ Routes `fetch(url)` by pathname to a canned JSON body — every {@link AdminSessionProvider}
 * mount probes `/admin/api/overview`, so every test registers it. */
function mockFetch(routes: Record<string, unknown>): void {
  vi.stubGlobal(
    "fetch",
    vi.fn(async (input: string | URL) => {
      const pathname = new URL(String(input)).pathname;
      const body = routes[pathname];
      if (body === undefined) return new Response("not found", { status: 404 });
      return new Response(JSON.stringify(body), { status: 200, headers: { "content-type": "application/json" } });
    }),
  );
}
//#endregion 🔖️FetchRouter

//#region 🔖️FakeWebSocket
/** 🧵️ Mirrors `🧰️framework/…/💻️os/🟦️component.ts`'s own `FakeDirectoryWebSocket` test double — this
 * package cannot import that in-source test class, so it is duplicated here at the same shape. */
class FakeDirectoryWebSocket {
  static instances: FakeDirectoryWebSocket[] = [];
  readonly url: string;
  readyState = 0;
  onopen: (() => void) | null = null;
  onmessage: ((event: { data: string }) => void) | null = null;
  onclose: (() => void) | null = null;
  onerror: (() => void) | null = null;
  constructor(url: string) {
    this.url = url;
    FakeDirectoryWebSocket.instances.push(this);
  }
  send(): void {}
  close(): void {
    this.readyState = 3;
  }
  triggerOpen(): void {
    this.readyState = 1;
    this.onopen?.();
  }
  triggerMessage(message: DirectoryStreamMessage): void {
    this.onmessage?.({ data: JSON.stringify(message) });
  }
}
//#endregion 🔖️FakeWebSocket

describe("SpacesPage", () => {
  it("renders rows from a mocked admin client", async () => {
    mockFetch({
      "/admin/api/overview": OVERVIEW_BODY,
      "/admin/api/spaces": [{ id: "sp-1", name: "Test Space", kind: "studio", visibility: "private", ownerUserId: "u-1", memberCount: 1, documentCount: 0, activeConnections: 0, createdAtMs: 0, updatedAtMs: 0 }],
    });

    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <SpacesPage />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );

    await waitFor(() => expect(screen.getByText("Test Space")).toBeTruthy());
    const row = document.querySelector('[data-row-id="space:sp-1"]');
    expect(row).not.toBeNull();
  });
});

describe("ConnectionsPage", () => {
  it("updates live on a pushed connection message", async () => {
    mockFetch({
      "/admin/api/overview": OVERVIEW_BODY,
      "/admin/api/connections": [],
    });
    FakeDirectoryWebSocket.instances = [];
    vi.stubGlobal("WebSocket", FakeDirectoryWebSocket);

    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <ConnectionsPage />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );

    await waitFor(() => expect(FakeDirectoryWebSocket.instances).toHaveLength(1));
    const socket = FakeDirectoryWebSocket.instances[0]!;
    expect(socket.url).toBe(`ws://${window.location.host}/directory/ws?since=${OVERVIEW_BODY.headSeq}`);
    socket.triggerOpen();
    socket.triggerMessage({
      kind: "connection",
      phase: "opened",
      connection: { syncSessionId: "sync-1", spaceId: "sp-1", documentId: "doc-1", surface: "s.space.home@1/*#editor", actor: "user:u-1#tab-a", userId: "u-1", email: "amara@semio.dev", role: "author", connectedAtMs: 0, presenceKnown: false },
    });

    await waitFor(() => expect(document.querySelector('[data-row-id="connection:sync-1"]')).not.toBeNull());
    expect(screen.getByText(/amara@semio.dev/)).toBeTruthy();

    socket.triggerMessage({
      kind: "connection",
      phase: "closed",
      connection: { syncSessionId: "sync-1", spaceId: "sp-1", documentId: "doc-1", surface: "s.space.home@1/*#editor", actor: "user:u-1#tab-a", userId: "u-1", email: "amara@semio.dev", role: "author", connectedAtMs: 0, presenceKnown: false },
    });

    await waitFor(() => expect(document.querySelector('[data-row-id="connection:sync-1"]')).toBeNull());
  });
});
