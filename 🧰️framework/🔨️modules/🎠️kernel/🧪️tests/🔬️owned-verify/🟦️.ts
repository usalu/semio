import { BundleScript } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCmd, runCargo } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runtimeExecutableV1 } from "../../../⚙️runtime/🏷️identity/🟦️.ts";
import { join } from "node:path";

/** 🧪️ Runs the verification lanes owned by the kernel domain: media-export-encoding, file-open-import, ui-dirty-scope, spawned-job-drive. */
export class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "media-export-encoding") {
      const { testMediaExportEncodingContract } = await import("../../📦️packages/🟦️typescript/🧪️testing/⬇️media-export-encoding/🟦️.ts");
      testMediaExportEncodingContract();
      runCmd(runtimeExecutableV1(), [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.repoRoot, "🧰️framework/🔨️modules/🚪️io/🔤️base64")}/🟦️.ts`], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-io-base64", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "media_export_encoding", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    if (segments[0] === "file-open-import") {
      const { testFileOpenImportContract } = await import("../../📦️packages/🟦️typescript/🧪️testing/📤️file-open-import/🟦️.ts");
      testFileOpenImportContract();
      runCmd(runtimeExecutableV1(), [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚪️host-io")}/🟦️.ts`], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "file_open_import", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-renderer-wgpu", "--lib", "file_open_import", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    if (segments[0] === "ui-dirty-scope") {
      const { testUiDirtyScopeContract } = await import("../../📦️packages/🟦️typescript/🧪️testing/🐢️ui-dirty-scope/🟦️.ts");
      testUiDirtyScopeContract();
      if (segments[1] === "native") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "ui_dirty_scope", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-renderer-wgpu", "--lib", "ui_dirty_scope", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    if (segments[0] === "spawned-job-drive") {
      const { testSpawnedJobDriveContract } = await import("../../📦️packages/🟦️typescript/🧪️testing/🧵️spawned-job-drive/🟦️.ts");
      await testSpawnedJobDriveContract();
      if (segments[1] === "native") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "spawned_job_drive", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    
    
    
    
    
    
    
    throw new Error(`Unknown owned verification ${segments.join(" ")}`);
  }
}
