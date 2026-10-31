/** 📊️ The runtime-neutral process identity used to bound a diagnostic output name. */
export interface RepoCpuProfileContextV1{readonly platform:string;readonly pid:number;readonly sequence:number}
/** 🎛️ Validates one closed diagnostic request and returns only fixed CPU profile arguments. */
export function repoCpuProfileArgumentsV1(environment:Readonly<Record<string,string|undefined>>,context:RepoCpuProfileContextV1):readonly string[];
/** 📁️ Prepares only the explicitly supplied artifact directory for a captured-command CPU profile. */
export function prepareRepoCpuProfileArgumentsV1(environment:Readonly<Record<string,string|undefined>>):readonly string[];

/** 🍞️ Returns the fixed owned sampler preload for one declared test profile. */
export function prepareRepoCpuTestProfileArgumentsV1(environment:Readonly<Record<string,string|undefined>>):readonly string[];
