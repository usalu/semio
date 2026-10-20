/** 🟠️ The wgpu publishes general directly tests of the ⚙️ General Settings Tree and bottom-panel flow suite. */
import { readModuleTreeSource, sliceModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { join } from "node:path";
import { expect, test } from "vitest";
import { engineRoot } from "../🟢️shared/🟦️.ts";

/** 🟡 Registers the wgpu publishes general directly tests of the ⚙️ General Settings Tree and bottom-panel flow suite. */
export function registerWgpuPublishesGeneralDirectlyTests(): void {
  test("WGPU publishes General directly as Tree rows and uses flow-owned panel geometry", () => {
    const shellRoot = join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs");
    const shellSource = readModuleTreeSource(shellRoot);
    const general = sliceModuleTreeSource(shellRoot, "fn build_settings_general_ui", "fn driver_rows");
    expect(general).toContain("UiNode::Tree(UiTreeNode");
    expect(general).not.toContain("UiNode::Stack(UiStackNode");
    expect(shellSource).toContain("flowed_anchor_content_rect(anchor.flow().block");
    expect(shellSource).toContain("flowed_anchor_tab_row_y(anchor.flow().block");
    expect(shellSource).toContain("restart_viewport_after_host_reflow");
  });
}
