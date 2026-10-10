import {createRequire} from "node:module";
const {buildDependencyDirectionPolicy:constructPolicy,authoredPackageExportAuthority:captureExports}=createRequire(import.meta.url)("./🟨️.cjs");
import type { DependencyPolicyPackage, DependencyPolicySnapshot, DependencyPolicyTaxonomy } from "../🚀️bootstrap/🟦️.ts";

export type DependencyPolicyConstructionInputs = Readonly<{ taxonomy: DependencyPolicyTaxonomy & Readonly<{ forbiddenPathSegments: readonly string[]; bannedNameStems?: readonly string[] }>; plugins: readonly string[]; workspacePackages: readonly DependencyPolicyPackage[]; nodeBuiltins: readonly string[] }>;

/** 🧱️ Constructs every boundary selector without filesystem access or authority caches. */
export const buildDependencyDirectionPolicy: (inputs: DependencyPolicyConstructionInputs) => DependencyPolicySnapshot["policy"] = constructPolicy;

/** 🧾️ Reads canonical authored public target metadata from a present manifest owner. */
export const authoredPackageExportAuthority:(owner:string,manifest:Readonly<Record<string,unknown>>)=>Pick<DependencyPolicyPackage,"sourceOwner"|"exports"|"exportTargets">=captureExports;
