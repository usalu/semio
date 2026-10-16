/** 🏷️ Component receipt value types: the neutral shape of the generated binding of one component's semantic descriptor to the exact bytes one build profile produced.
 * Only the types live here, so document open plans and execution-target leases can name a receipt without reaching the schema authority; the schema (`🔣️.json`), its validators and the canonical encoding stay with the plugin describe module. */

/** 🎚️ A component build profile: `release` ships, `prototype` is the local single-plugin loop. */
export type ComponentProfileV1 = "release" | "prototype";

/** 🧾️ One component's generated binding of its semantic descriptor to the bytes one profile produced. */
export type ComponentReceiptV1 = Readonly<{
  schema: string;
  pluginId: string;
  packageId: string;
  profile: ComponentProfileV1;
  target: string;
  wasmSha256: string;
  wasmBytes: number;
  semanticDescriptorSha256: string;
  toolchain: string;
  sourceTreeDigest: string;
  provenanceSha256: string;
}>;
