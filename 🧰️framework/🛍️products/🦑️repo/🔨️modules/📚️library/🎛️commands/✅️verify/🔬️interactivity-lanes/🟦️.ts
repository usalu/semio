import { dispatchOwnedScriptRoute } from "../../../🏃️process/🧭️routing/🧩️contributions/🟦️.ts";
import { interactivityAllAppActionDispositionFailures, interactivityAllAppActionProductionFailures, interactivityAllAppDiscovery } from "../../../⚖️laws/🔬️interactivity-audit/🧭️all-app/🟦️.ts";
import { interactivityAuditRun } from "../../../⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts";
import { INTERACTIVITY_AUDIT_ASYNC_FILE, INTERACTIVITY_AUDIT_DB_ARTIFACT_FAMILY, INTERACTIVITY_AUDIT_DB_CLI_FILE, INTERACTIVITY_AUDIT_DB_COMPACT_FILE, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY, INTERACTIVITY_AUDIT_DB_INDEX_FILE, INTERACTIVITY_AUDIT_DB_NEO4J_FILE, INTERACTIVITY_AUDIT_DB_POSTGRES_FILE, INTERACTIVITY_AUDIT_DB_QUERY_FILE, INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE, INTERACTIVITY_AUDIT_DB_SQLITE_FILE, INTERACTIVITY_AUDIT_DB_STATE_FILE, INTERACTIVITY_AUDIT_DB_STORAGE_FAMILY, INTERACTIVITY_AUDIT_DB_SYNC_FILE, INTERACTIVITY_AUDIT_DB_WAL_FAMILY, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, INTERACTIVITY_AUDIT_HUB_BIN_FAMILY, INTERACTIVITY_AUDIT_PACK_FORMAT_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE, INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS, INTERACTIVITY_AUDIT_SEVERITY, INTERACTIVITY_AUDIT_SURFACE_LANE_FILE, INTERACTIVITY_AUDIT_UI_ENGINE_FILE, INTERACTIVITY_AUDIT_UI_ROOTS, INTERACTIVITY_AUDIT_WINIT_HOST_FILE, INTERACTIVITY_P1X_CONTRACT_FILE, INTERACTIVITY_P1Y_CONTRACT_FILE, INTERACTIVITY_P1Z_CONTRACT_FILE } from "../../../⚖️laws/🔬️interactivity-audit/🗂️files/🟦️.ts";
import { toolJobCoverageRun } from "../../../⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts";
import { interactivityDbIoB1B6Failures, interactivityP1qR4Failures } from "../../../⚖️laws/🔬️interactivity-audit/🗄️database/🧮️b1b6-p1q-r4/🟦️.ts";
import { policyReadRustPolicyFamily, policyReadRustPolicySource } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { interactivityDatabaseCatalogBootstrapFailures, interactivityDatabaseCompactionFailures, interactivityDatabaseCreateCatalogFailures, interactivityDatabaseSyncHelloFailures } from "../../../⚖️laws/🔬️interactivity-audit/🗄️database/🌱️database-lifecycle/🟦️.ts";
import { interactivityMountedEngineSurfaceLifetimeFailures, interactivityMountedPreparedRenderFailures, interactivityMountedSurfaceLaneFailures } from "../../../⚖️laws/🔬️interactivity-audit/🖥️mounted/🖼️mounted-surface/🟦️.ts";
import { policyReadFileSafe, policyReadModuleTreeSource } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { VerifyTaxonomyLanes } from "../🧾️taxonomy-lanes/🟦️.ts";

