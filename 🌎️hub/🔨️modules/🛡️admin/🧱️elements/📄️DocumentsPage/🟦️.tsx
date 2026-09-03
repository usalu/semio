// #region 🧲️Header
// 💻️ hub/modules/admin/elements/📄️DocumentsPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Table, uiDataLabel, type TableColumn } from "@semio-tech/ui-react";
import type { DocumentView, SpaceView } from "@semio-tech/framework-os";
import { useAdminT } from "../📚️I18n/🟦️.tsx";
import { useAdminSession } from "../🔑️AdminSession/🟦️.tsx";
// #endregion 🔌️Adapters

const ALL_SPACES = "*";

/** 📄️ `GET /admin/api/documents?space=` scoped by a space selector (`GET /admin/api/spaces` for the
 * option list). `DocumentView` carries no per-document connection count (only `SpaceView.
 * activeConnections`, space-wide), so "active connections" per row is joined client-side against
 * `GET /admin/api/connections` — the same `(spaceId, documentId)` pair `ConnectionView` already
 * carries (contract §C1). */
export function DocumentsPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [spaces, setSpaces] = React.useState<SpaceView[]>([]);
  const [spaceId, setSpaceId] = React.useState<string>(ALL_SPACES);
  const [documents, setDocuments] = React.useState<DocumentView[]>([]);
  const [connectionCounts, setConnectionCounts] = React.useState<Map<string, number>>(new Map());

  React.useEffect(() => {
    client.spaces().then((page) => setSpaces([...page.rows])).catch(() => setSpaces([]));
  }, [client]);

  const load = React.useCallback(() => {
    const scope = spaceId === ALL_SPACES ? undefined : spaceId;
    client.documents(scope).then((page) => setDocuments([...page.rows])).catch(() => setDocuments([]));
    client
      .connections()
      .then((snapshot) => {
        const counts = new Map<string, number>();
        for (const connection of snapshot.rows) {
          const key = `${connection.scope.spaceId}:${connection.scope.documentId}`;
          counts.set(key, (counts.get(key) ?? 0) + 1);
        }
        setConnectionCounts(counts);
      })
      .catch(() => setConnectionCounts(new Map()));
  }, [client, spaceId]);

  React.useEffect(load, [load]);

  const columns: TableColumn<DocumentView>[] = [
    { id: "id", header: t("admin.documents.id"), accessor: (row) => row.descriptor.documentId },
    { id: "headSeq", header: t("admin.documents.headSeq"), accessor: (row) => row.headSeq },
    { id: "commitSeq", header: t("admin.documents.commitSeq"), accessor: (row) => row.commitSeq },
    { id: "epoch", header: t("admin.documents.epoch"), accessor: (row) => row.epoch },
    {
      id: "connections",
      header: t("admin.documents.activeConnections"),
      accessor: (row) => connectionCounts.get(`${row.descriptor.spaceId}:${row.descriptor.documentId}`) ?? 0,
    },
  ];

  return (
    <div className="flex h-full w-full flex-col gap-single overflow-auto p-single">
      <div className="flex items-center justify-between">
        <h1 className="text-lg font-semibold text-emphasized">{t("admin.documents.title")}</h1>
        <Select id="admin-documents-space-select" value={spaceId} onValueChange={setSpaceId}>
          <SelectTrigger id="admin-documents-space" className="w-48">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={ALL_SPACES}>{t("admin.documents.allSpaces")}</SelectItem>
            {spaces.map((space) => (
              <SelectItem key={space.id} value={space.id}>
                {space.name}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      <Table columns={columns} data={documents} emptyMessage={uiDataLabel(t("admin.documents.empty"))} getRowId={(row) => `document:${row.descriptor.spaceId}:${row.descriptor.documentId}`} />
    </div>
  );
}
