// #region 🧲️Header
// 💻️ hub/modules/admin/elements/📊️ObservabilityPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, Table, uiDataLabel, type TableColumn } from "@semio-tech/ui-react";
import type { HubDbIoKindV1, HubObservabilityV1, HubRouteLatencyV1, HubTraceEventRowV1, TrustedCatalogPackageProgressV1 } from "os-hub-ts/observability";
import { AdminNote, AdminPage, AdminSection } from "../📄️AdminPage/🟦️.tsx";
import { StatTiles } from "../📊️StatTiles/🟦️.tsx";
import { useAdminT, type AdminI18nKey } from "../📚️I18n/🟦️.tsx";
import { useAdminSession } from "../🔑️AdminSession/🟦️.tsx";
// #endregion 🔌️Adapters

/** ⏱️ How often the page re-reads `GET /admin/api/observability` while it is shown. */
const OBSERVABILITY_REFRESH_MS = 5_000;

/** 🔢️ Bytes in binary units. */
function formatBytes(bytes: number): string {
  const units = ["B", "KiB", "MiB", "GiB"];
  let value = bytes;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  return `${unit === 0 ? value : value.toFixed(1)} ${units[unit]}`;
}

/** ⏱️ Microseconds as the shortest readable unit. */
function formatMicros(micros: number): string {
  if (micros < 1_000) return `${micros} µs`;
  if (micros < 1_000_000) return `${(micros / 1_000).toFixed(1)} ms`;
  return `${(micros / 1_000_000).toFixed(2)} s`;
}

/** 📊️ `GET /admin/api/observability` as a live operator page: compiled-guest residency, the trusted catalog's
 * per-package load and verification, per-route answers and latency, trace events and the DB I/O census,
 * re-read every {@link OBSERVABILITY_REFRESH_MS} ms and on demand. */
