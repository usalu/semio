/**
 * 🗂️ The serializable plugin inventory a host mounts: the admitted rows and the execution authority that admits them. The admission itself reads a product's JSON schema and stays with the plugin registry; a host receives it as a port.
 * @see ../../🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts
 */
import type { PluginHostConfig, PluginCatalogTarget, PlaygroundCatalogTarget } from "@semio-tech/framework";

/** 🎯️ One plugin or extension row with its module address and its activation edges. */
export type PluginCatalogRowV1 = PluginCatalogTarget & { readonly moduleUrl: string; readonly dependsOn: readonly string[]; readonly activationEvents: readonly string[]; readonly capabilities: readonly string[] };

/** 📇️ The admitted inventory of one mount. */
export type PluginCatalogRowsV1 = { readonly version: 1; readonly targets: readonly PluginCatalogRowV1[]; readonly hosts: readonly PluginHostConfig[]; readonly playgrounds: readonly PlaygroundCatalogTarget[] };

/** 🛂️ The finite execution authority one admission runs under: byte, row, edge and work credits, a deadline, cancellation and progress. */
export type PluginCatalogAdmissionV1 = { readonly maxBytes: number; readonly maxRows: number; readonly maxEdges: number; readonly maxWork: number; readonly deadlineMs: number; readonly now: () => number; readonly cancelled: () => boolean; readonly progress: (event: { readonly completed: number; readonly total: number; readonly work: number }) => void };
