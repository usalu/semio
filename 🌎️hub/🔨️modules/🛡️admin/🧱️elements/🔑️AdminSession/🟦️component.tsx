// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🔑️AdminSession/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, Input } from "@semio-tech/ui-react";
import type { ConnectionView, DirectoryCommand, DirectoryEvent, DocumentView, InviteView, MemberView, SpaceView, UserView } from "@semio-tech/framework-os";
import { useAdminT } from "../📚️I18n/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️AdminClient
/** 🚨️ Thrown by every {@link AdminClient} call on a non-2xx response — mirrors `DirectoryHttpError`
 * (framework-os's `DirectoryClient`), which this crate cannot reuse: `/admin/api/*` is a route family
 * `DirectoryClient` never covers (contract §C2's admin surface, bearer `OS_HUB_ADMIN_TOKEN` /
 * loopback, not a user session). */
export class AdminHttpError extends Error {
  readonly status: number;
  constructor(status: number, message: string) {
    super(message);
    this.status = status;
  }
}

export interface AdminOverview {
  counts: { spaces: number; users: number; connections: number };
  backends: { sqlite: boolean; postgres: boolean; neo4j: boolean };
  dataDirBytes: number;
  headSeq: number;
  openArtifacts: number;
}

export type AdminSpaceDetail = SpaceView & { members: MemberView[]; documents: DocumentView[]; invites: InviteView[] };

/** 📡️ Typed facade over the hub's `/admin/api/*` surface (contract §C2) — the counterpart to
 * `@semio-tech/framework-os`'s `DirectoryClient`, which only covers `/directory/*` and
 * `/auth/sessions/*`. `fetch` only, no external HTTP library (CLAUDE.md). */
export class AdminClient {
  private readonly baseUrl: string;
  readonly token: string | undefined;

  constructor(baseUrl: string, token?: string) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
    this.token = token;
  }

  private headers(json: boolean): Record<string, string> {
    const headers: Record<string, string> = {};
    if (json) headers["content-type"] = "application/json";
    if (this.token) headers.authorization = `Bearer ${this.token}`;
    return headers;
  }

  private async getJson<T>(path: string): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, { headers: this.headers(false) });
    if (!response.ok) throw new AdminHttpError(response.status, `admin: GET ${path} failed (${response.status})`);
    return (await response.json()) as T;
  }

  private async postJson<T>(path: string, body?: unknown): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, { method: "POST", headers: this.headers(true), body: body === undefined ? undefined : JSON.stringify(body) });
    if (!response.ok) throw new AdminHttpError(response.status, `admin: POST ${path} failed (${response.status})`);
    const text = await response.text();
    return (text.length === 0 ? undefined : JSON.parse(text)) as T;
  }

  overview(): Promise<AdminOverview> {
    return this.getJson<AdminOverview>("/admin/api/overview");
  }

  spaces(): Promise<readonly SpaceView[]> {
    return this.getJson<SpaceView[]>("/admin/api/spaces");
  }

  space(id: string): Promise<AdminSpaceDetail> {
    return this.getJson<AdminSpaceDetail>(`/admin/api/spaces/${encodeURIComponent(id)}`);
  }

  users(): Promise<readonly UserView[]> {
    return this.getJson<UserView[]>("/admin/api/users");
  }

  connections(): Promise<readonly ConnectionView[]> {
    return this.getJson<ConnectionView[]>("/admin/api/connections");
  }

  documents(spaceId?: string): Promise<readonly DocumentView[]> {
    return this.getJson<DocumentView[]>(spaceId ? `/admin/api/documents?space=${encodeURIComponent(spaceId)}` : "/admin/api/documents");
  }

  events(since = 0, limit = 500): Promise<readonly DirectoryEvent[]> {
    return this.getJson<DirectoryEvent[]>(`/admin/api/events?since=${encodeURIComponent(String(since))}&limit=${encodeURIComponent(String(limit))}`);
  }

  command(command: DirectoryCommand): Promise<{ events: readonly DirectoryEvent[]; result?: unknown }> {
    return this.postJson("/admin/api/commands", command);
  }

  rebuild(): Promise<{ eventsReplayed: number }> {
    return this.postJson("/admin/api/directory/rebuild");
  }

  closeConnection(syncSessionId: string): Promise<void> {
    return this.postJson(`/admin/api/connections/${encodeURIComponent(syncSessionId)}/close`);
  }

  revokeUserSessions(userId: string): Promise<void> {
    return this.postJson(`/admin/api/users/${encodeURIComponent(userId)}/sessions/revoke`);
  }
}
//#endregion 🔖️AdminClient

//#region 🔖️Session
const TOKEN_STORAGE_KEY = "semio.hub.admin.token";

/** @emoji 🚦️ `unreachable` is deliberately distinct from `unauthorized`: a hub that is not running at
 * all fails the probe with a transport error, and reporting that as "your token was rejected" sends
 * the operator hunting for a credential when the actual fix is to start the hub. */