export function ObservabilityPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [reading, setReading] = React.useState<HubObservabilityV1 | null>(null);
  const [failed, setFailed] = React.useState(false);

  const load = React.useCallback((signal?: AbortSignal) => {
    client
      .observability(signal)
      .then((next) => {
        setReading(next);
        setFailed(false);
      })
      .catch((error: unknown) => {
        if (!(error instanceof DOMException && error.name === "AbortError")) setFailed(true);
      });
  }, [client]);

  React.useEffect(() => {
    const cancel = new AbortController();
    load(cancel.signal);
    const timer = setInterval(() => load(cancel.signal), OBSERVABILITY_REFRESH_MS);
    return () => {
      clearInterval(timer);
      cancel.abort();
    };
  }, [load]);

  const packageColumns: TableColumn<TrustedCatalogPackageProgressV1>[] = [
    { id: "package", header: t("admin.observability.catalogPackage"), accessor: (row) => row.pluginId },
    { id: "phase", header: t("admin.observability.catalogPhase"), accessor: (row) => t(`admin.observability.phase.${row.phase}` as AdminI18nKey) },
    { id: "bytes", header: t("admin.observability.catalogBytes"), accessor: (row) => formatBytes(row.componentBytes) },
    { id: "rows", header: t("admin.observability.catalogRows"), accessor: (row) => `${row.rowsPinned} / ${row.rowsVerified} / ${row.rows}` },
  ];
  const routeColumns: TableColumn<HubRouteLatencyV1>[] = [
    { id: "route", header: t("admin.observability.route"), accessor: (row) => `${row.method} ${row.route}` },
    { id: "requests", header: t("admin.observability.requests"), accessor: (row) => row.requests, width: "6rem" },
    { id: "refusals", header: t("admin.observability.refusals"), accessor: (row) => `${row.clientRefusals} / ${row.rateLimited} / ${row.unavailable}` },
    { id: "failures", header: t("admin.observability.failures"), accessor: (row) => row.serverFailures, width: "6rem" },
    { id: "latency", header: t("admin.observability.latency"), accessor: (row) => [row.p50Us, row.p95Us, row.p99Us, row.maxUs].map(formatMicros).join(" / ") },
  ];
  const eventColumns: TableColumn<HubTraceEventRowV1>[] = [
    { id: "event", header: t("admin.observability.event"), accessor: (row) => row.event },
    { id: "outcomes", header: t("admin.observability.outcomes"), accessor: (row) => `${row.ok} / ${row.refused} / ${row.failed} / ${row.cancelled}` },
    { id: "latency", header: t("admin.observability.latency"), accessor: (row) => [row.p50Us, row.p95Us, row.p99Us].map(formatMicros).join(" / ") },
  ];
  const dbIoColumns: TableColumn<HubDbIoKindV1>[] = [
    { id: "kind", header: t("admin.observability.dbIoKind"), accessor: (row) => row.kind },
    { id: "counts", header: t("admin.observability.dbIoCounts"), accessor: (row) => `${row.tasks} / ${row.steps} / ${row.turns}` },
    { id: "admission", header: t("admin.observability.dbIoAdmission"), accessor: (row) => `${row.admissionWaits} / ${formatMicros(row.admissionWaitMicros)} / ${row.admissionRefusals}` },
  ];
  const routes = reading ? [...reading.routes].sort((left, right) => right.p95Us - left.p95Us) : [];

  return (
    <AdminPage title={t("admin.observability.title")} data-slot="admin-observability" actions={<Button id="admin-observability-refresh" icon="save" text={t("admin.observability.refresh")} variant="ghost" onClick={() => load()} />}>
      {failed && <AdminNote role="alert">{t("admin.observability.unavailable")}</AdminNote>}
      {reading && (
        <>
          <StatTiles columns={2} tiles={[
            { label: t("admin.observability.uptime"), value: formatMicros(reading.uptimeMs * 1_000) },
            { label: t("admin.observability.droppedEvents"), value: reading.droppedEvents },
          ]} />
          <AdminSection id="admin-observability-residency" title={t("admin.observability.residencyTitle")}>
            {reading.residency ? (
              <StatTiles columns={4} tiles={[
                { label: t("admin.observability.residencyResident"), value: `${reading.residency.residentGuests} / ${reading.residency.registeredGuests} · ${formatBytes(reading.residency.residentBytes)}` },
                { label: t("admin.observability.residencyBudget"), value: formatBytes(reading.residency.budgetBytes) },
                { label: t("admin.observability.residencyFootprint"), value: formatBytes(reading.residency.footprintBytes) },
                { label: t("admin.observability.residencyHits"), value: reading.residency.hits },
                { label: t("admin.observability.residencyCompiles"), value: reading.residency.compiles },
                { label: t("admin.observability.residencyAdmitted"), value: reading.residency.admitted },
                { label: t("admin.observability.residencyBypassed"), value: reading.residency.bypassed },
                { label: t("admin.observability.residencyReleased"), value: reading.residency.released },
                { label: t("admin.observability.residencyCompileTime"), value: formatMicros(reading.residency.compileMicros) },
              ]} />
            ) : (
              <AdminNote>{t("admin.observability.residencyNone")}</AdminNote>
            )}
          </AdminSection>
          {reading.catalog && (
            <AdminSection id="admin-observability-catalog" title={t("admin.observability.catalogTitle")}>
              <AdminNote className="text-sm">
                {t("admin.observability.catalogSummary", { ready: reading.catalog.packagesReady, total: reading.catalog.packagesTotal, refused: reading.catalog.packagesRefused, pinned: reading.catalog.rowsPinned, verified: reading.catalog.rowsVerified, rows: reading.catalog.rowsTotal })}
              </AdminNote>
              <Table columns={packageColumns} data={[...reading.catalog.packages]} emptyMessage={uiDataLabel(t("admin.observability.residencyNone"))} getRowId={(row) => `package:${row.pluginId}`} />
            </AdminSection>
          )}
          <AdminSection id="admin-observability-routes" title={t("admin.observability.routesTitle")}>
            {reading.routesOverflowed > 0 && <AdminNote>{t("admin.observability.routesOverflowed", { count: reading.routesOverflowed })}</AdminNote>}
            <Table columns={routeColumns} data={routes} emptyMessage={uiDataLabel(t("admin.observability.requests"))} getRowId={(row) => `route:${row.method}:${row.route}`} />
          </AdminSection>
          <AdminSection id="admin-observability-events" title={t("admin.observability.eventsTitle")}>
            <Table columns={eventColumns} data={[...reading.rows]} emptyMessage={uiDataLabel(t("admin.observability.eventsTitle"))} getRowId={(row) => `event:${row.event}`} />
          </AdminSection>
          <AdminSection id="admin-observability-db-io" title={t("admin.observability.dbIoTitle")}>
            <AdminNote className="text-sm">{t("admin.observability.dbIoSummary", { tasks: reading.dbIo.tasks, steps: reading.dbIo.steps, turns: reading.dbIo.turns })}</AdminNote>
            <AdminNote className="text-sm">{t("admin.observability.dbIoAdmissionSummary", { waits: reading.dbIo.admissionWaits, waited: formatMicros(reading.dbIo.admissionWaitMicros), refusals: reading.dbIo.admissionRefusals })}</AdminNote>
            <Table columns={dbIoColumns} data={[...reading.dbIo.kinds]} emptyMessage={uiDataLabel(t("admin.observability.dbIoTitle"))} getRowId={(row) => `db-io:${row.kind}`} />
          </AdminSection>
        </>
      )}
    </AdminPage>
  );
}
