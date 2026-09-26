// #region 🧲️Header
/** 📊️ TypeScript twin of `HubObservabilityV1` ([`🔣️.json`](🧬️schema/🔣️.json), Rust projection `🦀️.rs`): what
 * `GET /admin/api/observability` answers an administrator. Pure data; the Ajv oracle
 * (`🌎️hub/🧪️tests/📊️observability/🟦️.ts`) holds the fixture body against the schema. */
// #endregion 🧲️Header

/** 📝️ One trace event's counters since boot. */
export interface HubTraceEventRowV1 {
  readonly event: string;
  readonly started: number;
  readonly ok: number;
  readonly refused: number;
  readonly failed: number;
  readonly cancelled: number;
  readonly total: number;
  readonly samples: number;
  readonly p50Us: number;
  readonly p95Us: number;
  readonly p99Us: number;
}

/** 🛣️ One registered route's answers by class and its latency percentiles, keyed by method and template. */
export interface HubRouteLatencyV1 {
  readonly method: string;
  readonly route: string;
  readonly requests: number;
  readonly successes: number;
  readonly clientRefusals: number;
  readonly rateLimited: number;
  readonly unavailable: number;
  readonly serverFailures: number;
  readonly samples: number;
  readonly p50Us: number;
  readonly p95Us: number;
  readonly p99Us: number;
  readonly maxUs: number;
}

/** 🧊️ The compiled-guest residency (`TrustedCatalogGuestResidencyStateV1`). */
export interface TrustedCatalogGuestResidencyStateV1 {
  readonly budgetBytes: number;
  readonly registeredGuests: number;
  readonly residentGuests: number;
  readonly residentBytes: number;
  readonly hits: number;
  readonly compiles: number;
  readonly admitted: number;
  readonly bypassed: number;
  readonly released: number;
  readonly compileMicros: number;
}

/** 🚦️ Where one selected package stands (`TrustedCatalogPackagePhaseV1`). */
export type TrustedCatalogPackagePhaseV1 = "pending" | "reading" | "staged" | "verifying" | "ready" | "refused";

/** 📦️ One package's place in its catalog's load and verification. */
export interface TrustedCatalogPackageProgressV1 {
  readonly pluginId: string;
  readonly componentBytes: number;
  readonly phase: TrustedCatalogPackagePhaseV1;
  readonly rows: number;
  readonly rowsPinned: number;
  readonly rowsVerified: number;
}

/** 📈️ How far the trusted catalog's load and codec-row verification have come, counts only. */
export interface TrustedCatalogLoadProgressV1 {
  readonly packages: readonly TrustedCatalogPackageProgressV1[];
  readonly packagesTotal: number;
  readonly packagesReady: number;
  readonly packagesRefused: number;
  readonly componentBytesTotal: number;
  readonly componentBytesRead: number;
  readonly rowsTotal: number;
  readonly rowsPinned: number;
  readonly rowsVerified: number;
}

/** 🗄️ The DB I/O one task kind cost since the process started, and its admission waits. */
export interface HubDbIoKindV1 {
  readonly kind: string;
  readonly tasks: number;
  readonly steps: number;
  readonly turns: number;
  readonly admissionWaits: number;
  readonly admissionWaitMicros: number;
  readonly admissionRefusals: number;
}

/** 🗄️ The process-wide DB I/O census. */
export interface HubDbIoCensusV1 {
  readonly tasks: number;
  readonly steps: number;
  readonly turns: number;
  readonly admissionWaits: number;
  readonly admissionWaitMicros: number;
  readonly admissionRefusals: number;
  readonly kinds: readonly HubDbIoKindV1[];
}

/** 📊️ One administrator's reading of a hub. */
export interface HubObservabilityV1 {
  readonly schema: "semio.hub.observability/v1";
  readonly level: "off" | "error" | "warn" | "info" | "debug";
  readonly uptimeMs: number;
  readonly droppedEvents: number;
  readonly declaredEvents: readonly string[];
  readonly rows: readonly HubTraceEventRowV1[];
  readonly routes: readonly HubRouteLatencyV1[];
  readonly routesOverflowed: number;
  readonly residency: TrustedCatalogGuestResidencyStateV1 | null;
  readonly catalog: TrustedCatalogLoadProgressV1 | null;
  readonly dbIo: HubDbIoCensusV1;
}

/** 🏷️ The `schema` of every {@link HubObservabilityV1}. */
export const HUB_OBSERVABILITY_SCHEMA = "semio.hub.observability/v1" as const;
