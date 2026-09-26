export type WgpuDynamicExtensionRecord = {
  readonly extensionId: string;
  readonly directoryName: string;
  readonly version: string;
  readonly label: string;
  readonly extends: string;
  readonly moduleUrl: string;
  readonly packageHash: string;
  readonly installedAt: number;
};

export type WgpuDynamicExtensionAdmission = {
  readonly handle: unknown;
  readonly manifest: { readonly pluginId: string; readonly version: string };
};

export const WGPU_DYNAMIC_EXTENSION_INSTALL_GLOBAL = "semioWgpuInstallExtension";
export const WGPU_DYNAMIC_EXTENSION_RETIRE_GLOBAL = "semioWgpuRetireExtension";

export function parseWgpuDynamicExtensionRecord(input: unknown): WgpuDynamicExtensionRecord {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("extension-install.record: expected object");
  const row = input as Record<string, unknown>;
  const fields = ["extensionId", "directoryName", "version", "label", "extends", "moduleUrl", "packageHash"] as const;
  if (Object.keys(row).sort().join(",") !== [...fields, "installedAt"].sort().join(",")) throw new Error("extension-install.record: fields mismatch");
  for (const field of fields) if (typeof row[field] !== "string" || (field !== "label" && row[field].length === 0)) throw new Error(`extension-install.record: invalid ${field}`);
  if (!Number.isSafeInteger(row.installedAt) || Number(row.installedAt) < 0) throw new Error("extension-install.record: invalid installedAt");
  return Object.freeze(row) as WgpuDynamicExtensionRecord;
}

export async function admitWgpuDynamicExtension(recordJson: string, mount: (record: WgpuDynamicExtensionRecord) => Promise<WgpuDynamicExtensionAdmission>): Promise<unknown> {
  let decoded: unknown;
  try {
    decoded = JSON.parse(recordJson);
  } catch {
    throw new Error("extension-install.record: invalid JSON");
  }
  const record = parseWgpuDynamicExtensionRecord(decoded);
  const admitted = await mount(record);
  if (admitted.manifest.pluginId !== record.extensionId) throw new Error(`extension-install.identity: expected ${record.extensionId}, received ${admitted.manifest.pluginId}`);
  if (admitted.manifest.version !== record.version) throw new Error(`extension-install.version: expected ${record.version}, received ${admitted.manifest.version}`);
  return admitted.handle;
}

export function installWgpuDynamicExtensionDoor(
  host: { [WGPU_DYNAMIC_EXTENSION_INSTALL_GLOBAL]?: (recordJson: string) => Promise<unknown>; [WGPU_DYNAMIC_EXTENSION_RETIRE_GLOBAL]?: (extensionId: string) => Promise<void> },
  mount: (record: WgpuDynamicExtensionRecord) => Promise<WgpuDynamicExtensionAdmission>,
  retire: (extensionId: string) => Promise<void>,
): void {
  host[WGPU_DYNAMIC_EXTENSION_INSTALL_GLOBAL] = async (recordJson) => {
    try {
      return await admitWgpuDynamicExtension(recordJson, mount);
    } catch (error) {
      try {
        await retire(parseWgpuDynamicExtensionRecord(JSON.parse(recordJson) as unknown).extensionId);
      } catch {}
      throw error;
    }
  };
  host[WGPU_DYNAMIC_EXTENSION_RETIRE_GLOBAL] = async (extensionId) => {
    if (!extensionId) throw new Error("extension-retire.identity: missing extensionId");
    await retire(extensionId);
  };
}
