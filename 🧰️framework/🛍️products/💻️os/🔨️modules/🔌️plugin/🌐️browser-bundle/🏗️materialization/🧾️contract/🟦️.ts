/** 🧾️ Contract of the plugin web materialization: the generated file names, the shard worker byte, heartbeat, diagnostics and drive declarations that the generated worker interpolates, and the materialization context. */
import type { CompilerAdmissionControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { SHARD_LIVENESS_POLICY } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🫀️liveness/🟦️.ts";

export const PLUGIN_HOST_SHIM_FILE = "🟨️.js";
export const SHARD_WORKER_FILE = "🟨️shard-worker.js";
export const GUESTSLIM_FONT_RELATIVE = "🪞️vendor/🔤️guestslim-typst-fonts.bin";

/** 🫀️ Uses the canonical lightweight shard watchdog policy for generated progress ticks. */
export const SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS = SHARD_LIVENESS_POLICY.progressIntervalMs;

/** 📤️ The generated worker's per-chunk byte cap for the segmented-download lane, interpolated into
 * {@link shardWorkerSource}. Its OWNER is the schema-owned chunk contract
 * (`https://json.schemas.assets.semio-tech.com/framework/actor/shard-client/segmented-download/schema.json` —
 * `🎭️actor/📮️shard-client/📤️segmented-download/🧬️schema/🔣️.json` + `🧫️fixtures/🔣️.json`, mirrored on the
 * host side by `SEGMENTED_DOWNLOAD_CONTRACT` and on the guest side by `🔌️plugin/🦀️.rs`'s
 * `ARTIFACT_OUTPUT_CHUNK_BYTES`); this declaration is held equal to `contract.chunkBytes` by the
 * shard-client suite, which reads this literal straight out of this file, so editing one alone fails
 * closed. Declared rather than imported for the same reason
 * {@link SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS} is. */
export const SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4096;

/** 🩺️ Mirrors `🎭️actor/🧵️shard-runtime/🟦️.ts`'s `SHARD_RUNTIME_DIAGNOSTICS_KEY`/
 * `SHARD_WORKER_DIAGNOSTICS_PARAM` and, through them, the guest's own `RUNTIME_DIAGNOSTICS_ENV`
 * (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs`). Declared rather than imported for the same reason
 * {@link SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS} is: this module is bundled into `⚙️vite.config.ts`'s
 * import graph, and reaching `🧵️shard-runtime` from here would drag `ShardClient` and the whole
 * kernel/ui/resident graph into config loading. Held equal by the `🛡️runtime-admission` suite. */
export const SHARD_RUNTIME_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";
export const SHARD_WORKER_DIAGNOSTICS_PARAM = "diagnostics";

/** 🚚️ What ONE whole reactor turn costs on the renderer this drive was measured against, in
 * milliseconds — the `worker.guest` mean of `🐍️react-hop-cost-probe.mjs` over a 10-step run of the
 * procedural 3d React door (12.9 ms over 1 154 crossings and 13.2 ms over 1 214, 2026-09-15). It is a
 * MEASUREMENT carried as a declaration, because the drive's bounds are DERIVED from it rather than
 * chosen: `SHARD_TURN_DRIVE_STEPS = floor(grant wall / this)`.
 *
 * 🐛️ It replaces a wall budget of `SHARD_TURN_SILENT_HOLD_MS = 8`, which was the reactor's own
 * executor slice (`⚛️reactor/🔄️turn/🦀️.rs`'s `run_until_deadline(64, 256 KiB, now + 8 ms)`) borrowed
 * as the worker's drive budget. A whole turn costs more than the reactor's slice of it, so an 8 ms
 * wall admitted `floor(8 / 13) = 0` further turns and the hold degenerated into "one extra poll" —
 * named but not taken by `📓️reactor-reconcile-spin-2026-09-14.md` §7 item 2. The reactor's slice is
 * still the reactor's: `REACTOR_TURN_EXECUTOR_HOLD_MS` is unchanged and unrelated to this number.
 *
 * Held equal to `⚛️reactor/🧫️fixtures/🚚️more-work-drive.json`'s `measuredGuestTurnCostMs` — and
 * through it to the reactor's own `MEASURED_GUEST_TURN_COST_MS` — by the more-work-drive suite, which
 * reads this literal straight out of this file. Declared rather than imported for the same reason
 * {@link SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS} is. */
export const SHARD_TURN_GUEST_COST_MS = 13;
/** 🚚️ Hard cap on the drive's step ceiling however large a lane's granted wall gets, so a guest whose
 * turns cost microseconds cannot park the worker's whole message queue behind one drive. It bounds
 * the DERIVATION, it is not the derivation. */
export const SHARD_TURN_DRIVE_STEP_CEILING = 512;

export type PluginWebMaterializeContext = {
  readonly repoRoot: string;
  readonly environment?: Readonly<Record<string, string | undefined>>;
  readonly preview2VendorDir: string;
  readonly signal?: AbortSignal;
  readonly control?: CompilerAdmissionControl;
  readonly optimize?: boolean;
  readonly wasmOptBin?: string;
  readonly cores?: number;
};