export type AdminSessionStatus = "probing" | "authorized" | "unauthorized" | "unreachable";

export interface AdminSessionState {
  status: AdminSessionStatus;
  client: AdminClient;
  setToken: (token: string) => void;
}

const AdminSessionContext = React.createContext<AdminSessionState | null>(null);

/** 🔑️ Probes `GET /admin/api/overview` on mount/token-change (contract §C2's admin auth: bearer
 * `OS_HUB_ADMIN_TOKEN`, or loopback-peer-is-admin when unset — this page cannot tell which applies,
 * so it always probes rather than guessing). `authorized` renders `children`; `unauthorized` renders
 * the bearer-token form instead. The token, once accepted, is kept in `sessionStorage` only (never
 * `localStorage` — an admin bearer token should not outlive the browser tab). */
export function AdminSessionProvider({ baseUrl, children }: { readonly baseUrl: string; readonly children: React.ReactNode }): React.ReactElement {
  const [token, setTokenState] = React.useState<string | undefined>(() => {
    if (typeof sessionStorage === "undefined") return undefined;
    return sessionStorage.getItem(TOKEN_STORAGE_KEY) ?? undefined;
  });
  const [status, setStatus] = React.useState<AdminSessionStatus>("probing");
  const client = React.useMemo(() => new AdminClient(baseUrl, token), [baseUrl, token]);

  React.useEffect(() => {
    let cancelled = false;
    setStatus("probing");
    client
      .overview()
      .then(() => {
        if (!cancelled) setStatus("authorized");
      })
      .catch((error: unknown) => {
        if (cancelled) return;
        // 🩺️ Only a real HTTP rejection means "bad/absent token"; anything else (ECONNREFUSED, DNS,
        // timeout, CORS) means the hub is not answering at all.
        setStatus(error instanceof AdminHttpError ? "unauthorized" : "unreachable");
      });
    return () => {
      cancelled = true;
    };
  }, [client]);

  const setToken = React.useCallback((next: string) => {
    if (typeof sessionStorage !== "undefined") sessionStorage.setItem(TOKEN_STORAGE_KEY, next);
    setTokenState(next);
  }, []);

  const value = React.useMemo<AdminSessionState>(() => ({ status, client, setToken }), [status, client, setToken]);
  return <AdminSessionContext.Provider value={value}>{children}</AdminSessionContext.Provider>;
}

export function useAdminSession(): AdminSessionState {
  const state = React.useContext(AdminSessionContext);
  if (!state) throw new Error("useAdminSession must be used within AdminSessionProvider");
  return state;
}
//#endregion 🔖️Session

//#region 🔖️TokenForm
/** 🚪️ Rendered by `AdminApp` in place of every other page while `status !== "authorized"` — a bare
 * bearer-token form. Submitting re-probes via `setToken` (which recreates `client`, re-running the
 * probe effect above); a still-rejected token flips back to `"unauthorized"` and this form re-renders
 * with the error line shown. */
export function AdminTokenForm(): React.ReactElement {
  const t = useAdminT();
  const { status, setToken } = useAdminSession();
  const [draft, setDraft] = React.useState("");

  // 🔌️ No hub answering: a token form here would be a dead end, so show what is actually wrong and the
  // command that fixes it instead of prompting for a credential that cannot help.
  if (status === "unreachable") {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div id="admin-session-unreachable" className="flex w-full max-w-md flex-col gap-single">
          <h1 className="text-lg font-semibold text-emphasized">{t("admin.session.unreachableTitle")}</h1>
          <p className="text-sm text-muted-foreground">{t("admin.session.unreachableDescription")}</p>
          <code className="rounded bg-muted px-single py-single text-xs">bun nx run os-hub:dev</code>
          <p className="text-sm text-muted-foreground">{t("admin.session.unreachableHint")}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full w-full items-center justify-center">
      <form
        className="flex w-full max-w-sm flex-col gap-single"
        onSubmit={(event) => {
          event.preventDefault();
          if (draft.trim().length > 0) setToken(draft.trim());
        }}
      >
        <h1 className="text-lg font-semibold text-emphasized">{t("admin.session.title")}</h1>
        <p className="text-sm text-muted-foreground">{t("admin.session.description")}</p>
        <label className="flex flex-col gap-single text-sm" htmlFor="admin-session-token">
          {t("admin.session.tokenLabel")}
          <Input id="admin-session-token" type="password" value={draft} placeholder={t("admin.session.tokenPlaceholder")} onChange={(event) => setDraft(event.target.value)} />
        </label>
        {status === "unauthorized" ? <p className="text-sm text-destructive">{t("admin.session.error")}</p> : null}
        <Button id="admin-session-submit" icon="check" text={t("admin.session.submit")} type="submit" disabled={status === "probing"} />
      </form>
    </div>
  );
}
//#endregion 🔖️TokenForm
