import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";
import { registerTreeHelpersTests } from "./🔶️tree-helpers/🟦️.tsx";
import { registerVirtualFileSystemTests } from "./🔸️virtual-file-system/🟦️.tsx";
import { registerSceneHelpersTests } from "./🟠️scene-helpers/🟦️.tsx";
import { registerElementsSurfaceChromeTests } from "./🟢️elements-surface-chrome/🟦️.tsx";

export async function registerTreeHelpers4AndMoreTests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { describe, it, buildVirtualFileSystemVisibleRows, expect, VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS, uiDataLabel, buildVirtualFileSystemDescriptorColumns, renderToStaticMarkup, formatVirtualFileSystemTime, VirtualFileSystem, resolveVirtualFileSystemSchemaIcon, vi, getVirtualFileSystemNextSelectionState, SCENE_GIZMO_LABELS, resolveSceneGizmoSnapTarget, THREE, resolveSceneGizmoViewportPlacement, resetElementsSurfaceChromeForTests, applyElementsSurfaceChrome, DEFAULT_UI_DRIVER, COMPACT_UI_DRIVER, bootstrapElementsSurfaceChromeDocument, applyChromeRevealAtPoint, useCanvasAppearanceSync } = registerTreeHelpersTests({ dependencies, vitest });
  registerVirtualFileSystemTests({ describe, it, buildVirtualFileSystemVisibleRows, expect, VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS, uiDataLabel, buildVirtualFileSystemDescriptorColumns, renderToStaticMarkup, formatVirtualFileSystemTime, VirtualFileSystem, resolveVirtualFileSystemSchemaIcon, vi, getVirtualFileSystemNextSelectionState });
  registerSceneHelpersTests({ describe, it, expect, SCENE_GIZMO_LABELS, resolveSceneGizmoSnapTarget, THREE, resolveSceneGizmoViewportPlacement });
  registerElementsSurfaceChromeTests({ describe, it, resetElementsSurfaceChromeForTests, applyElementsSurfaceChrome, DEFAULT_UI_DRIVER, expect, COMPACT_UI_DRIVER, vi, bootstrapElementsSurfaceChromeDocument, applyChromeRevealAtPoint, useCanvasAppearanceSync });
}
