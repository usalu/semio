import { expect, test } from "bun:test";
import { Children, isValidElement, type ReactNode } from "react";
import { createFrameworkMarketplacePanelTab, type MarketplaceHostApi } from "../🟦️.tsx";
import fixture from "./🔣️.json" with { type: "json" };

test("Marketplace exposes exactly the extension controls the host can perform", () => {
  for (const row of fixture) {
    const toggleCalls: boolean[] = [], noop = () => {};
    const host = {
      plugins: [{pluginId:"host", label:"Host", sourceId:"fixture", status:"loaded", canUninstall:false}],
      extensions: [{extensionId:"bundle", label:"Bundle", extendsHost:"host", status:"loaded", enabled:false}],
      installPlugin: noop, uninstallPlugin: noop, reloadPlugin: noop,
      ...(row.extensionManagement ? {extensionManagement:{installFromUrl:noop, installFromFile:noop, remove:noop}} : {}),
      setExtensionEnabled: (_:string, enabled:boolean) => toggleCalls.push(enabled),
    } as MarketplaceHostApi;
    const tab = createFrameworkMarketplacePanelTab(() => host);
    if (tab.kind !== "leaf") throw Error("Marketplace requires a leaf");
    const source = tab.trees[0]!.tree;
    if (!("resolveTree" in source)) throw Error("Marketplace requires a host view");
    const tree = source.resolveTree(), controls = new Map<string, (() => void) | undefined>();
    const inspect = (node: ReactNode) => Children.forEach(node, child => {
      if (!isValidElement<{id?:string;children?:ReactNode;onClick?:()=>void}>(child)) return;
      const { id, children, onClick } = child.props;
      if (id?.startsWith("framework.marketplace.extension")) controls.set(id, onClick);
      inspect(children);
    });
    for (const section of tree.sections) for (const plugin of section.items) {
      inspect(plugin.control);
      for (const extension of plugin.items ?? []) inspect(extension.control);
    }
    expect([...controls.keys()].sort(), row.id).toEqual(row.controls);
    controls.get("framework.marketplace.extension.bundle.enable")?.();
    expect(toggleCalls).toEqual([true]);
  }
});
