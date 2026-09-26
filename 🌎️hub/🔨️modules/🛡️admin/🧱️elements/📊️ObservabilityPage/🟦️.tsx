// #region 🧲️Header
// 💻️ hub/modules/admin/elements/📊️ObservabilityPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, Table, uiDataLabel, type TableColumn } from "@semio-tech/ui-react";
import type { HubDbIoKindV1, HubObservabilityV1, HubRouteLatencyV1, HubTraceEventRowV1, TrustedCatalogPackageProgressV1 } from "../../../../📊️observability/🟦️.ts";
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

/** 📊️ One labelled figure. */
function Figure({ label, value }: { readonly label: string; readonly value: React.ReactNode }): React.ReactElement {
  return (
    <div className="flex flex-col gap-single rounded border p-single">
      <span className="text-sm text-muted-foreground">{label}</span>
      <span className="text-lg font-semibold text-emphasized">{value}</span>
    </div>
  );
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
    <div className="flex h-full w-full flex-col gap-single overflow-auto p-single">
      <div className="flex items-center justify-between">
        <h1 className="text-lg font-semibold text-emphasized">{t("admin.observability.title")}</h1>
        <Button id="admin-observability-refresh" icon="save" text={t("admin.observability.refresh")} variant="ghost" onClick={() => load()} />
      </div>
      {failed && (
        <p role="alert" className="text-sm text-muted-foreground">
          {t("admin.observability.unavailable")}
        </p>
      )}
      {reading && (
        <>
          <div className="grid grid-cols-2 gap-single md:grid-cols-4">
            <Figure label={t("admin.observability.uptime")} value={formatMicros(reading.uptimeMs * 1_000)} />
            <Figure label={t("admin.observability.droppedEvents")} value={reading.droppedEvents} />
          </div>
          <section aria-labelledby="admin-observability-residency" className="flex flex-col gap-single">
            <h2 id="admin-observability-residency" className="font-semibold">{t("admin.observability.residencyTitle")}</h2>
            {reading.residency ? (
              <div className="grid grid-cols-2 gap-single md:grid-cols-4">
                <Figure label={t("admin.observability.residencyResident")} value={`${reading.residency.residentGuests} / ${reading.residency.registeredGuests} · ${formatBytes(reading.residency.residentBytes)}`} />
                <Figure label={t("admin.observability.residencyBudget")} value={formatBytes(reading.residency.budgetBytes)} />
                <Figure label={t("admin.observability.residencyHits")} value={reading.residency.hits} />
                <Figure label={t("admin.observability.residencyCompiles")} value={reading.residency.compiles} />
                <Figure label={t("admin.observability.residencyAdmitted")} value={reading.residency.admitted} />
                <Figure label={t("admin.observability.residencyBypassed")} value={reading.residency.bypassed} />
                <Figure label={t("admin.observability.residencyReleased")} value={reading.residency.released} />
                <Figure label={t("admin.observability.residencyCompileTime")} value={formatMicros(reading.residency.compileMicros)} />
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">{t("admin.observability.residencyNone")}</p>
            )}
          </section>
          {reading.catalog && (
            <section aria-labelledby="admin-observability-catalog" className="flex flex-col gap-single">
              <h2 id="admin-observability-catalog" className="font-semibold">{t("admin.observability.catalogTitle")}</h2>
              <p className="text-sm">
                {t("admin.observability.catalogSummary", { ready: reading.catalog.packagesReady, total: reading.catalog.packagesTotal, refused: reading.catalog.packagesRefused, pinned: reading.catalog.rowsPinned, verified: reading.catalog.rowsVerified, rows: reading.catalog.rowsTotal })}
              </p>
              <Table columns={packageColumns} data={[...reading.catalog.packages]} emptyMessage={uiDataLabel(t("admin.observability.residencyNone"))} getRowId={(row) => `package:${row.pluginId}`} />
            </section>
          )}
          <section aria-labelledby="admin-observability-routes" className="flex flex-col gap-single">
            <h2 id="admin-observability-routes" className="font-semibold">{t("admin.observability.routesTitle")}</h2>
            {reading.routesOverflowed > 0 && <p className="text-sm text-muted-foreground">{t("admin.observability.routesOverflowed", { count: reading.routesOverflowed })}</p>}
            <Table columns={routeColumns} data={routes} emptyMessage={uiDataLabel(t("admin.observability.requests"))} getRowId={(row) => `route:${row.method}:${row.route}`} />
          </section>
          <section aria-labelledby="admin-observability-events" className="flex flex-col gap-single">
            <h2 id="admin-observability-events" className="font-semibold">{t("admin.observability.eventsTitle")}</h2>
            <Table columns={eventColumns} data={[...reading.rows]} emptyMessage={uiDataLabel(t("admin.observability.eventsTitle"))} getRowId={(row) => `event:${row.event}`} />
          </section>
          <section aria-labelledby="admin-observability-db-io" className="flex flex-col gap-single">
            <h2 id="admin-observability-db-io" className="font-semibold">{t("admin.observability.dbIoTitle")}</h2>
            <p className="text-sm">{t("admin.observability.dbIoSummary", { tasks: reading.dbIo.tasks, steps: reading.dbIo.steps, turns: reading.dbIo.turns })}</p>
            <p className="text-sm">{t("admin.observability.dbIoAdmissionSummary", { waits: reading.dbIo.admissionWaits, waited: formatMicros(reading.dbIo.admissionWaitMicros), refusals: reading.dbIo.admissionRefusals })}</p>
            <Table columns={dbIoColumns} data={[...reading.dbIo.kinds]} emptyMessage={uiDataLabel(t("admin.observability.dbIoTitle"))} getRowId={(row) => `db-io:${row.kind}`} />
          </section>
        </>
      )}
    </div>
  );
}
