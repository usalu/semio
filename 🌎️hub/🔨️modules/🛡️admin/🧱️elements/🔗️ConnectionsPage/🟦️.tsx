// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🔗️ConnectionsPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button } from "@semio-tech/ui-react";
import type { AdminRecordedConnectionV1 } from "@semio-tech/framework-os";
import { useAdminT } from "../📚️I18n/🟦️.tsx";
import { useAdminSession } from "../🔑️AdminSession/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Grouping
type ConnectionGroup = Map<string, Map<string, AdminRecordedConnectionV1[]>>;

/** 🧮️ `space -> document -> recorded bindings`, without synthesized actor or surface claims. */
function groupConnections(connections: readonly AdminRecordedConnectionV1[]): ConnectionGroup {
  const bySpace: ConnectionGroup = new Map();
  for (const connection of connections) {
    const byDocument = bySpace.get(connection.scope.spaceId) ?? new Map();
    bySpace.set(connection.scope.spaceId, byDocument);
    const list = byDocument.get(connection.scope.documentId) ?? [];
    list.push(connection);
    byDocument.set(connection.scope.documentId, list);
  }
  return bySpace;
}
//#endregion 🔖️Grouping

const CONNECTION_POLL_MS = 2_000;
const CONNECTION_POLL_DEADLINE_MS = 2_000;

/** 🔴️ Bounded authenticated REST snapshots of every currently-open document connection. */
export function ConnectionsPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [connections, setConnections] = React.useState<Map<string, AdminRecordedConnectionV1>>(new Map());
  const [fresh, setFresh] = React.useState(false);

  React.useEffect(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let controller: AbortController | undefined;
    const poll = async (): Promise<void> => {
      controller = new AbortController();
      const deadline = setTimeout(() => controller?.abort(), CONNECTION_POLL_DEADLINE_MS);
      try {
        const snapshot = await client.connections(controller.signal);
        if (cancelled) return;
        setConnections(new Map(snapshot.rows.map((row) => [row.syncSessionId, row])));
        setFresh(true);
      } catch {
        if (!cancelled) setFresh(false);
      } finally {
        clearTimeout(deadline);
        controller = undefined;
        if (!cancelled) timer = setTimeout(poll, CONNECTION_POLL_MS);
      }
    };
    void poll();

    return () => {
      cancelled = true;
      if (timer) clearTimeout(timer);
      controller?.abort();
    };
  }, [client]);

  const grouped = React.useMemo(() => groupConnections([...connections.values()]), [connections]);
  const isEmpty = connections.size === 0;

  return (
    <div className="flex h-full w-full flex-col gap-single overflow-auto p-single">
      <div className="flex items-center gap-single">
        <h1 className="text-lg font-semibold text-emphasized">{t("admin.connections.title")}</h1>
        <span className={`text-xs ${fresh ? "text-emphasized" : "text-muted-foreground"}`} role="status" data-slot="admin-connections-freshness">
          {fresh ? t("admin.connections.fresh") : t("admin.connections.stale")}
        </span>
      </div>
      {isEmpty ? (
        <p className="text-sm text-muted-foreground">{t("admin.connections.empty")}</p>
      ) : (
        [...grouped.entries()].map(([spaceId, byDocument]) => (
          <div key={spaceId} className="flex flex-col gap-single rounded border p-single">
            <h2 className="text-sm font-semibold text-emphasized">
              {t("admin.connections.space")}: {spaceId}
            </h2>
            {[...byDocument.entries()].map(([documentId, rows]) => (
              <div key={documentId} className="ml-single flex flex-col gap-single">
                <h3 className="text-sm text-emphasized">
                  {t("admin.connections.document")}: {documentId}
                </h3>
                <div className="ml-single flex flex-col gap-single">
                  {rows.map((row) => (
                    <div key={row.syncSessionId} className="ml-single flex items-center justify-between gap-single text-sm" data-row-id={`connection:${row.syncSessionId}`}>
                      <span>
                        {row.email ?? row.authenticatedUserId ?? "—"}
                        {row.role ? ` · ${row.role}` : ""}
                      </span>
                      <Button id={`admin-connection-kick-${row.syncSessionId}`} icon="x" text={t("admin.connections.kick")} variant="ghost" onClick={() => client.closeConnection(row.syncSessionId)} />
                    </div>
                  ))}
                  </div>
              </div>
            ))}
          </div>
        ))
      )}
    </div>
  );
}