/** 🔬️ Interactivity audit verification lanes. */
export abstract class VerifyInteractivityLanes extends VerifyTaxonomyLanes {
  /**
   * ⏱️ `verify interactivity` — the Phase 0 forbidden-call audit (ticket
   * `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`). Scans [[INTERACTIVITY_AUDIT_UI_ROOTS]] for
   * `block_on`/`run_blocking`/sync-fs/sync-net/sync-clipboard/sync-process/sync-db, and the whole
   * repo (minus `compose/` and [[INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS]]) for
   * direct thread construction/scoped threads/Rayon/Tokio runtime construction. WARN in Phase 0 (report,
   * exit 0); flip [[INTERACTIVITY_AUDIT_SEVERITY]] to `"deny"` at Phase 3 packet P3c.
   */
  protected async runInteractivityAudit(): Promise<void> {
    await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", "puzzle-fill-policy-self-tests"]);
    const apps = interactivityAllAppDiscovery(this.root);
    const appSelfTests = await this.lanes.interactivityAllAppDiscoverySelfTests();
    console.log(`[verify interactivity apps] ${apps.descriptors.length} descriptor(s), ${apps.appCount} app declaration(s), ${apps.playgrounds} dashboard playground command(s), ${apps.surfaceCount} dev surface(s) over ${apps.renderers.join("/")}, ${apps.actionCount} action row(s), ${apps.surfaceCoveredAppCount} surface-covered app context(s), ${apps.surfaceMissingAppCount} missing surface context(s), ${apps.gates} dashboard-startable gate(s), ${appSelfTests} hostile/oracle self-test(s).`);
    for (const failure of apps.failures.slice(0, 25)) console.log(`  ${failure}`);
    if (apps.failures.length > 25) console.log(`  ${apps.failures.length - 25} additional all-app discovery failure(s) omitted.`);
    const report = interactivityAuditRun(this.root, this.lanes);
    console.log(`[verify interactivity] severity=${INTERACTIVITY_AUDIT_SEVERITY} scope: ${INTERACTIVITY_AUDIT_UI_ROOTS.length} UI root(s), ${INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS.length} runtime-sanctioned root(s) exempt from the thread-pool rule.`);
    console.log(`[verify interactivity] ${report.findings.length} finding(s) total:`);
    for (const category of Object.keys(report.byCategory).sort()) console.log(`  ${category}: ${report.byCategory[category]}`);
    if (report.blockingBridgeUnlisted.length > 0) {
      console.log(`[verify interactivity] ${report.blockingBridgeUnlisted.length} block_on/run_blocking finding(s) NOT covered by the allowlist:`);
      for (const f of report.blockingBridgeUnlisted) console.log(`  ${f.file}:${f.line}: ${f.text}`);
    }
    for (const category of Object.keys(report.byCategory).sort()) {
      if (category === "blocking-bridge") continue; // already printed above (allowlist-reconciled).
      const inCategory = report.findings.filter((f) => f.category === category);
      console.log(`[verify interactivity] ${inCategory.length} "${category}" finding(s):`);
      for (const f of inCategory) console.log(`  ${f.file}:${f.line}: ${f.text}`);
    }
    if (report.staleAllowlistEntries.length > 0) {
      console.log(`[verify interactivity] ${report.staleAllowlistEntries.length} allowlist entr(y/ies) no longer match any finding (stale — shrink the list):`);
      for (const e of report.staleAllowlistEntries) console.log(`  ${e.file} (${e.pattern}, ${e.phase})`);
    }
    if (report.expectedNeverToMatchEntries.length > 0) {
      console.log(`[verify interactivity] ${report.expectedNeverToMatchEntries.length} allowlist entr(y/ies) kept for the record but structurally invisible to this scanner (e.g. test-mod-only — never counted as stale):`);
      for (const e of report.expectedNeverToMatchEntries) console.log(`  ${e.file} (${e.pattern}, ${e.phase})`);
    }
    if (report.preDeclaredOutOfScope.length > 0) {
      console.log(`[verify interactivity] ${report.preDeclaredOutOfScope.length} allowlist entr(y/ies) pre-declared for future scope widening (not scanned today, not counted as stale):`);
      for (const e of report.preDeclaredOutOfScope) console.log(`  ${e.file} (${e.pattern}, ${e.phase})`);
    }
    const otherCategoryFindings = report.findings.filter((f) => f.category !== "blocking-bridge").length;
    if (INTERACTIVITY_AUDIT_SEVERITY === "warn") {
      console.log("[verify interactivity] WARN mode — reporting only, exit 0.");
      return;
    }
    const failures = apps.failures.length + report.blockingBridgeUnlisted.length + report.staleAllowlistEntries.length + otherCategoryFindings;
    if (failures > 0) throw new Error(`[verify interactivity] DENY mode — ${failures} failure(s) (all-app discovery/dashboard surface coverage, unlisted blocking bridges, stale allowlist entries, or sync-fs/net/clipboard/process/db/thread-pool findings).`);
    console.log("[verify interactivity] DENY mode — clean.");
  }

