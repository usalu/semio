import type{BuildInferenceContextV1,BuildDependencyContextV1,BuildInferencePluginV1,GeneratedNodesV1,BuildInferenceEdgeV1}from"../📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs";
/** 🧪️ Dynamic inference DATA stays unknown until its firstparty consumer validates it. */
export function testBuildInferenceDirectoryV1(workspace:string):string;
export function createDependencies(options:Readonly<Record<string,unknown>>,context:BuildDependencyContextV1):Promise<readonly BuildInferenceEdgeV1[]>;
declare const plugin:Pick<BuildInferencePluginV1,"createNodesV2">;
export default plugin;
/** 🔣️ Names the frozen test vocabulary the taxonomy authority publishes; only its owner reads its fields. */
export interface TestVocabularyV1 extends Readonly<Record<string,unknown>>{}
export const internals:Readonly<{
 isExcluded(vocabulary:TestVocabularyV1,relPath:string):boolean;
 projectNameFor(ownerRel:string,hash:string):string;
 inputsFor(workspaceRoot:string,vocabulary:TestVocabularyV1,ownerRel:string,caseRel:string,adapters:readonly string[]):string[];
 taxonomy(workspaceRoot:string):TestVocabularyV1;
 rustSutCratePath(workspaceRoot:string,ownerRel:string):string|null;
 oracleContributionPaths(workspaceRoot:string,vocabulary:TestVocabularyV1,ownerRel:string):string[];
 target:unknown;
}>;
export function discoverCaseDirs(workspaceRoot:string):unknown;
export function discoverTestBuildProjectsV1(files:readonly string[],options:Readonly<Record<string,unknown>>,context:BuildInferenceContextV1):Promise<GeneratedNodesV1>;
export function discoverTestBuildDependenciesV1(options:unknown,context:unknown):Promise<unknown>;
