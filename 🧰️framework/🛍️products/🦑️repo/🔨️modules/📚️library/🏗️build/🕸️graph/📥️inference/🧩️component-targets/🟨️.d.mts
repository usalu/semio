/** 🧩️ Declares the component-target inference component: print documents, Cargo targets, component builds and the release pipeline. */
export const component: "component-targets";
export function wire(registry: Record<string, any>): void;
export function printDocumentTargets(project: any, root: string, workspaceRoot: string, scripts: any, intern?: (inputs: any) => any): Promise<any>;
export function cargoTargets(root: string, workspaceRoot: string, commandInputs: any, scripts: any): Promise<any>;
export function componentBuildTargets(root: string, workspaceRoot: string): any;
export function componentCrateRoots(configFiles: readonly string[], workspaceRoot: string): any;
export function withReleasePipelinePreparation(project: any, workspaceRoot: string, contracts: any, componentRoots: any, cache?: Map<string, any>, closures?: Map<string, any>): any;
export function rootCommandTargets(script: any): any;