  /** 🧭️Prints the metadata-derived all-app ledger and enforces its bounded discovery contract. */
  protected async runInteractivityApps(args: string[]): Promise<void> {
    const report = interactivityAllAppDiscovery(this.root);
    const selfTests = await this.lanes.interactivityAllAppDiscoverySelfTests();
    if (args.includes("--json")) console.log(JSON.stringify({ ...report, selfTests }, null, 2));
    else console.log(`[verify interactivity apps] descriptors=${report.descriptors.length} extensions=${report.descriptors.filter((descriptor) => descriptor.kind === "extension").length} apps=${report.appCount} playgrounds=${report.playgrounds} renderers=${report.renderers.join("/")} surfaces=${report.surfaceCount} actions=${report.actionCount} migratedActions=${report.migratedActionCount} missingActions=${report.missingActionCount} surfaceCoveredApps=${report.surfaceCoveredAppCount} surfaceMissingApps=${report.surfaceMissingAppCount} gates=${report.gates} failures=${report.failures.length} selfTests=${selfTests}`);
    for (const failure of report.failures.slice(0, 25)) console.error(`[verify interactivity apps] ${failure}`);
    if (report.failures.length > 25) console.error(`[verify interactivity apps] ${report.failures.length - 25} additional discovery or dashboard-surface failure(s) omitted.`);
    if (report.failures.length > 0) throw new Error(`[verify interactivity apps] ${report.failures.length} discovery or dashboard-surface failure(s).`);
    if (args.includes("--actions")) {
      const toolJobs = await toolJobCoverageRun(this.root, this.lanes);
      const dispositionFailures = interactivityAllAppActionDispositionFailures(report.descriptors);
      const productionFailures = interactivityAllAppActionProductionFailures(report.descriptors, toolJobs.acceptedCommandRows, toolJobs.acceptedReservedRoutes);
      const actionFailures = [...dispositionFailures, ...productionFailures];
      console.error(`[verify interactivity apps actions] dispositionFailures=${dispositionFailures.length} productionJoinFailures=${productionFailures.length} acceptedCommands=${toolJobs.acceptedCommandRows.length} acceptedSharedRoutes=${toolJobs.acceptedReservedRoutes.length}`);
      for (const failure of dispositionFailures.slice(0, 12)) console.error(`[verify interactivity apps actions] ${failure}`);
      for (const failure of productionFailures.slice(0, 12)) console.error(`[verify interactivity apps actions] ${failure}`);
      const printedFailures = Math.min(dispositionFailures.length, 12) + Math.min(productionFailures.length, 12);
      if (actionFailures.length > printedFailures) console.error(`[verify interactivity apps actions] ${actionFailures.length - printedFailures} additional action failure(s) omitted from console output.`);
      if (actionFailures.length > 0) throw new Error(`[verify interactivity apps actions] ${actionFailures.length} descriptor action disposition or production-reachability join failure(s).`);
      console.log(`[verify interactivity apps actions] ${report.actionCount} descriptor action row(s) joined to ${toolJobs.acceptedCommandRows.length} accepted production command row(s) and ${toolJobs.acceptedReservedRoutes.length} shared reserved route(s).`);
    }
  }

