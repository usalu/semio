// #region 🧲️Header
/** @emoji 🧪️ Component tests for `@semio-tech/hub-admin` — mocks at the transport boundary (`fetch`
 * for `AdminClient`, a fake `WebSocket` for `DirectoryClient.stream`), matching this repo's own
 * `DirectoryClient` test style (`🧰️framework/…/💻️os/🟦️.ts`'s `FakeDirectoryWebSocket`)
 * rather than reaching into React context internals. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { DirectoryStreamMessage } from "@semio-tech/framework-os";
import { AdminLocaleProvider } from "../../🧱️elements/📚️I18n/🟦️.tsx";
import { AdminSessionProvider, AdminTokenForm } from "../../🧱️elements/🔑️AdminSession/🟦️.tsx";
import { SpacesPage } from "../../🧱️elements/🏛️SpacesPage/🟦️.tsx";
import { ConnectionsPage } from "../../🧱️elements/🔴️ConnectionsPage/🟦️.tsx";
import { AdminApp } from "../../🧱️elements/🛡️AdminApp/🟦️.tsx";
// #endregion 🔌️Adapters

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

//#region 🔖️FetchRouter
const OVERVIEW_BODY = { counts: { spaces: 1, users: 0, connections: 0 }, backends: { sqlite: true, postgres: false, neo4j: false }, dataDirBytes: 0, headSeq: 3, openArtifacts: 0 };

/** 🧵️ Routes `fetch(url)` by pathname to a canned JSON body — every {@link AdminSessionProvider}
 * mount probes `/admin/api/overview`, so every test registers it. */
function mockFetch(routes: Record<string, unknown>): ReturnType<typeof vi.fn> {
  const fetchMock = vi.fn(async (input: string | URL) => {
    const pathname = new URL(String(input)).pathname;
    const body = routes[pathname];
    if (body === undefined) return new Response("not found", { status: 404 });
    return new Response(JSON.stringify(body), { status: 200, headers: { "content-type": "application/json" } });
  });
  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
}

/** 🔢️ Counts transport requests to one exact admin path. */
function requestCount(fetchMock: ReturnType<typeof vi.fn>, pathname: string): number {
  return fetchMock.mock.calls.filter(([input]) => new URL(String(input)).pathname === pathname).length;
}
//#endregion 🔖️FetchRouter

//#region 🔖️FakeWebSocket
/** 🧵️ Mirrors `🧰️framework/…/💻️os/🟦️.ts`'s own `FakeDirectoryWebSocket` test double — this
 * package cannot import that in-source test class, so it is duplicated here at the same shape. */
class FakeDirectoryWebSocket {
  static instances: FakeDirectoryWebSocket[] = [];
  readonly url: string;
  readyState = 0;
  onopen: (() => void) | null = null;
  onmessage: ((event: { data: string }) => void) | null = null;
  onclose: (() => void) | null = null;
  onerror: (() => void) | null = null;
  closeCount = 0;
  constructor(url: string) {
    this.url = url;
    FakeDirectoryWebSocket.instances.push(this);
  }
  send(): void {}
  close(): void {
    if (this.readyState >= 2) return;
    this.closeCount += 1;
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

//#region 🔑️AdminTokenForm
describe("AdminTokenForm", () => {
  it("submits a trimmed token through the owned native form boundary", async () => {
    sessionStorage.clear();
    const authorizations: Array<string | undefined> = [];
    vi.stubGlobal(
      "fetch",
      vi.fn(async (_input: string | URL, init?: RequestInit) => {
        const authorization = (init?.headers as Record<string, string> | undefined)?.authorization;
        authorizations.push(authorization);
        return authorization === "Bearer secret-token" ? new Response(JSON.stringify(OVERVIEW_BODY), { status: 200, headers: { "content-type": "application/json" } }) : new Response("unauthorized", { status: 401 });
      }),
    );

    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <AdminTokenForm />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );

    const input = document.getElementById("admin-session-token") as HTMLInputElement;
    const form = input.form!;
    await waitFor(() => expect((document.getElementById("admin-session-submit") as HTMLButtonElement).disabled).toBe(false));
    expect(form.dataset.slot).toBe("form");
    fireEvent.change(input, { target: { value: "  secret-token  " } });
    fireEvent.submit(form);
    await waitFor(() => expect(authorizations).toContain("Bearer secret-token"));
    expect(sessionStorage.getItem("semio.hub.admin.token")).toBe("secret-token");
  });
});
//#endregion 🔑️AdminTokenForm

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
    const create = document.getElementById("admin-space-create-open") as HTMLButtonElement;
    create.focus();
    fireEvent.click(create);
    const dialog = await screen.findByRole("dialog");
    const title = dialog.querySelector<HTMLElement>('[data-slot="dialog-title"]');
    const name = document.getElementById("admin-space-create-name") as HTMLInputElement;
    expect(title).not.toBeNull();
    expect(dialog.getAttribute("aria-labelledby")).toBe(title?.id);
    expect(document.activeElement).toBe(name);
    fireEvent.click(document.getElementById("admin-space-create-cancel")!);
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
    expect(document.activeElement).toBe(create);
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

//#region 🛡️AdminTabs
describe("AdminApp tabs", () => {
  it("changes locale through the owned portalled Select", async () => {
    sessionStorage.clear();
    localStorage.clear();
    mockFetch({ "/admin/api/overview": OVERVIEW_BODY });
    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <AdminApp />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );
    await waitFor(() => expect(document.getElementById("admin-tab-overview")).not.toBeNull());
    const trigger = screen.getByRole("combobox");
    expect(trigger.textContent).toContain("EN");
    fireEvent.click(trigger);
    const listbox = await screen.findByRole("listbox");
    expect(document.activeElement).toBe(listbox);
    fireEvent.click(screen.getByRole("option", { name: "DE" }));
    await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull());
    expect(trigger.textContent).toContain("DE");
    expect(document.activeElement).toBe(trigger);
  });

