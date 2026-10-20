/** 🧭️ Resolves a workspace file to the name of its owning project, or undefined. */
export function createProjectFileIndexV1(projects:Record<string,{root:string}>):(file:string)=>string|undefined;