  /** 🗄️ Runs the isolated P1q B1–B6 retained DB I/O source and hostile-mutation gate. */
  protected runInteractivityP1qB1B6(): void {
    this.lanes.interactivityDbIoB1B6SelfTests();
    this.lanes.interactivityP1qR4SelfTests();
    const failures = interactivityDbIoB1B6Failures(
      policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_STORAGE_FAMILY),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_SQLITE_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_POSTGRES_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_NEO4J_FILE),
      policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_HUB_BIN_FAMILY),
    );
    failures.push(
      ...interactivityP1qR4Failures(
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_PACK_FORMAT_FILE),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_INDEX_FILE),
        policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_WAL_FAMILY),
        policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ARTIFACT_FAMILY),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_QUERY_FILE),
        policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_COMPACT_FILE),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_CLI_FILE),
        policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_STATE_FILE),
      ),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p1q-b1-b6] ${failures.join("; ")}`);
    console.log("[verify interactivity p1q-b1-b6] live-source and hostile mutations clean.");
  }

  /** 🌱️ Runs the isolated P1w retained initial-catalog CAS source and hostile-mutation gate. */
  protected runInteractivityP1w(): void {
    this.lanes.interactivityDatabaseCatalogBootstrapSelfTests();
    const failures = interactivityDatabaseCatalogBootstrapFailures(policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY));
    if (failures.length > 0) throw new Error(`[verify interactivity p1w] ${failures.join("; ")}`);
    console.log("[verify interactivity p1w] live-source and hostile mutations clean.");
  }

  /** 🌱️ Runs the isolated P1x retained create-document catalog CAS source and hostile-mutation gate. */
  protected runInteractivityP1x(): void {
    this.lanes.interactivityDatabaseCreateCatalogSelfTests();
    const failures = interactivityDatabaseCreateCatalogFailures(
      policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_ASYNC_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_P1X_CONTRACT_FILE),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p1x] ${failures.join("; ")}`);
    console.log("[verify interactivity p1x] live-source and hostile mutations clean.");
  }

  /** 🧹 Runs the isolated P1y retained database-compaction source and hostile-mutation gate. */
  protected runInteractivityP1y(): void {
    const compact = policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_COMPACT_FILE);
    const engine = policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY);
    const snapshot = policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE);
    const index = policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_INDEX_FILE);
    const contract = policyReadRustPolicySource(this.root, INTERACTIVITY_P1Y_CONTRACT_FILE);
    this.lanes.interactivityDatabaseCompactionSelfTests(compact, engine, snapshot, index, contract);
    const failures = interactivityDatabaseCompactionFailures(compact, engine, snapshot, index, contract);
    if (failures.length > 0) throw new Error(`[verify interactivity p1y] ${failures.join("; ")}`);
    console.log("[verify interactivity p1y] live-source and hostile mutations clean.");
  }

  /** 👋️ Runs the isolated P1z retained sync-hello source and hostile-mutation gate. */
  protected runInteractivityP1z(): void {
    const sync = policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_DB_SYNC_FILE);
    const engine = policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY);
    const hub = policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_HUB_BIN_FAMILY);
    const wal = policyReadRustPolicyFamily(this.root, INTERACTIVITY_AUDIT_DB_WAL_FAMILY);
    const protocol = policyReadRustPolicySource(this.root, "🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs");
    const contract = policyReadRustPolicySource(this.root, INTERACTIVITY_P1Z_CONTRACT_FILE);
    this.lanes.interactivityDatabaseSyncHelloSelfTests(sync, engine, hub, wal, protocol, contract);
    const failures = interactivityDatabaseSyncHelloFailures(sync, engine, hub, wal, protocol, contract);
    if (failures.length > 0) throw new Error(`[verify interactivity p1z] ${failures.join("; ")}`);
    console.log("[verify interactivity p1z] live-source and hostile mutations clean.");
  }

  /** 🎨️ Runs the isolated P5d mounted prepared-render source and hostile-mutation gate. */
  protected runInteractivityP5d(): void {
    this.lanes.interactivityMountedPreparedRenderSelfTests(this.root);
    const failures = interactivityMountedPreparedRenderFailures(
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE),
      policyReadRustPolicySource(this.root, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE),
      policyReadRustPolicySource(this.root, "🧰️framework/🔨️modules/🖱️ui/🖌️render/🖼️frame/🦀️.rs"),
      policyReadRustPolicySource(this.root, "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎬️scene/🦀️.rs"),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p5d] ${failures.join("; ")}`);
    console.log("[verify interactivity p5d] live-source and hostile mutations clean.");
  }

  /** 📐️ Runs the isolated P5e mounted multi-window resize/surface lane source and mutation gate. */
  protected runInteractivityP5e(): void {
    this.lanes.interactivityLiveReconcileSelfTests(this.root);
    this.lanes.interactivityMountedLayoutTextSelfTests(this.root);
    this.lanes.interactivityMountedFrameTransactionSelfTests(this.root);
    this.lanes.interactivityMountedPreparedRenderSelfTests(this.root);
    this.lanes.interactivityMountedSurfaceLaneSelfTests(this.root);
    const failures = interactivityMountedSurfaceLaneFailures(
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_SURFACE_LANE_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_UI_ENGINE_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_WINIT_HOST_FILE),
      policyReadModuleTreeSource(this.root, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p5e] ${failures.join("; ")}`);
    console.log("[verify interactivity p5e] live-source and hostile mutations clean.");
  }

  /** 🖥️ Runs the isolated P3mn paired engine-surface lifetime source and hostile-mutation gate. */
  protected runInteractivityP3mn(): void {
    this.lanes.interactivityMountedEngineSurfaceLifetimeSelfTests(this.root);
    const failures = interactivityMountedEngineSurfaceLifetimeFailures(
      policyReadModuleTreeSource(this.root, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE),
      policyReadFileSafe(this.root, INTERACTIVITY_AUDIT_WINIT_HOST_FILE),
      policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"),
      policyReadModuleTreeSource(this.root, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE),
      policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs"),
      ["🧱️core", "🎞️session"].map((node) => policyReadModuleTreeSource(this.root, `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/${node}/🦀️.rs`)).join("\n"),
      policyReadModuleTreeSource(this.root, "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs"),
      policyReadModuleTreeSource(this.root, "🧰️framework/🔨️modules/✍️editor/🦀️.rs"),
      ["🗃️model", "🕸️graph", "🧭️host"].map((node) => policyReadModuleTreeSource(this.root, `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/${node}/🦀️.rs`)).join("\n"),
    );
    if (failures.length > 0) throw new Error(`[verify interactivity p3mn] ${failures.join("; ")}`);
    console.log("[verify interactivity p3mn] live-source and hostile mutations clean.");
  }
}
