// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🙋️UsersPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, Table, uiDataLabel, type TableColumn } from "@semio-tech/ui-react";
import type { UserView } from "@semio-tech/framework-os";
import { useAdminT } from "../📚️I18n/🟦️.tsx";
import { useAdminSession } from "../🔑️AdminSession/🟦️.tsx";
// #endregion 🔌️Adapters

/** 🙋️ `GET /admin/api/users` as a `Table`; each row's "revoke sessions" kicks every LIVE connection
 * for that user (see `AdminClient.revokeUserSessions`'s own doc for the trait-gap this degrades
 * to — a login session that never opened a document WS cannot be revoked from here). */
export function UsersPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [users, setUsers] = React.useState<UserView[] | null>(null);

  const load = React.useCallback(() => {
    client.users().then((page) => setUsers([...page.rows])).catch(() => setUsers([]));
  }, [client]);

  React.useEffect(load, [load]);

  const columns: TableColumn<UserView>[] = [
    { id: "email", header: t("admin.users.email"), accessor: (row) => row.email },
    { id: "displayName", header: t("admin.users.displayName"), accessor: (row) => row.displayName },
    { id: "createdAt", header: t("admin.users.createdAt"), accessor: (row) => new Date(row.createdAtMs).toLocaleString() },
    {
      id: "actions",
      header: t("admin.spaces.actions"),
      accessor: (row) => <Button id={`admin-user-revoke-${row.id}`} icon="trash" text={t("admin.users.revokeSessions")} variant="ghost" onClick={() => client.revokeUserSessions(row.id).then(load)} />,
    },
  ];

  return (
    <div className="flex h-full w-full flex-col gap-single overflow-auto p-single">
      <h1 className="text-lg font-semibold text-emphasized">{t("admin.users.title")}</h1>
      <Table columns={columns} data={users ?? []} emptyMessage={uiDataLabel(t("admin.users.empty"))} getRowId={(row) => `user:${row.id}`} rowDragProps={(row) => ({ "data-row-id": `user:${row.id}` }) as React.HTMLAttributes<HTMLTableRowElement>} />
    </div>
  );
}
