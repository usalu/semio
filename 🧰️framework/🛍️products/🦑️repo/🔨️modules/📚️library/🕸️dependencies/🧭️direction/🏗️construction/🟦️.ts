import { buildDependencyDirectionPolicy as constructPolicy } from "./🟨️.cjs";
import type { DependencyPolicyPackage, DependencyPolicySnapshot, DependencyPolicyTaxonomy } from "../🚀️bootstrap/🟦️.ts";

export type DependencyPolicyConstructionInputs = Readonly<{ taxonomy: DependencyPolicyTaxonomy & Readonly<{ forbiddenPathSegments: readonly string[]; bannedNameStems?: readonly string[] }>; plugins: readonly string[]; workspacePackages: readonly DependencyPolicyPackage[]; nodeBuiltins: readonly string[] }>;

/** 🧱️ Constructs every boundary selector without filesystem access or authority caches. */
export const buildDependencyDirectionPolicy: (inputs: DependencyPolicyConstructionInputs) => DependencyPolicySnapshot["policy"] = constructPolicy;
