import {loadFrameworkOsPlaygroundCatalog} from "../../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/📚️catalog/🟦️.ts";

/** 🎯️ Resolves the authored activation producer for one admitted playground runtime. */
export function playgroundActivationTargetV1(variant:string,profile:"prototype"|"release",renderer:"react"|"wgpu",catalog:readonly {readonly variant:string}[]=loadFrameworkOsPlaygroundCatalog()):string{
 if(!catalog.some(row=>row.variant===variant))throw Error(`Missing generated playground ${variant}`);
 return `@semio-tech/framework-os-dev:activate-${variant}-${renderer}-${profile}`;
}
