/** 🚀️ Primary session establishment, plugin install, reload and uninstall, hub program installs and the extension ledger with its install, uninstall and enable operations.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `usePluginLifecycle` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

// #region 🔌️Adapters
import type { ShellCoreAfterDirectoryAndRoute } from "../🛂️directory-route/🟦️.ts";
import { usePluginInstallCallbacks, usePluginLifecycleState } from "./🧱️lifecycle-state-and-install/🟦️.ts";
import { useExtensionCallbacks, usePluginReloadCallbacks } from "./🔄️reload-and-extension-callbacks/🟦️.ts";
// #endregion 🔌️Adapters

export type ShellCoreAfterPluginLifecycle = ShellCoreAfterDirectoryAndRoute & ReturnType<typeof usePluginLifecycle>;
export function usePluginLifecycle(core: ShellCoreAfterDirectoryAndRoute) {
  const { pluginOpInFlightRef, loadedPluginsRef, hubPluginSource, registry, pluginSource, dispatch, pluginInstallAbortsRef, notePluginInstallProgress, pluginModuleUrlByIdRef, recordPluginArtifactRebuiltAt, primaryPluginId, sessionRef, establishPrimaryWithShardRetry, appId, pluginStatusById, pluginInstallProgressById, extensionRegistry, directoryHomeOwnerRef, backboneWorkerRef, retireDirectoryHomeOwner, contributorInstancesRef, hostMode, establishPrimarySession, pluginArtifactRebuiltAtRef, catalog, uiLocaleRef, uiTerminologyRef, extraWindowInstancesRef, extensionStoreUrl, extensionFetchAbortRef, extensionRoute, shellPluginCanvasStatus } = usePluginLifecycleState({ core });
  const { installPlugin, installingPluginIds, pluginInstallProgress, cancelPluginInstalls, commandStalls, commandStallClockMs, installHubDocumentProgram } = usePluginInstallCallbacks({ pluginOpInFlightRef, loadedPluginsRef, hubPluginSource, registry, pluginSource, dispatch, pluginInstallAbortsRef, notePluginInstallProgress, pluginModuleUrlByIdRef, recordPluginArtifactRebuiltAt, primaryPluginId, sessionRef, establishPrimaryWithShardRetry, appId, pluginStatusById, pluginInstallProgressById, extensionRegistry });
  const { extensionTargetById, setExtensionLedger, dispatchSpaceExtensionOp, reloadPlugin, uninstallPlugin, extensionLedger, extensionLedgerRef, extensionIdSet } = usePluginReloadCallbacks({ pluginOpInFlightRef, loadedPluginsRef, installPlugin, dispatch, pluginSource, sessionRef, directoryHomeOwnerRef, backboneWorkerRef, retireDirectoryHomeOwner, core, contributorInstancesRef, hostMode, pluginModuleUrlByIdRef, recordPluginArtifactRebuiltAt, establishPrimarySession, primaryPluginId, pluginArtifactRebuiltAtRef, catalog, uiLocaleRef, uiTerminologyRef, extraWindowInstancesRef });
  const { installExtension, installExtensionFromFile, uninstallExtension, setExtensionEnabled } = useExtensionCallbacks({ extensionStoreUrl, extensionFetchAbortRef, pluginSource, pluginOpInFlightRef, dispatch, pluginModuleUrlByIdRef, extensionTargetById, setExtensionLedger, dispatchSpaceExtensionOp, extensionRoute, loadedPluginsRef, contributorInstancesRef, pluginArtifactRebuiltAtRef });
  return {
    primaryPluginId, shellPluginCanvasStatus, hubPluginSource, extensionRegistry, pluginSource,
    establishPrimaryWithShardRetry, installPlugin, installingPluginIds, pluginInstallProgress, cancelPluginInstalls,
    commandStalls, commandStallClockMs, installHubDocumentProgram, reloadPlugin, uninstallPlugin, extensionLedger,
    extensionLedgerRef, extensionIdSet, installExtension, installExtensionFromFile, uninstallExtension,
    setExtensionEnabled
  };
}
