/** 📦️ Neutral canonical space package identity. */
export interface SpacePackageDeclaration {
 readonly definition_version: number;
 readonly id: string;
 readonly artifact: string;
 readonly directory: string;
 readonly rust_package: string;
 readonly nx_project: string;
 readonly dependencies: readonly string[];
}
export type SpaceArtifactPackage = Omit<SpacePackageDeclaration, "definition_version">;
/** 🛬️ Admits a typed declaration against the canonical identity. */
export function admitSpacePackageDeclaration(declaration: SpacePackageDeclaration): SpaceArtifactPackage {
 if (declaration.definition_version !== 1 || declaration.id !== "os.space" || declaration.artifact !== "space" || declaration.directory !== "🪐️space" || declaration.rust_package !== "semio-framework-artifact-space-space" || declaration.nx_project !== "@semio-tech/framework-space-space-rs" || declaration.dependencies.length !== 0) throw Error("Invalid space package identity");
 const {definition_version, ...identity}=declaration; return {...identity,dependencies:[...identity.dependencies]};
}
