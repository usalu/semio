// #region 🧲️Header
// 💻️ hub/modules/admin/elements/📰️EventsPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, Table, uiDataLabel, type TableColumn } from "@semio-tech/ui-react";
import type { DirectoryEvent } from "@semio-tech/framework-os";
import { useAdminT } from "../📚️I18n/🟦️component.tsx";
import { useAdminSession } from "../🔑️AdminSession/🟦️component.tsx";
// #endregion 🔌️Adapters

const PAGE_SIZE = 200;

/** 📰️ `GET /admin/api/events?since=&limit=` — a tail view: `since` starts at `0` and advances to the
 * highest `seq` seen after every load, so "load newer" only ever fetches forward. */
export function EventsPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [events, setEvents] = React.useState<DirectoryEvent[]>([]);
  const [since, setSince] = React.useState(0);

  const load = React.useCallback(() => {
    client.events(since, PAGE_SIZE).then((rows) => {
      if (rows.length === 0) return;
      setEvents((existing) => [...existing, ...rows]);
      setSince(rows.reduce((max, event) => Math.max(max, event.seq), since));
    });
  }, [client, since]);

  const initialLoad = React.useCallback((): void => {
    client.events(0, PAGE_SIZE).then((rows) => {
      setEvents([...rows]);
      setSince(rows.reduce((max, event) => Math.max(max, event.seq), 0));
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
