// #region 🧲️Header
// 💻️ hub/modules/admin/elements/📰️EventsPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, Table, uiDataLabel, type TableColumn } from "@semio-tech/ui-react";
import type { DirectoryEvent } from "@semio-tech/framework-os";
import { useAdminT } from "../📚️I18n/🟦️.tsx";
import { useAdminSession } from "../🔑️AdminSession/🟦️.tsx";
// #endregion 🔌️Adapters

/** 📰️ Bounded opaque-cursor event pages. */
export function EventsPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [events, setEvents] = React.useState<DirectoryEvent[]>([]);
  const [cursor, setCursor] = React.useState<string | undefined>();

  const load = React.useCallback(() => {
    client.events(cursor).then((page) => {
      if (page.rows.length === 0) return;
      setEvents((existing) => [...existing, ...page.rows]);
      setCursor(page.nextCursor);
    });
  }, [client, cursor]);

  const initialLoad = React.useCallback((): void => {
    client.events().then((page) => {
      setEvents([...page.rows]);
      setCursor(page.nextCursor);
    });
  }, [client]);

  React.useEffect(initialLoad, [initialLoad]);

  const columns: TableColumn<DirectoryEvent>[] = [
    { id: "seq", header: "#", accessor: (row) => row.seq, width: "4rem" },
    { id: "kind", header: t("admin.events.kind"), accessor: (row) => row.body.kind },
    { id: "actor", header: t("admin.events.actor"), accessor: (row) => `${row.actor.kind}:${row.actor.id}` },
    { id: "time", header: t("admin.events.time"), accessor: (row) => new Date(row.recordedAtMs).toLocaleString() },
  ];

  return (
    <div className="flex h-full w-full flex-col gap-single overflow-auto p-single">
      <div className="flex items-center justify-between">
        <h1 className="text-lg font-semibold text-emphasized">{t("admin.events.title")}</h1>
        <Button id="admin-events-load-more" icon="save" text={t("admin.events.loadMore")} variant="ghost" onClick={load} />
      </div>
      <Table columns={columns} data={events} emptyMessage={uiDataLabel(t("admin.events.empty"))} getRowId={(row) => `event:${row.id}`} />
    </div>
  );
}
