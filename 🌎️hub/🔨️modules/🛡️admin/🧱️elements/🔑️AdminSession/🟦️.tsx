// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🔑️AdminSession/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import type {
  AdminConnectionSnapshotV1,
  AdminIntentReceiptV1,
  AdminIntentV1,
  AdminOperationStatusV1,
  AdminPageV1,
  DirectoryEvent,
  DirectorySpaceKind,
  DirectorySpaceRole,
  DirectorySpaceVisibility,
  DocumentView,
  MemberView,
  SpaceView,
  UserView,
} from "@semio-tech/framework-os";
import { useAdminT } from "../📚️I18n/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️AdminClient
/** 🚨️ Thrown by every {@link AdminClient} call on a non-2xx response — mirrors `DirectoryHttpError`
 * (framework-os's `DirectoryClient`), which this crate cannot reuse: `/admin/api/*` is a route family
 * `DirectoryClient` never covers (contract §C2's admin surface, verified administrator
 * `session.v1` capability). */
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

export type AdminSpaceDetail = SpaceView & { members: AdminPageV1<MemberView> };

/** 📡️ Typed facade over the hub's `/admin/api/*` surface (contract §C2) — the counterpart to
 * `@semio-tech/framework-os`'s `DirectoryClient`, which only covers `/directory/*` and
 * `/auth/sessions/*`. `fetch` only, no external HTTP library (CLAUDE.md). */
