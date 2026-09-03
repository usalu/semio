// #region 🧲️Header
/** @emoji 🧪️ Component tests for `@semio-tech/hub-admin` at its fetch boundary. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { vitestRunArguments } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { AdminLocaleProvider } from "../../🧱️elements/📚️I18n/🟦️.tsx";
import { AdminAccessGate, AdminClient, AdminSessionProvider } from "../../🧱️elements/🔑️AdminSession/🟦️.tsx";
import { SpacesPage } from "../../🧱️elements/🏛️SpacesPage/🟦️.tsx";
import { ConnectionsPage } from "../../🧱️elements/🔴️ConnectionsPage/🟦️.tsx";
import { AdminApp } from "../../🧱️elements/🛡️AdminApp/🟦️.tsx";
// #endregion 🔌️Adapters

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
  window.history.replaceState(null, "", "/admin/");
  sessionStorage.clear();
  localStorage.clear();
});

describe("Admin locale selection", () => {
  it("renders an accessible bilingual chooser when the browser selected no supported locale", async () => {
    vi.spyOn(window.navigator, "languages", "get").mockReturnValue(["fr-FR"]);
    vi.spyOn(window.navigator, "language", "get").mockReturnValue("fr-FR");
    render(
      <AdminLocaleProvider>
        <div>gewählt</div>
      </AdminLocaleProvider>,
    );
    const chooser = screen.getByRole("dialog", { name: "Language · Sprache" });
    expect(chooser).toBeTruthy();
    const english = screen.getByRole("button", { name: "English" });
    const german = screen.getByRole("button", { name: "Deutsch" });
    expect(english.getAttribute("lang")).toBe("en");
    expect(german.getAttribute("lang")).toBe("de");
    fireEvent.click(german);
    await screen.findByText("gewählt");
  });
});

describe("test command routing", () => {
  it("consumes the Nx test level before forwarding Vitest arguments", async () => {
    const ts = await import("typescript");
    const path = resolve(process.cwd(), "📜️script.ts");
    const syntax = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const scriptClass = syntax.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
    expect(scriptClass && ts.isClassDeclaration(scriptClass)).toBe(true);
    const run = scriptClass && ts.isClassDeclaration(scriptClass) ? scriptClass.members.find((node) => ts.isMethodDeclaration(node) && node.name.getText(syntax) === "run") : undefined;
    const source = run?.getText(syntax) ?? "";
    expect(source).toContain("resolveTestLevel(segments)");
    expect(source).toContain("runVitest(this.root, rest,");
  });

  it("leaves an empty-selection failure to the owning Vitest config", () => {
    const args = vitestRunArguments(process.cwd(), ["🧪️admin.test.tsx"], "🧪️tests/🟦️.ts", false);
    expect(args).not.toContain("--passWithNoTests");
    expect(args.at(-1)).toBe("🧪️admin.test.tsx");
  });
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

describe("AdminClient typed intents", () => {
  it("requests space and document projections through exact bounded cursor pages", async () => {
    const requests: string[] = [];
    vi.stubGlobal(
      "fetch",
      vi.fn(async (input: string | URL) => {
        requests.push(String(input));
        return new Response(JSON.stringify({ rows: [], observedAtMs: 7 }), { status: 200, headers: { "content-type": "application/json" } });
      }),
    );
    const cursor = "a".repeat(84);
    await new AdminClient("http://hub.test").spaces(cursor);
    await new AdminClient("http://hub.test").documents("space/one", cursor);
    await new AdminClient("http://hub.test").space("space/one", cursor);
    await new AdminClient("http://hub.test").operation("operation/one");
    await new AdminClient("http://hub.test").cancelOperation("operation/one");
    const spaces = new URL(requests[0]!);
    expect(spaces.pathname).toBe("/admin/api/spaces");
    expect([...spaces.searchParams]).toEqual([
      ["limit", "100"],
      ["cursor", cursor],
    ]);
    const documents = new URL(requests[1]!);
    expect(documents.pathname).toBe("/admin/api/documents");
    expect([...documents.searchParams]).toEqual([
      ["limit", "100"],
      ["space", "space/one"],
      ["cursor", cursor],
    ]);
    const detail = new URL(requests[2]!);
    expect(detail.pathname).toBe("/admin/api/spaces/space%2Fone");
    expect([...detail.searchParams]).toEqual([
      ["limit", "100"],
      ["cursor", cursor],
    ]);
    expect(new URL(requests[3]!).pathname).toBe("/admin/api/operations/operation%2Fone");
    expect(new URL(requests[4]!).pathname).toBe("/admin/api/operations/operation%2Fone/cancel");
  });

  it("posts only closed typed intents and preserves terminal receipt semantics", async () => {
    const requests: Array<{ readonly path: string; readonly init?: RequestInit; readonly body: Record<string, unknown> }> = [];
    vi.stubGlobal(
      "fetch",
      vi.fn(async (input: string | URL, init?: RequestInit) => {
        const body = JSON.parse(String(init?.body)) as Record<string, unknown>;
        requests.push({ path: new URL(String(input)).pathname, init, body });
        const accepted = body.kind === "rebuild-directory-projections";
        return new Response(
          JSON.stringify({
            operationId: `operation:${String(body.kind)}`,
            correlationId: "correlation:test",
            state: accepted ? "accepted" : "succeeded",
            outcome: { code: accepted ? "directory-rebuild-running" : "ok", durable: !accepted },
          }),
          { status: accepted ? 202 : 200, headers: { "content-type": "application/json" } },
        );
      }),
    );
    const client = new AdminClient("http://hub.test");
    await client.createSpace("Studio", "studio", "private");
    await client.closeConnection("sync:one");
    await client.revokeUserSessions("user:one");
    const rebuild = await client.rebuild(7);
    expect(rebuild.receipt.state).toBe("accepted");
    expect(requests.map((request) => request.path)).toEqual(Array.from({ length: 4 }, () => "/admin/api/intents"));
    expect(requests.map((request) => request.body.kind)).toEqual(["create-space", "kick-connection", "revoke-user-sessions", "rebuild-directory-projections"]);
    expect(requests[0]!.body).toMatchObject({ name: "Studio", spaceKind: "studio", visibility: "private" });
    expect(requests[1]!.body).toMatchObject({ syncSessionId: "sync:one", reasonCode: "operator-kick" });
    expect(requests[2]!.body).toMatchObject({ userId: "user:one", reasonCode: "operator-revoke" });
    expect(requests[3]!.body).toMatchObject({ expectedHeadSeq: 7 });
    for (const request of requests) {
      expect(request.init?.credentials).toBe("same-origin");
      expect((request.init?.headers as Record<string, string>)["content-type"]).toBe("application/json");
      expect(request.body.requestId).toMatch(new RegExp(`^${String(request.body.kind)}:`));
      expect(request.body).not.toHaveProperty("command");
    }
  });

  it("rejects a failed terminal receipt instead of treating the mutation as success", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () =>
        new Response(JSON.stringify({ operationId: "operation:failed", correlationId: "correlation:test", state: "failed", outcome: { code: "rejected", durable: false } }), {
          status: 200,
          headers: { "content-type": "application/json" },
        }),
      ),
    );
    await expect(new AdminClient("http://hub.test").deleteSpace("space:one")).rejects.toMatchObject({ status: 409 });
  });
});

//#region 🔑️AdminBootstrap
describe("AdminSessionProvider", () => {
  it("exchanges one fragment nonce, clears it, and never creates a JavaScript bearer carrier", async () => {
    const proof = "a".repeat(64);
    window.history.replaceState(null, "", `/admin/#semio-admin=${proof}`);
    const requests: Array<{ readonly path: string; readonly init?: RequestInit }> = [];
    vi.stubGlobal("fetch", vi.fn(async (input: string | URL, init?: RequestInit) => {
      const path = new URL(String(input)).pathname;
      requests.push({ path, init });
      if (path === "/__semio/admin/bootstrap") return new Response(null, { status: 204 });
      if (path === "/admin/api/overview") return new Response(JSON.stringify(OVERVIEW_BODY), { status: 200, headers: { "content-type": "application/json" } });
      return new Response("not found", { status: 404 });
    }));

    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <AdminAccessGate />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );

    await waitFor(() => expect(requests.some((request) => request.path === "/admin/api/overview")).toBe(true));
    const bootstrap = requests.find((request) => request.path === "/__semio/admin/bootstrap")!;
    expect((bootstrap.init?.headers as Record<string, string>)["x-semio-admin-bootstrap"]).toBe(proof);
    expect(bootstrap.init?.credentials).toBe("same-origin");
    expect(window.location.hash).toBe("");
    expect(sessionStorage.length).toBe(0);
    expect(localStorage.length).toBe(0);
    expect(document.cookie).not.toContain("session.v1");
    for (const request of requests) expect(JSON.stringify(request.init ?? {})).not.toContain("Authorization");
  });

  it("clears and rejects a malformed fragment before any admin API request", async () => {
    window.history.replaceState(null, "", "/admin/#semio-admin=not-a-proof");
    const fetchMock = vi.fn();
    vi.stubGlobal("fetch", fetchMock);
    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <AdminAccessGate />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );
    await screen.findByRole("alert");
    expect(window.location.hash).toBe("");
    expect(fetchMock).not.toHaveBeenCalled();
    expect(screen.getByText(/dev-secure-admin/)).toBeTruthy();
  });
});
//#endregion 🔑️AdminBootstrap

describe("SpacesPage", () => {
  it("renders rows from a mocked admin client", async () => {
    mockFetch({
      "/admin/api/overview": OVERVIEW_BODY,
      "/admin/api/spaces": { rows: [{ id: "sp-1", name: "Test Space", kind: "studio", visibility: "private", ownerUserId: "u-1", memberCount: 1, documentCount: 0, activeConnections: 0, createdAtMs: 0, updatedAtMs: 0 }], observedAtMs: 0 },
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

  it("retains space and expanded-member rows across both bounded continuations", async () => {
    const spaceCursor = "a".repeat(84);
    const memberCursor = "b".repeat(84);
    const firstSpace = { id: "sp-1", name: "First Space", kind: "studio", visibility: "private", ownerUserId: "u-1", memberCount: 2, documentCount: 0, activeConnections: 0, createdAtMs: 0, updatedAtMs: 0 };
    const secondSpace = { ...firstSpace, id: "sp-2", name: "Second Space" };
    const requests: string[] = [];
    vi.stubGlobal(
      "fetch",
      vi.fn(async (input: string | URL) => {
        const url = new URL(String(input));
        requests.push(url.href);
        if (url.pathname === "/admin/api/overview") return new Response(JSON.stringify(OVERVIEW_BODY), { status: 200, headers: { "content-type": "application/json" } });
        if (url.pathname === "/admin/api/spaces") {
          const body = url.searchParams.get("cursor") === spaceCursor ? { rows: [secondSpace], observedAtMs: 2 } : { rows: [firstSpace], nextCursor: spaceCursor, observedAtMs: 1 };
          return new Response(JSON.stringify(body), { status: 200, headers: { "content-type": "application/json" } });
        }
        if (url.pathname === "/admin/api/spaces/sp-1") {
          const members =
            url.searchParams.get("cursor") === memberCursor
              ? { rows: [{ userId: "u-2", email: "second@example.test", displayName: "Second", role: "spectator" }], observedAtMs: 4 }
              : { rows: [{ userId: "u-1", email: "first@example.test", displayName: "First", role: "author" }], nextCursor: memberCursor, observedAtMs: 3 };
          return new Response(JSON.stringify({ ...firstSpace, members }), { status: 200, headers: { "content-type": "application/json" } });
        }
        return new Response("not found", { status: 404 });
      }),
    );

    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <SpacesPage />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );

    await screen.findByText("First Space");
    fireEvent.click(screen.getByRole("button", { name: "Load more spaces" }));
    await screen.findByText("Second Space");
    expect(screen.getByText("First Space")).toBeTruthy();
    fireEvent.click(document.querySelector('[data-row-id="space:sp-1"]')!);
    await screen.findByText("first@example.test");
    fireEvent.click(screen.getByRole("button", { name: "Load more members" }));
    await screen.findByText("second@example.test");
    expect(screen.getByText("first@example.test")).toBeTruthy();
    expect(requests.some((request) => new URL(request).searchParams.get("cursor") === spaceCursor)).toBe(true);
    expect(requests.some((request) => new URL(request).pathname === "/admin/api/spaces/sp-1" && new URL(request).searchParams.get("cursor") === memberCursor)).toBe(true);
  });
});

describe("ConnectionsPage", () => {
  it("renders only the authenticated REST snapshot and reports freshness", async () => {
    mockFetch({
      "/admin/api/overview": OVERVIEW_BODY,
      "/admin/api/connections": {
        rows: [{ syncSessionId: "sync-1", scope: { spaceId: "sp-1", documentId: "doc-1" }, authenticatedUserId: "u-1", email: "amara@semio.dev", role: "author", connectedAtMs: 0, source: "recorded-sync-session" }],
        observedAtMs: 0,
        source: "recorded-sync-sessions",
        headSeq: 3,
      },
    });

    render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <ConnectionsPage />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );

    await waitFor(() => expect(document.querySelector('[data-row-id="connection:sync-1"]')).not.toBeNull());
    expect(screen.getByText(/amara@semio.dev/)).toBeTruthy();
    expect(document.querySelector('[data-slot="admin-connections-freshness"]')?.textContent).toBe("Fresh snapshot");
  });

  it("aborts one pending poll on unmount and never schedules its successor", async () => {
    let connectionCalls = 0;
    let pendingSignal: AbortSignal | undefined;
    vi.stubGlobal("fetch", vi.fn(async (input: string | URL, init?: RequestInit) => {
      const path = new URL(String(input)).pathname;
      if (path === "/admin/api/overview") return new Response(JSON.stringify(OVERVIEW_BODY), { status: 200, headers: { "content-type": "application/json" } });
      if (path === "/admin/api/connections") {
        connectionCalls += 1;
        pendingSignal = init?.signal ?? undefined;
        return await new Promise<Response>((_resolve, reject) => pendingSignal?.addEventListener("abort", () => reject(new DOMException("aborted", "AbortError")), { once: true }));
      }
      return new Response("not found", { status: 404 });
    }));
    const view = render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <ConnectionsPage />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );
    await waitFor(() => expect(connectionCalls).toBe(1));
    view.unmount();
    expect(pendingSignal?.aborted).toBe(true);
    await new Promise((resolveWait) => setTimeout(resolveWait, 25));
    expect(connectionCalls).toBe(1);
  });

  it("retains the last successful connection snapshot and marks it stale after a failed poll", async () => {
    vi.useFakeTimers();
    let connectionCalls = 0;
    vi.stubGlobal(
      "fetch",
      vi.fn(async (input: string | URL) => {
        const path = new URL(String(input)).pathname;
        if (path === "/admin/api/overview") return new Response(JSON.stringify(OVERVIEW_BODY), { status: 200, headers: { "content-type": "application/json" } });
        if (path === "/admin/api/connections") {
          connectionCalls += 1;
          if (connectionCalls > 1) return new Response("unavailable", { status: 503 });
          return new Response(
            JSON.stringify({
              rows: [{ syncSessionId: "sync-retained", scope: { spaceId: "space", documentId: "document" }, authenticatedUserId: "user", connectedAtMs: 0, source: "recorded-sync-session" }],
              observedAtMs: 0,
              source: "recorded-sync-sessions",
              headSeq: 3,
            }),
            { status: 200, headers: { "content-type": "application/json" } },
          );
        }
        return new Response("not found", { status: 404 });
      }),
    );
    const view = render(
      <AdminLocaleProvider>
        <AdminSessionProvider baseUrl="http://hub.test">
          <ConnectionsPage />
        </AdminSessionProvider>
      </AdminLocaleProvider>,
    );
    await act(async () => void (await Promise.resolve()));
    expect(document.querySelector('[data-row-id="connection:sync-retained"]')).not.toBeNull();
    expect(document.querySelector('[data-slot="admin-connections-freshness"]')?.textContent).toBe("Fresh snapshot");
    await act(async () => {
      await vi.advanceTimersByTimeAsync(2_000);
    });
    expect(connectionCalls).toBe(2);
    expect(document.querySelector('[data-row-id="connection:sync-retained"]')).not.toBeNull();
    expect(document.querySelector('[data-slot="admin-connections-freshness"]')?.textContent).toBe("Stale snapshot");
    view.unmount();
    vi.useRealTimers();
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

  it("mounts REST polling only for the active panel and cleans up on every switch", async () => {
    sessionStorage.clear();
    const fetchMock = mockFetch({
      "/admin/api/overview": OVERVIEW_BODY,
      "/admin/api/spaces": { rows: [], observedAtMs: 0 },
      "/admin/api/connections": { rows: [], observedAtMs: 0, source: "recorded-sync-sessions", headSeq: 3 },
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

    fireEvent.click(spacesTab);
    expect(spacesTab.getAttribute("aria-selected")).toBe("true");
    expect(overviewTab.getAttribute("aria-selected")).toBe("false");
    expect(overviewPanel.isConnected).toBe(false);
    const spacesPanel = document.getElementById(spacesTab.getAttribute("aria-controls")!) as HTMLDivElement;
    expect(spacesPanel).not.toBeNull();
    expect(spacesPanel.getAttribute("aria-labelledby")).toBe("admin-tab-spaces");
    await waitFor(() => expect(requestCount(fetchMock, "/admin/api/spaces")).toBe(1));
    expect(requestCount(fetchMock, "/admin/api/connections")).toBe(0);

    fireEvent.click(connectionsTab);
    await waitFor(() => expect(requestCount(fetchMock, "/admin/api/connections")).toBe(1));
    expect(requestCount(fetchMock, "/admin/api/overview")).toBe(2);
    expect(document.querySelectorAll('[role="tabpanel"]')).toHaveLength(1);

    fireEvent.click(spacesTab);
    await waitFor(() => expect(requestCount(fetchMock, "/admin/api/spaces")).toBe(2));

    fireEvent.click(connectionsTab);
    await waitFor(() => expect(requestCount(fetchMock, "/admin/api/connections")).toBe(2));
    expect(requestCount(fetchMock, "/admin/api/overview")).toBe(2);
  });
});
//#endregion 🛡️AdminTabs
