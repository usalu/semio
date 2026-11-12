import { BundleScript } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCmd, runCargo } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runtimeExecutableV1 } from "../../../⚙️runtime/🏷️identity/🟦️.ts";
import { join } from "node:path";

/** 🧪️ Runs the verification lanes owned by the pack domain: framework-retained-pack-ownership. */
export class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "framework-retained-pack-ownership") {
      const testRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws");
      const { testRetainedPackPhysicalOwnership } = await import(`${testRoot}/🟦️.ts`);
      testRetainedPackPhysicalOwnership();
      runCmd(runtimeExecutableV1(), [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.repoRoot });
      if (segments[1] === "outer-native") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-host", "--lib", "retained_pack_outer_", "--", "--nocapture"], this.repoRoot);
        return;
      }
      if (segments[1] === "catalog-native") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_catalog_", "--", "--nocapture"], this.repoRoot);
        return;
      }
      if (segments[1] === "value") return;
      if (segments[1] === "diagnostic") return;
      if (segments[1] === "inflater") return;
      if (segments[1] === "inflater-native") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-deflate", "--lib", "retained_inflater_physical_", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_inflater_physical_", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_pipeline_diagnostic_deflate_", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "write_then_read_round_trip_with_compressed_segment_and_chunk", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
        for (const packageName of ["semio-s-artifact-procedural-2d-io-binary", "semio-s-artifact-procedural-3d-io-binary"]) {
          await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", "retained_mounted_laws", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
        }
        return;
      }
      if (segments[1] === "diagnostic-native") {
        for (const filter of [
          "retained_pack_pipeline_diagnostic_",
          "retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable",
          "retained_anchor_rejects_hostile_crc_and_requires_explicit_close",
        ]) {
          await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", filter, "--", "--nocapture"], this.repoRoot);
        }
        return;
      }
      if (segments[1] === "value-native") {
        for (const [packageName, filters] of [
          ["semio-framework-replication", ["retained_paged_list_capacity_admission_and_exact_release_grants"]],
          ["semio-framework-pack", ["retained_symbol_table_", "retained_pack_catalog_", "write_then_read_round_trip_with_compressed_segment_and_chunk"]],
          ["semio-os-kernel", ["retained_value_", "retained_record_body_"]],
          ["semio-s-artifact-procedural-2d-host", ["every_fourteen_variant_decodes_through_retained_structural_grants"]],
          ["semio-s-artifact-procedural-2d-io-binary", ["retained_mounted_laws"]],
          ["semio-s-artifact-procedural-generation3d", ["every_fourteen_variant_decodes_through_retained_structural_grants"]],
          ["semio-s-artifact-procedural-3d-io-binary", ["retained_mounted_laws"]],
        ] as const) {
          for (const filter of filters) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", filter, "--", "--nocapture"], this.repoRoot);
        }
        return;
      }
      if (segments[1] === "native") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "value::list::", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_source_laws", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "write_then_read_round_trip_with_compressed_segment_and_chunk", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-io-binary", "--lib", "retained_mounted_laws", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-host", "--lib", "retained_pack_outer_", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-3d-io-binary", "--lib", "retained_mounted_laws", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    
    throw new Error(`Unknown owned verification ${segments.join(" ")}`);
  }
}
