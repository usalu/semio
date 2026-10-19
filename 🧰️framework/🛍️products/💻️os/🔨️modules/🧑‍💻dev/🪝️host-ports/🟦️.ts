/**
 * 🪝️ The composition root of the shell host's ports: it binds the plugin registry's catalog admission, the hub plugin source, the directory access policy, the preference vocabulary and the wasm engine packages to the contracts the React renderer declares, so the renderer imports none of them.
 * Every root that mounts the React shell (`bootFrameworkOsDev`, the `✏️s` dev entry, the demonstrator and play pages) passes the result as its host ports.
 * @see ../../📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪝️host-ports/🟦️.ts
 */
import type { CatalogAdmissionPortV1, DocumentAccessPortV1, PluginDistributionPortV1, ShellHostPortsV1, UiPreferenceVocabularyV1 } from "@semio-tech/framework-renderer-react";
import accessPolicy from "@semio-tech/framework-os/modules/directory/access-policy.json";
import { directoryAccessPermits, parseDirectoryAccessPolicyV1 } from "@semio-tech/framework-os/modules/directory/access-policy/schema";
import { UI_PREFERENCE_MUTATION_KEYS, uiPreferenceMutationDataClassV1 } from "@semio-tech/framework-os/config/schema/mutations";
import { MODULE_EXTENSION_ROUTE } from "@semio-tech/plugin-registry/deployment";
import { createHubPluginSource, HUB_SAME_ORIGIN_MOUNT } from "@semio-tech/plugin-registry/hub-source";
import pluginModuleStoreWorkerUrl from "@semio-tech/plugin-registry/hub-source/service-worker?worker&url";
import { admitPluginCatalogV1, pluginCatalogV1 } from "../../🔌️plugin/📇️registry/🟦️.ts";
import { WASM_BINDINGS_V1 } from "./🕸️wasm-bindings/🟦️.ts";

const declaredAccessPolicy = parseDirectoryAccessPolicyV1(accessPolicy);

/** 🗂️ The preference mutation vocabulary read from the mutation schema and the data classes of its keys. */
export const UI_PREFERENCE_VOCABULARY_V1: UiPreferenceVocabularyV1 = Object.freeze({
  knows: (mutationName: string) => mutationName in UI_PREFERENCE_MUTATION_KEYS,
  keyOf: (mutation) => UI_PREFERENCE_MUTATION_KEYS[mutation.mutation],
  dataClassOf: uiPreferenceMutationDataClassV1,
});

const CATALOG_ADMISSION_PORT_V1: CatalogAdmissionPortV1 = Object.freeze({ admit: admitPluginCatalogV1, catalogOf: pluginCatalogV1 });

const PLUGIN_DISTRIBUTION_PORT_V1: PluginDistributionPortV1 = Object.freeze({
  extensionRoute: MODULE_EXTENSION_ROUTE,
  hubPluginSource: ({ onNotice }) =>
    typeof navigator !== "undefined" && "serviceWorker" in navigator && typeof caches !== "undefined" && "locks" in navigator
      ? createHubPluginSource({
          hubMount: HUB_SAME_ORIGIN_MOUNT,
          storeWorkerUrl: pluginModuleStoreWorkerUrl,
          origin: globalThis.location.origin,
          onNotice,
          caches,
          locks: navigator.locks,
          serviceWorker: navigator.serviceWorker,
          persist: () => navigator.storage?.persist?.() ?? Promise.resolve(false),
        })
      : null,
});

const DOCUMENT_ACCESS_PORT_V1: DocumentAccessPortV1 = Object.freeze({ permitsDocumentWrite: (role) => directoryAccessPermits(declaredAccessPolicy, [role], "document.write") });

/** 🪝️ Every port the React shell host reads, bound to this product. */
export function createFrameworkOsHostPortsV1(): ShellHostPortsV1 {
  return Object.freeze({
    catalogAdmission: CATALOG_ADMISSION_PORT_V1,
    pluginDistribution: PLUGIN_DISTRIBUTION_PORT_V1,
    documentAccess: DOCUMENT_ACCESS_PORT_V1,
    preferenceVocabulary: UI_PREFERENCE_VOCABULARY_V1,
    wasmBindings: WASM_BINDINGS_V1,
  });
}
