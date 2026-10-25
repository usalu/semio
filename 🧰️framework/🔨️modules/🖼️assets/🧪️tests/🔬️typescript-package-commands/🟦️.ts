import type { ScriptInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createChromiumSvgVideoRuntimeV1 } from "../../../../📦️packages/🟦️typescript/🖌️raster/🎥️video/🖋️svg-export/🌐️browser/🟦️.ts";
import { renderCatalogArtifacts } from "../../🔣️icons/🏗️builder/📽️projection/🟦️.ts";
import { previewAssetArtifacts, writeAssetArtifacts } from "../../🏗️builder/📦️publication/🟦️.ts";
import { exportLogoAnimation } from "../../🪧️logos/🏗️builder/🎞️animation/🟦️.ts";

/** 🏃️ Runs the `GenerateCatalogScript` command under the caller's roots and invocation. */
export function runGenerateCatalogScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): void {
  const target = (segments[0] ?? "all").toLowerCase();
  if (!["js", "net", "py", "rust", "all"].includes(target)) throw new Error(`Unknown catalog generate target: ${target}`);
  const artifacts = renderCatalogArtifacts(target);
  writeAssetArtifacts(artifacts);
  console.log(`[asset] wrote ${artifacts.length} catalog artifacts → ${target}`);
}

/** ▶️ Runs the `ExportLogoScript` command under the caller's roots and invocation. */
export async function runExportLogoScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): Promise<void> {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("Logo video export requires caller-owned SEMIO_TEST_ARTIFACT_DIR");
  const runtime = await createChromiumSvgVideoRuntimeV1();
  try {
    await exportLogoAnimation({ artifactRoot, openBrowser: runtime.openBrowser, encoderExecutable: "ffmpeg" });
  } finally {
    await runtime.close();
  }
}

/** 📜️ Runs the `PreviewGeneratedScript` command under the caller's roots and invocation. */
export function runPreviewGeneratedScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): void {
  process.stdout.write(previewAssetArtifacts(context.repoRoot));
}
