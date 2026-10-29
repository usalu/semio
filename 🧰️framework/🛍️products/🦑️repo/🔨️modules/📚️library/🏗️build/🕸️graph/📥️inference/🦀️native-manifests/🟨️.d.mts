/** 🦀️ Declares the native-manifest inference component: Cargo and Go manifests, source files and native dependency closures. */
export const component: "native-manifests";
export function wire(registry: Record<string, any>): void;
export function rustSourceFiles(entries: readonly any[], manifestRoot: string, cache?: Map<string, any>, snapshots?: Map<string, any>, compiled?: boolean, allowedFiles?:ReadonlySet<string>): any;
export function assertedCargoBuildScriptInputsV1(cargo:unknown,script:string,workspaceRoot:string,operations:Readonly<{stat(path:string):Readonly<{isFile():boolean;isSymbolicLink():boolean;size:number}>;read(path:string):Uint8Array;sources(script:string,allowed:ReadonlySet<string>):string[]}>):string[]|undefined;
export function cargoSourceInputs(root: string, workspaceRoot: string, facts: any, includeTests?: boolean, compiled?: boolean): any;
export function readToml(path: string): any;
export function nativeDependencies(manifest: any, workspace: any): any;
export function nativeWorkspaceV1(directory: string, workspaceRoot: string, manifest?: any): any;
export function nativeDependencyRoots(root: string, workspaceRoot: string, tests?: boolean, cache?: Map<string, any>, closures?: Map<string, any>): any;
export function nativePreparation(root: string, workspaceRoot: string, contracts: any, tests?: boolean, cache?: Map<string, any>, closures?: Map<string, any>): any;
export function withNativePreparation(project: any, workspaceRoot: string, contracts: any, cache?: Map<string, any>, projectsByRoot?: Map<string, any>, closures?: Map<string, any>): any;
export function goDependencies(text: string): any;
export function goManifest(root: string, workspaceRoot: string): any;
