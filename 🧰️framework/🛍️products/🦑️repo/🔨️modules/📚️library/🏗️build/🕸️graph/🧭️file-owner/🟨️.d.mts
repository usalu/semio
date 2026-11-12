/** 🧭️ Resolves a workspace file to the name of its owning project, or undefined. */
export function createProjectFileIndexV1(projects:Record<string,{root:string}>):(file:string)=>string|undefined;

/** 🚫️ The length of the longest withheld root directory containing a file, or -1. */
export function createWithheldRootIndexV1(roots:Iterable<string>):(file:string)=>number;
