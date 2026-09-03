// #region 🧲️Header
/** @emoji 🧪️ Component tests for `@semio-tech/hub-admin` at its fetch boundary. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { vitestRunArguments } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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
  it("renders only the authenticated REST snapshot and reports freshness", async () => {
    mockFetch({
      "/admin/api/overview": OVERVIEW_BODY,
      "/admin/api/connections": [{ syncSessionId: "sync-1", spaceId: "sp-1", documentId: "doc-1", surface: "s.space.home@1/*#editor", actor: "user:u-1#tab-a", userId: "u-1", email: "amara@semio.dev", role: "author", connectedAtMs: 0, presenceKnown: false }],
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
