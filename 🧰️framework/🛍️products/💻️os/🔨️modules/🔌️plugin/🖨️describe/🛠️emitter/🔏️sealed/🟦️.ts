export type SealedFileV1=Readonly<{path:string;sha256:string}>;

/** 🔏️ Verifies a sealed descriptor compiler against its own recorded digests and names every path with both digests; the live worktree is never consulted because the sealed binary is the product of its recorded inputs. */
export async function assertSealedFilesV1(files:Iterable<SealedFileV1>,digest:(path:string)=>Promise<string|undefined>):Promise<void>{
 const drifted:string[]=[];for(const file of files){const actual=await digest(file.path);if(actual!==file.sha256)drifted.push(file.path+" expected "+file.sha256+" actual "+(actual??"absent"));}
 if(drifted.length)throw Error("Sealed descriptor compiler file changed: "+drifted.join("; "));
}