export class AdminClient {
  private readonly baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
  }

  private headers(json: boolean): Record<string, string> {
    const headers: Record<string, string> = {};
    if (json) headers["content-type"] = "application/json";
    return headers;
  }

  private async getJson<T>(path: string, signal?: AbortSignal): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, { credentials: "same-origin", headers: this.headers(false), signal });
    if (!response.ok) throw new AdminHttpError(response.status, `admin: GET ${path} failed (${response.status})`);
    return (await response.json()) as T;
  }

  private async postJson<T>(path: string, body?: unknown): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, { method: "POST", credentials: "same-origin", headers: this.headers(true), body: body === undefined ? undefined : JSON.stringify(body) });
    if (!response.ok) throw new AdminHttpError(response.status, `admin: POST ${path} failed (${response.status})`);
    const text = await response.text();
    return (text.length === 0 ? undefined : JSON.parse(text)) as T;
  }

  private requestId(kind: AdminIntentV1["kind"]): string {
    return `${kind}:${globalThis.crypto.randomUUID()}`;
  }

  private async submit(intent: AdminIntentV1): Promise<AdminIntentReceiptV1> {
    const receipt = await this.postJson<AdminIntentReceiptV1>("/admin/api/intents", intent);
    if (
      typeof receipt?.operationId !== "string" ||
      typeof receipt.correlationId !== "string" ||
      !["accepted", "succeeded", "failed", "cancelled"].includes(receipt.state) ||
      typeof receipt.outcome?.code !== "string" ||
      typeof receipt.outcome.durable !== "boolean"
    ) {
      throw new AdminHttpError(502, "admin: malformed intent receipt");
    }
    return receipt;
  }

  private async submitTerminal(intent: AdminIntentV1): Promise<AdminIntentReceiptV1> {
    const receipt = await this.submit(intent);
    if (receipt.state !== "succeeded") throw new AdminHttpError(409, `admin: intent ${intent.kind} ended as ${receipt.state}`);
    return receipt;
  }

  overview(signal?: AbortSignal): Promise<AdminOverview> {
    return this.getJson<AdminOverview>("/admin/api/overview", signal);
  }

  spaces(cursor?: string): Promise<AdminPageV1<SpaceView>> {
    return this.getJson<AdminPageV1<SpaceView>>(`/admin/api/spaces?limit=100${cursor ? `&cursor=${encodeURIComponent(cursor)}` : ""}`);
  }

  space(id: string, cursor?: string): Promise<AdminSpaceDetail> {
    return this.getJson<AdminSpaceDetail>(`/admin/api/spaces/${encodeURIComponent(id)}?limit=100${cursor ? `&cursor=${encodeURIComponent(cursor)}` : ""}`);
  }

  users(cursor?: string): Promise<AdminPageV1<UserView>> {
    return this.getJson<AdminPageV1<UserView>>(`/admin/api/users?limit=100${cursor ? `&cursor=${encodeURIComponent(cursor)}` : ""}`);
  }

  connections(signal?: AbortSignal, cursor?: string): Promise<AdminConnectionSnapshotV1> {
    return this.getJson<AdminConnectionSnapshotV1>(`/admin/api/connections?limit=100${cursor ? `&cursor=${encodeURIComponent(cursor)}` : ""}`, signal);
  }

  documents(spaceId?: string, cursor?: string): Promise<AdminPageV1<DocumentView>> {
    const query = new URLSearchParams({ limit: "100" });
    if (spaceId) query.set("space", spaceId);
    if (cursor) query.set("cursor", cursor);
    return this.getJson<AdminPageV1<DocumentView>>(`/admin/api/documents?${query.toString()}`);
  }

  events(cursor?: string): Promise<AdminPageV1<DirectoryEvent>> {
    return this.getJson<AdminPageV1<DirectoryEvent>>(`/admin/api/events?limit=100${cursor ? `&cursor=${encodeURIComponent(cursor)}` : ""}`);
  }

  createSpace(name: string, spaceKind: DirectorySpaceKind, visibility: DirectorySpaceVisibility): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "create-space", requestId: this.requestId("create-space"), name, spaceKind, visibility });
  }

  renameSpace(spaceId: string, name: string): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "rename-space", requestId: this.requestId("rename-space"), spaceId, name });
  }

  setSpaceVisibility(spaceId: string, visibility: DirectorySpaceVisibility): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "set-space-visibility", requestId: this.requestId("set-space-visibility"), spaceId, visibility });
  }

  archiveSpace(spaceId: string): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "archive-space", requestId: this.requestId("archive-space"), spaceId });
  }

  deleteSpace(spaceId: string): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "delete-space", requestId: this.requestId("delete-space"), spaceId });
  }

  upsertSpaceMember(spaceId: string, email: string, role: DirectorySpaceRole): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "upsert-space-member", requestId: this.requestId("upsert-space-member"), spaceId, email, role });
  }

  removeSpaceMember(spaceId: string, userId: string): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "remove-space-member", requestId: this.requestId("remove-space-member"), spaceId, userId });
  }

  createSpaceInvite(spaceId: string, role: DirectorySpaceRole, ttlSecs: number): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "create-space-invite", requestId: this.requestId("create-space-invite"), spaceId, role, ttlSecs });
  }

  async rebuild(expectedHeadSeq: number): Promise<{ readonly requestId: string; readonly receipt: AdminIntentReceiptV1 }> {
    const requestId = this.requestId("rebuild-directory-projections");
    return { requestId, receipt: await this.submit({ kind: "rebuild-directory-projections", requestId, expectedHeadSeq }) };
  }

  closeConnection(syncSessionId: string, reasonCode = "operator-kick"): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "kick-connection", requestId: this.requestId("kick-connection"), syncSessionId, reasonCode });
  }

  revokeUserSessions(userId: string, reasonCode = "operator-revoke"): Promise<AdminIntentReceiptV1> {
    return this.submitTerminal({ kind: "revoke-user-sessions", requestId: this.requestId("revoke-user-sessions"), userId, reasonCode });
  }

  operation(operationId: string): Promise<AdminOperationStatusV1> {
    return this.getJson<AdminOperationStatusV1>(`/admin/api/operations/${encodeURIComponent(operationId)}`);
  }

  cancelOperation(operationId: string): Promise<AdminOperationStatusV1> {
    return this.postJson<AdminOperationStatusV1>(`/admin/api/operations/${encodeURIComponent(operationId)}/cancel`);
  }
}
//#endregion 🔖️AdminClient

