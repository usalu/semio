/** 🏪️ Marketplace chrome panel: the plugin and extension entries, the marketplace host api, the status labels, the marketplace tree and the Marketplace panel tab. */
import { Button, type PanelTabNode, type TreeDataItem, type TreeDataSection, type TreePanelConfig, type UiLabel, singleTreeLeaf } from "@semio-tech/ui-react";
import { type PluginPanelStatus } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { shellLabel, shellTabIcon } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";

//#region 🔖️os-chrome-panels
//#region MarketplacePanel
/** 🔌️ One registry entry as the marketplace wants to render it — `sourceId` and `canUninstall` come
 * straight from the shell's `PluginSource`/primary-plugin bookkeeping; `label`/`version` fall back to
 * the bare pluginId when a plugin hasn't loaded far enough to have a manifest yet (`"available"`). */
export type MarketplacePluginEntry = {
  readonly pluginId: string;
  readonly label: string;
  readonly version?: string;
  readonly status: PluginPanelStatus;
  readonly sourceId: string;
  readonly canUninstall: boolean;
};

/** 🧩️ One extension nested beneath the plugin identified by `extendsHost`. */
export type MarketplaceExtensionEntry = {
  readonly extensionId: string;
  readonly label: string;
  readonly version?: string;
  readonly extendsHost: string;
  readonly enabled: boolean;
  readonly status: PluginPanelStatus;
};

export type MarketplaceHostApi = {
  readonly plugins: readonly MarketplacePluginEntry[];
  readonly extensions: readonly MarketplaceExtensionEntry[];
  readonly installPlugin: (pluginId: string) => void;
  readonly uninstallPlugin: (pluginId: string) => void;
  readonly reloadPlugin: (pluginId: string) => void;
  readonly installExtensionFromUrl: (sourceUri: string) => void;
  readonly installExtensionFromFile: (file: File) => void;
  readonly uninstallExtension: (extensionId: string) => void;
  readonly setExtensionEnabled: (extensionId: string, enabled: boolean) => void;
};

export const FRAMEWORK_MARKETPLACE_TAB_ID = "framework.marketplace";

function pluginStatusLabel(status: PluginPanelStatus): UiLabel {
  if (status === "available") return shellLabel("ui.plugins.status.available");
  if (status === "installing") return shellLabel("ui.plugins.status.installing");
  if (status === "loaded") return shellLabel("ui.plugins.status.loaded");
  if (status === "failed") return shellLabel("ui.plugins.status.failed");
  return shellLabel("ui.plugins.status.reloading");
}

function marketplaceExtensionItem(entry: MarketplaceExtensionEntry, host: MarketplaceHostApi): TreeDataItem {
  return {
    id: `framework.marketplace.plugin.${entry.extendsHost}.extension.${entry.extensionId}`,
    label: `${entry.label}${entry.version ? ` · ${entry.version}` : ""} · ${entry.enabled ? shellLabel("ui.plugins.extension.enabled") : shellLabel("ui.plugins.extension.disabled")}`,
    loading: entry.status === "installing" || entry.status === "reloading",
    control: (
      <div className="flex items-center gap-1">
        <Button icon={entry.enabled ? "eye-off" : "eye"}
          id={`framework.marketplace.extension.${entry.extensionId}.enable`}
          text={entry.enabled ? shellLabel("ui.plugins.extension.disable") : shellLabel("ui.plugins.extension.enable")}
          disabled={entry.status !== "loaded" && entry.status !== "available"}
          onClick={() => host.setExtensionEnabled(entry.extensionId, !entry.enabled)}
        />
        <Button icon="trash-2"
          id={`framework.marketplace.extension.${entry.extensionId}.uninstall`}
          text={shellLabel("ui.plugins.action.uninstall")}
          disabled={entry.status === "installing" || entry.status === "reloading"}
          onClick={() => host.uninstallExtension(entry.extensionId)}
        />
      </div>
    ),
  };
}

