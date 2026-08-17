// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🔴️ConnectionsPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button } from "@semio-tech/ui-react";
import { DirectoryClient, isDirectoryStreamMessageKind, type ConnectionView } from "@semio-tech/framework-os";
import { useAdminT } from "../📚️I18n/🟦️component.tsx";
import { useAdminSession } from "../🔑️AdminSession/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Grouping
type ConnectionGroup = Map<string, Map<string, Map<string, ConnectionView[]>>>;

/** 🧮️ `space -> document -> surface -> actor[]` — pure grouping over the live connection set, so the
 * component body only ever renders, never mutates. */
function groupConnections(connections: readonly ConnectionView[]): ConnectionGroup {
  const bySpace: ConnectionGroup = new Map();
  for (const connection of connections) {
    const byDocument = bySpace.get(connection.spaceId) ?? new Map();
    bySpace.set(connection.spaceId, byDocument);
    const bySurface = byDocument.get(connection.documentId) ?? new Map();
    byDocument.set(connection.documentId, bySurface);
    const list = bySurface.get(connection.surface || "—") ?? [];
    list.push(connection);
    bySurface.set(connection.surface || "—", list);
  }
  return bySpace;
}
//#endregion 🔖️Grouping

/** 🔴️ Live view of every currently-open document-WS connection, grouped `space -> document ->
 * surface -> actor` (contract §C1's `ConnectionView`). Seeded from `GET /admin/api/connections`
 * (a snapshot), then kept live by subscribing to `/directory/ws` through `@semio-tech/framework-os`'s
 * `DirectoryClient.stream()` — the SAME channel `POST /directory/commands`'s event replay uses, but
 * `DirectoryStreamMessage::Connection{phase,...}` frames are broadcast-only (never persisted/replayed,
 * see the hub's own `handle_ws`/`handle_directory_ws` doc), so this page reacts to them as they
 * happen rather than polling. `since` is seeded from the admin overview's `headSeq` so the WS replay
 * phase has nothing stale to resend — this page only cares about live `Connection` frames. */
export function ConnectionsPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [connections, setConnections] = React.useState<Map<string, ConnectionView>>(new Map());
  const [live, setLive] = React.useState(false);

  React.useEffect(() => {
    let cancelled = false;
    client.connections().then((rows) => {
      if (cancelled) return;
      setConnections(new Map(rows.map((row) => [row.syncSessionId, row])));
    });

    let stream: { close: () => void } | undefined;
    client.overview().then((overview) => {
      if (cancelled) return;
      const directory = new DirectoryClient(window.location.origin);
      stream = directory.stream(overview.headSeq, (message) => {
        if (!isDirectoryStreamMessageKind(message, "connection")) return;
        setLive(true);
        setConnections((existing) => {
          const next = new Map(existing);
          if (message.phase === "opened") next.set(message.connection.syncSessionId, message.connection);
          else next.delete(message.connection.syncSessionId);
          return next;
        });
      });
    });

    return () => {
      cancelled = true;
      stream?.close();
    };
  }, [client]);

  const grouped = React.useMemo(() => groupConnections([...connections.values()]), [connections]);
  const isEmpty = connections.size === 0;

  return (
    <div className="flex h-full w-full flex-col gap-single overflow-auto p-single">
      <div className="flex items-center gap-single">
        <h1 className="text-lg font-semibold text-emphasized">{t("admin.connections.title")}</h1>
        <span className={`text-xs ${live ? "text-emphasized" : "text-muted-foreground"}`} data-slot="admin-connections-live-indicator">
          {live ? t("admin.connections.live") : t("admin.connections.offline")}
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
            {[...byDocument.entries()].map(([documentId, bySurface]) => (
              <div key={documentId} className="ml-single flex flex-col gap-single">
                <h3 className="text-sm text-emphasized">
                  {t("admin.connections.document")}: {documentId}
                </h3>
                {[...bySurface.entries()].map(([surface, rows]) => (
                  <div key={surface} className="ml-single flex flex-col gap-single">
                    <h4 className="text-xs text-muted-foreground">
                      {t("admin.connections.surface")}: {surface}
                    </h4>
                    {rows.map((row) => (
                      <div key={row.syncSessionId} className="ml-single flex items-center justify-between gap-single text-sm" data-row-id={`connection:${row.syncSessionId}`}>
                        <span>
                          {t("admin.connections.actor")}: {row.actor}
                          {row.email ? ` (${row.email})` : ""} · {row.role}
                        </span>
                        <Button id={`admin-connection-kick-${row.syncSessionId}`} icon="x" text={t("admin.connections.kick")} variant="ghost" onClick={() => client.closeConnection(row.syncSessionId)} />
                      </div>
                    ))}
                  </div>
                ))}
              </div>
            ))}
          </div>
        ))
      )}
    </div>
  );
}
