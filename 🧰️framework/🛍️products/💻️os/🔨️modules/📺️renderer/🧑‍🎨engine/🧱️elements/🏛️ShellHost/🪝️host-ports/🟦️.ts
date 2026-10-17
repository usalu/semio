/**
 * 🪝️ The ports a composition root injects into the shell host: the behaviour the host needs from products it must not import. Each port is optional and independent; an absent port means the capability is absent, never a built-in default.
 * @see ../../🐚️Shell/🟦️.tsx
 * @see ../../🪪️WasmSessionLoader/🟦️.tsx
 */
import type { UiPreferenceVocabularyV1 } from "../../🎚️UiPreferences/🟦️.ts";
import type { WasmBindingsV1 } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import type { HubPluginSourceNoticeV1, HubPluginSourceV1 } from "@semio-tech/framework/plugin-module/hub-source/port";
import type { DirectorySpaceRole } from "@semio-tech/framework/directory/schema";
import type { PluginCatalog } from "@semio-tech/framework";
import type { PluginCatalogAdmissionV1, PluginCatalogRowsV1 } from "@semio-tech/framework/plugin-catalog";

/** 🌎️ How plugin modules reach this page: the route the extension store answers on, and the hub-backed source a device with the needed browser capabilities installs from. */
export type PluginDistributionPortV1 = Readonly<{
  extensionRoute: string;
  hubPluginSource(options: Readonly<{ onNotice: (notice: HubPluginSourceNoticeV1) => void }>): HubPluginSourceV1 | null;
}>;

/** 🛡️ What the declared access policy lets a member do with a shared space's documents. */
export type DocumentAccessPortV1 = Readonly<{
  permitsDocumentWrite(role: DirectorySpaceRole): boolean;
}>;

/** 🛂️ Admits an inventory under an execution authority and builds the runtime catalog from admitted rows; the admission reads the plugin registry's JSON schema, which a host must not import. */
export type CatalogAdmissionPortV1 = Readonly<{
  admit(input: unknown, control: PluginCatalogAdmissionV1): PluginCatalogRowsV1;
  catalogOf(rows: PluginCatalogRowsV1): PluginCatalog;
}>;

/** 🪝️ Every port a shell host reads. */
export type ShellHostPortsV1 = Readonly<{
  catalogAdmission?: CatalogAdmissionPortV1;
  pluginDistribution?: PluginDistributionPortV1;
  documentAccess?: DocumentAccessPortV1;
  preferenceVocabulary?: UiPreferenceVocabularyV1;
  wasmBindings?: WasmBindingsV1;
}>;