function marketplacePluginItem(entry: MarketplacePluginEntry, extensions: readonly MarketplaceExtensionEntry[], host: MarketplaceHostApi): TreeDataItem {
  return {
    id: `framework.marketplace.plugin.${entry.pluginId}`,
    label: `${entry.label}${entry.version ? ` · ${entry.version}` : ""} · ${pluginStatusLabel(entry.status)}`,
    loading: entry.status === "installing" || entry.status === "reloading",
    defaultOpen: extensions.length > 0,
    ...(extensions.length > 0 ? { items: extensions.map((extension) => marketplaceExtensionItem(extension, host)) } : {}),
    control:
      entry.status === "available" || entry.status === "failed" ? (
        <Button icon="download" id={`framework.marketplace.plugin.${entry.pluginId}.install`} text={shellLabel("ui.plugins.action.install")} onClick={() => host.installPlugin(entry.pluginId)} />
      ) : (
        <div className="flex items-center gap-1">
          <Button icon="rotate-ccw"
            id={`framework.marketplace.plugin.${entry.pluginId}.reload`}
            text={shellLabel("ui.plugins.action.reload")}
            disabled={entry.status !== "loaded"}
            onClick={() => host.reloadPlugin(entry.pluginId)}
          />
          <Button icon="trash-2"
            id={`framework.marketplace.plugin.${entry.pluginId}.uninstall`}
            text={shellLabel("ui.plugins.action.uninstall")}
            disabled={!entry.canUninstall || entry.status !== "loaded"}
            onClick={() => host.uninstallPlugin(entry.pluginId)}
          />
        </div>
      ),
  };
}

function buildMarketplaceTree(host: MarketplaceHostApi): TreePanelConfig {
  const extensionsByHost = new Map<string, MarketplaceExtensionEntry[]>();
  for (const entry of host.extensions) {
    const list = extensionsByHost.get(entry.extendsHost) ?? [];
    list.push(entry);
    extensionsByHost.set(entry.extendsHost, list);
  }
  for (const entries of extensionsByHost.values()) entries.sort((left, right) => left.extensionId.localeCompare(right.extensionId));

  const bySource = new Map<string, MarketplacePluginEntry[]>();
  for (const entry of host.plugins) {
    const list = bySource.get(entry.sourceId) ?? [];
    list.push(entry);
    bySource.set(entry.sourceId, list);
  }
  const listedPluginIds = new Set(host.plugins.map((entry) => entry.pluginId));
  const sections: TreeDataSection[] = [
    {
      id: "framework.marketplace.extensions.install",
      label: shellLabel("ui.plugins.extension.install"),
      defaultOpen: true,
      items: [
        {
          id: "framework.marketplace.extensions.install.url",
          label: shellLabel("ui.plugins.extension.fromUrl"),
          control: (
            <Button icon="download"
              id="framework.marketplace.extensions.install.url"
              text={shellLabel("ui.plugins.extension.installFromUrl")}
              onClick={() => {
                const sourceUri = typeof window !== "undefined" ? window.prompt(String(shellLabel("ui.plugins.extension.urlPrompt"))) : null;
                if (sourceUri?.trim()) host.installExtensionFromUrl(sourceUri.trim());
              }}
            />
          ),
        },
        {
          id: "framework.marketplace.extensions.install.file",
          label: shellLabel("ui.plugins.extension.fromFile"),
          control: (
            <label className="inline-flex cursor-pointer">
              <input
                id="framework.marketplace.extensions.install.file"
                type="file"
                accept=".sxt,.semio,application/octet-stream"
                className="sr-only"
                onChange={(event) => {
                  const file = event.target.files?.[0];
                  if (file) host.installExtensionFromFile(file);
                  event.target.value = "";
                }}
              />
              <Button icon="download" id="framework.marketplace.extensions.install.file.trigger" text={shellLabel("ui.plugins.extension.installFromFile")} />
            </label>
          ),
        },
      ],
    },
    ...[...bySource.entries()]
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([sourceId, entries]) => ({
        id: `framework.marketplace.source.${sourceId}`,
        label: `${shellLabel("ui.plugins.source")}: ${sourceId}`,
        defaultOpen: true,
        items: [...entries]
          .sort((left, right) => left.pluginId.localeCompare(right.pluginId))
          .map((entry) => marketplacePluginItem(entry, extensionsByHost.get(entry.pluginId) ?? [], host)),
      })),
    ...[...extensionsByHost.entries()]
      .filter(([hostId]) => !listedPluginIds.has(hostId))
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([hostId, extensions]) => ({
        id: `framework.marketplace.missing-host.${hostId}`,
        label: hostId,
        defaultOpen: true,
        items: extensions.map((extension) => marketplaceExtensionItem(extension, host)),
      })),
  ];
  return { sections };
}

export function createFrameworkMarketplacePanelTab(getHost: () => MarketplaceHostApi | null): PanelTabNode {
  return singleTreeLeaf({
    id: FRAMEWORK_MARKETPLACE_TAB_ID,
    icon: shellTabIcon("store"),
    name: shellLabel("ui.plugins.marketplace"),
    order: 1,
    tree: {
      resolveTree: () => {
        const host = getHost();
        return host ? buildMarketplaceTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.plugins.marketplaceUnavailable") }] }] };
      },
    },
  });
}
//#endregion MarketplacePanel
//#endregion 🔖️os-chrome-panels