//#region 🔖️Session
/** @emoji 🚦️ `unreachable` is deliberately distinct from `unauthorized`: a hub that is not running at
 * all fails the probe with a transport error, and reporting that as "your token was rejected" sends
 * the operator hunting for a credential when the actual fix is to start the hub. */
export type AdminSessionStatus = "probing" | "authorized" | "unauthorized" | "unreachable";

export interface AdminSessionState {
  status: AdminSessionStatus;
  client: AdminClient;
}

const AdminSessionContext = React.createContext<AdminSessionState | null>(null);

/** 🔑️ Exchanges one exact fragment nonce for the relay's host-only HttpOnly cookie, clears the
 * fragment before probing, and leaves the administrator capability exclusively in the local relay. */
export function AdminSessionProvider({ baseUrl, children }: { readonly baseUrl: string; readonly children: React.ReactNode }): React.ReactElement {
  const [status, setStatus] = React.useState<AdminSessionStatus>("probing");
  const client = React.useMemo(() => new AdminClient(baseUrl), [baseUrl]);

  React.useEffect(() => {
    let cancelled = false;
    const controller = new AbortController();
    setStatus("probing");
    const authorize = async (): Promise<void> => {
      const hash = typeof window === "undefined" ? "" : window.location.hash;
      const match = /^#semio-admin=([0-9a-f]{64})$/u.exec(hash);
      if (hash.length > 0) window.history.replaceState(window.history.state, "", `${window.location.pathname}${window.location.search}`);
      if (hash.length > 0 && !match) throw new AdminHttpError(401, "admin bootstrap fragment invalid");
      if (match) {
        const response = await fetch(`${baseUrl.replace(/\/+$/, "")}/__semio/admin/bootstrap`, {
          method: "POST",
          credentials: "same-origin",
          headers: { "x-semio-admin-bootstrap": match[1]! },
          signal: controller.signal,
        });
        if (!response.ok) throw new AdminHttpError(response.status, `admin bootstrap failed (${response.status})`);
      }
      await client.overview(controller.signal);
    };
    authorize()
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
      controller.abort();
    };
  }, [client]);

  const value = React.useMemo<AdminSessionState>(() => ({ status, client }), [status, client]);
  return <AdminSessionContext.Provider value={value}>{children}</AdminSessionContext.Provider>;
}

export function useAdminSession(): AdminSessionState {
  const state = React.useContext(AdminSessionContext);
  if (!state) throw new Error("useAdminSession must be used within AdminSessionProvider");
  return state;
}
//#endregion 🔖️Session

//#region 🔖️AccessGate
/** 🚪️ Accessible fail-closed status for an absent, rejected, or unreachable local admin relay. */
export function AdminAccessGate(): React.ReactElement {
  const t = useAdminT();
  const { status } = useAdminSession();

  // 🔌️ No hub answering: a token form here would be a dead end, so show what is actually wrong and the
  // command that fixes it instead of prompting for a credential that cannot help.
  if (status === "unreachable") {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div id="admin-session-unreachable" className="flex w-full max-w-md flex-col gap-single" role="alert" aria-live="assertive">
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
      <div className="flex w-full max-w-md flex-col gap-single" role={status === "probing" ? "status" : "alert"} aria-live={status === "probing" ? "polite" : "assertive"}>
        <h1 className="text-lg font-semibold text-emphasized">{t("admin.session.title")}</h1>
        <p className={status === "unauthorized" ? "text-sm text-destructive" : "text-sm text-muted-foreground"}>{t(status === "probing" ? "admin.session.probing" : "admin.session.error")}</p>
        {status === "unauthorized" ? <code className="rounded bg-muted px-single py-single text-xs">bun nx run os-hub:dev-secure-admin</code> : null}
      </div>
    </div>
  );
}
//#endregion 🔖️AccessGate
