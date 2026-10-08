/** 📦️ Neutral canonical collection package identity. */
export interface CollectionPackageDeclaration {
 readonly definition_version: number;
 readonly id: string;
 readonly artifact: string;
 readonly directory: string;
 readonly rust_package: string;
 readonly nx_project: string;
 readonly dependencies: readonly string[];
}
export type CollectionArtifactPackage = Omit<CollectionPackageDeclaration, "definition_version">;
/** 🛬️ Admits a typed declaration against the canonical identity. */
export function admitCollectionPackageDeclaration(declaration: CollectionPackageDeclaration): CollectionArtifactPackage {
 if (declaration.definition_version !== 1 || declaration.id !== "os.collection" || declaration.artifact !== "collection" || declaration.directory !== "🗂️collection" || declaration.rust_package !== "semio-framework-artifact-space-collection" || declaration.nx_project !== "@semio-tech/framework-space-collection-rs" || declaration.dependencies.length !== 0) throw Error("Invalid collection package identity");
 const {definition_version, ...identity}=declaration; return {...identity,dependencies:[...identity.dependencies]};
}