  it("mounts requests and streams only for the active panel and cleans up on every switch", async () => {
    sessionStorage.clear();
    FakeDirectoryWebSocket.instances = [];
    vi.stubGlobal("WebSocket", FakeDirectoryWebSocket);
    const fetchMock = mockFetch({
      "/admin/api/overview": OVERVIEW_BODY,
      "/admin/api/spaces": [],
      "/admin/api/connections": [],
    });

    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <AdminApp />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );

    await waitFor(() => expect(document.getElementById("admin-tab-overview")).not.toBeNull());
    const overviewTab = document.getElementById("admin-tab-overview") as HTMLButtonElement;
    const spacesTab = document.getElementById("admin-tab-spaces") as HTMLButtonElement;
    const connectionsTab = document.getElementById("admin-tab-connections") as HTMLButtonElement;
    expect(overviewTab.getAttribute("aria-selected")).toBe("true");
    const overviewPanel = document.getElementById(overviewTab.getAttribute("aria-controls")!) as HTMLDivElement;
    expect(overviewPanel).not.toBeNull();
    await waitFor(() => expect(requestCount(fetchMock, "/admin/api/overview")).toBe(2));
    expect(requestCount(fetchMock, "/admin/api/spaces")).toBe(0);
    expect(requestCount(fetchMock, "/admin/api/connections")).toBe(0);
    expect(requestCount(fetchMock, "/admin/api/users")).toBe(0);
    expect(requestCount(fetchMock, "/admin/api/documents")).toBe(0);
    expect(requestCount(fetchMock, "/admin/api/events")).toBe(0);
    expect(FakeDirectoryWebSocket.instances).toHaveLength(0);

    fireEvent.click(spacesTab);
    expect(spacesTab.getAttribute("aria-selected")).toBe("true");
    expect(overviewTab.getAttribute("aria-selected")).toBe("false");
    expect(overviewPanel.isConnected).toBe(false);
    const spacesPanel = document.getElementById(spacesTab.getAttribute("aria-controls")!) as HTMLDivElement;
    expect(spacesPanel).not.toBeNull();
    expect(spacesPanel.getAttribute("aria-labelledby")).toBe("admin-tab-spaces");
    await waitFor(() => expect(requestCount(fetchMock, "/admin/api/spaces")).toBe(1));
    expect(requestCount(fetchMock, "/admin/api/connections")).toBe(0);
    expect(FakeDirectoryWebSocket.instances).toHaveLength(0);

    fireEvent.click(connectionsTab);
    await waitFor(() => expect(FakeDirectoryWebSocket.instances).toHaveLength(1));
    expect(requestCount(fetchMock, "/admin/api/connections")).toBe(1);
    expect(requestCount(fetchMock, "/admin/api/overview")).toBe(3);
    expect(document.querySelectorAll('[role="tabpanel"]')).toHaveLength(1);
    const firstSocket = FakeDirectoryWebSocket.instances[0]!;
    expect(firstSocket.closeCount).toBe(0);

    fireEvent.click(spacesTab);
    await waitFor(() => expect(firstSocket.closeCount).toBe(1));
    await waitFor(() => expect(requestCount(fetchMock, "/admin/api/spaces")).toBe(2));
    expect(FakeDirectoryWebSocket.instances).toHaveLength(1);

    fireEvent.click(connectionsTab);
    await waitFor(() => expect(FakeDirectoryWebSocket.instances).toHaveLength(2));
    expect(requestCount(fetchMock, "/admin/api/connections")).toBe(2);
    expect(requestCount(fetchMock, "/admin/api/overview")).toBe(4);
    expect(firstSocket.closeCount).toBe(1);
    expect(FakeDirectoryWebSocket.instances[1]?.closeCount).toBe(0);
  });
});
//#endregion 🛡️AdminTabs
