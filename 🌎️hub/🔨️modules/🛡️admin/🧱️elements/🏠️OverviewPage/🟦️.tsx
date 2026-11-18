// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🏠️OverviewPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, SharedBorderRow, SharedBorderRowCell } from "@semio-tech/ui-react";
import type { AdminOperationStatusV1 } from "@semio-tech/framework-os";
import { AdminPage } from "../📄️AdminPage/🟦️.tsx";
import { StatTiles } from "../📊️StatTiles/🟦️.tsx";
import { useAdminT } from "../📚️I18n/🟦️.tsx";
import { useAdminSession, type AdminOverview } from "../🔑️AdminSession/🟦️.tsx";
// #endregion 🔌️Adapters

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let value = bytes;
  let unitIndex = -1;
  do {
    value /= 1024;
    unitIndex += 1;
  } while (value >= 1024 && unitIndex < units.length - 1);
  return `${value.toFixed(1)} ${units[unitIndex]}`;
}

/** 🏠️ `GET /admin/api/overview` rendered as a stat-tile grid plus a typed rebuild intent. */
export function OverviewPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [overview, setOverview] = React.useState<AdminOverview | null>(null);
  const [rebuilding, setRebuilding] = React.useState(false);
  const [rebuildOperationId, setRebuildOperationId] = React.useState<string | null>(null);
  const [rebuildStatus, setRebuildStatus] = React.useState<AdminOperationStatusV1 | null>(null);
  const [message, setMessage] = React.useState<string | null>(null);

  const load = React.useCallback(() => {
    client.overview().then(setOverview).catch(() => setOverview(null));
  }, [client]);

  React.useEffect(load, [load]);

  const rebuild = React.useCallback(() => {
    if (!overview) return;
    setRebuilding(true);
    setMessage(null);
    client
      .rebuild(overview.headSeq)
      .then(({ receipt }) => {
        setRebuildOperationId(receipt.operationId);
        setRebuildStatus({ receipt });
        if (receipt.state !== "accepted") {
          setRebuilding(false);
          setMessage(receipt.state === "succeeded" ? t("admin.overview.rebuildSuccess", { count: 0 }) : t("admin.overview.rebuildError"));
          load();
        }
      })
      .catch(() => {
        setRebuilding(false);
        setMessage(t("admin.overview.rebuildError"));
      });
  }, [client, load, overview, t]);

  React.useEffect(() => {
    if (!rebuilding || !rebuildOperationId) return;
    let cancelled = false;
    const timer = setInterval(() => {
      client
        .operation(rebuildOperationId)
        .then((status) => {
          if (cancelled) return;
          setRebuildStatus(status);
          if (status.receipt.state !== "accepted") {
            clearInterval(timer);
            setRebuilding(false);
            setMessage(status.receipt.state === "succeeded" ? t("admin.overview.rebuildSuccess", { count: status.progress?.completedEvents ?? 0 }) : t("admin.overview.rebuildError"));
            load();
          }
        })
        .catch(() => undefined);
    }, 250);
    return () => {
      cancelled = true;
      clearInterval(timer);
    };
  }, [client, load, rebuildOperationId, rebuilding, t]);

  if (!overview) return <div className="px-single py-1 text-sm text-muted-foreground">{t("admin.session.probing")}</div>;

  const backends = Object.entries(overview.backends)
    .filter(([, enabled]) => enabled)
    .map(([name]) => name)
    .join(", ");

  return (
    <AdminPage title={t("admin.overview.title")} data-slot="admin-overview">
      <StatTiles columns={3} tiles={[
        { label: t("admin.overview.spaces"), value: overview.counts.spaces },
        { label: t("admin.overview.users"), value: overview.counts.users },
        { label: t("admin.overview.connections"), value: overview.counts.connections },
        { label: t("admin.overview.openArtifacts"), value: overview.openArtifacts },
        { label: t("admin.overview.headSeq"), value: overview.headSeq },
        { label: t("admin.overview.dataDirBytes"), value: formatBytes(overview.dataDirBytes) },
        { label: t("admin.overview.backends"), value: backends || "—" },
      ]} />
      <SharedBorderRow edges={["bottom"]} data-slot="admin-overview-rebuild-row">
          <Button id="admin-overview-rebuild" icon="save" text={rebuilding ? t("admin.overview.rebuilding") : t("admin.overview.rebuild")} disabled={rebuilding} onClick={rebuild} />
          {rebuilding && rebuildOperationId ? (
            <Button id="admin-overview-rebuild-cancel" icon="x" text={t("admin.spaces.cancel")} variant="ghost" onClick={() => client.cancelOperation(rebuildOperationId)} />
          ) : null}
        {rebuildStatus?.progress ? (
          <SharedBorderRowCell className="text-sm text-muted-foreground" role="status">
            {rebuildStatus.progress.completedEvents}/{rebuildStatus.progress.totalEvents}
          </SharedBorderRowCell>
        ) : null}
        {message ? <SharedBorderRowCell className="text-sm text-muted-foreground">{message}</SharedBorderRowCell> : null}
      </SharedBorderRow>
    </AdminPage>
  );
}
