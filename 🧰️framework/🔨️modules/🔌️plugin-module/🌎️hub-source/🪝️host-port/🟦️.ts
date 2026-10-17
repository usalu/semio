/**
 * 🪝️ The shell-facing contract of the hub-backed plugin source: what a composition root hands a shell host that installs hub modules, with no implementation behind it. The source itself (store, service worker, transfer) is a product module; a host only reads these shapes.
 * @see ../🔍️resolution/🟦️.ts
 */
import type { PluginModuleAcquisition, PluginSource } from "@semio-tech/framework";
import type { TrustedPluginModuleIndexEntryV1, TrustedPluginModuleIndexV1 } from "../../🟦️.ts";
import type { PluginModuleSourceV1 } from "../🔍️resolution/🟦️.ts";

/** 🔔️ What the source tells the person using the shell: a stored module that must be reinstalled, or a store that is full. */
export type HubPluginSourceNoticeV1 = Readonly<{ kind: "reinstalling" | "quota-exceeded"; pluginId: string }>;

/** 🧩️ One installed hub program: the catalog generation's module of one plugin, loaded from this device's store. */
export type HubProgramModuleV1 = Readonly<{
  programId: string;
  pluginId: string;
  generationId: string;
  bundleSha256: string;
  moduleUrl: string;
  source: PluginModuleSourceV1;
}>;

/** 🌎️ The hub source plus what only it knows: whether its store controls this page, the hub's current catalog of plugin
 * modules, and the programs hub documents run on. */
export type HubPluginSourceV1 = PluginSource & Readonly<{
  /** 🚦️ Registers the store's service worker and waits until it controls the page — before any plugin worker exists. */
  ready(): Promise<void>;
  /** 🏷️ The hub package whose surfaces open this dialect artifact kind (current index, else this device's records). */
  ownerOfDialect(artifactKind: string): Promise<string | undefined>;
  /** 📚️ The hub's current generation of plugin modules, or `null` while the hub is unreachable. */
  catalog(signal: AbortSignal): Promise<TrustedPluginModuleIndexV1 | null>;
  /** 📥️ Installs `entry` of `generationId` as a hub program: from the store when it holds the bundle complete, else from
   * the staged module at `localModuleUrl` when every file of the bundle is byte-identical there, else from the hub. */
  installProgram(generationId: string, entry: TrustedPluginModuleIndexEntryV1, localModuleUrl: string | null, acquisition: PluginModuleAcquisition): Promise<HubProgramModuleV1>;
}>;
