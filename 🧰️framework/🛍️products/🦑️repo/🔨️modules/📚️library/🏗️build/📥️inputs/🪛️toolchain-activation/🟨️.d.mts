/** 🪛️ Names the toolchains whose provisioning targets the given targets depend on. */
export function toolchainsActivatedByV1(targets:Readonly<Record<string,{readonly dependsOn?:readonly (string|{readonly target?:string})[]}>>|undefined,toolchains:Readonly<Record<string,{readonly activatedBy?:readonly string[]}>>|undefined):string[];
