/** 🏭️ Declares the generator-output inference component: contract inputs, output owners and coupling inputs of generated files. */
export const component: "generator-outputs";
export function wire(registry: Record<string, any>): void;
export function generatorContractInputs(contract: Record<string, any>): any;
export function generatorOutputOwners(contracts: Record<string, any>, root: string, project: string, targets: Record<string, any>): Map<string, string[]>;
export function outputRootInputs(output: Record<string, any>, workspaceRoot: string): any;
export function resolveOutputPath(output: string, root: string, workspaceRoot: string): string | undefined;
export function generatorOutputCouplingInputs(name: string, target: Record<string, any>, targets: Record<string, any>, root: string, workspaceRoot: string): any;
