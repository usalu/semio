import type {StateConfigV1,TaskConfigV1} from "../../🗃️state/🟨️.mjs";
export interface DerivedStateConfigV1 extends StateConfigV1{readonly owners:readonly {name:string;root:string;deps:readonly string[];tasks:readonly (TaskConfigV1&{dependsOn:readonly string[];outputs:readonly string[];cacheable:boolean})[]}[];readonly env:{paths:readonly string[];prefixes:readonly string[]}}
export function stateConfigurationFromBuildGraphV1(graph:any,options?:{environmentPaths?:readonly string[];environmentPrefixes?:readonly string[]}):DerivedStateConfigV1;
export function createBuildGraphPortV1(options:{loadGraph:()=>Promise<any>;salt:()=>Promise<string>|string;listing:import("../../🏛️service/🟨️.mjs").ResidentPortsV1["graph"]["listing"];environmentPaths?:readonly string[];environmentPrefixes?:readonly string[];generation?:(graph:any)=>number|null}):import("../../🏛️service/🟨️.mjs").ResidentPortsV1["graph"];
export function taskClosureV1(graph:unknown,named:readonly string[]):Set<string>;
