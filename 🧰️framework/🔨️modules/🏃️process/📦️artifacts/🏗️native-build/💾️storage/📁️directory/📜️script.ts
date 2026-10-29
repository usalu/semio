import{requireCapturedRuntimeEntryProofV1}from"../../../../🎛️owned-execution/📥️capture/🚪️entry/🔏️proof/🟦️.ts";
import type{RuntimeEntryDeclarationV1}from"../../../../🎛️owned-execution/📥️capture/🚪️entry/🟦️.ts";
import{runNativeDirectoryActionV1}from"./🟦️.ts";
/** 🐧️ Runs one closed directory-partition action as the owned unprivileged replacement of the former image tools. */
export function runNativeDirectoryScriptV1(argv:readonly string[]):number{try{process.stdout.write(runNativeDirectoryActionV1(argv));return 0;}catch(error){process.stderr.write((error instanceof Error?error.message:String(error))+"\n");return 1;}}


/** 🚪️ Declares the actual helper as a static captured runtime dependency. */
export function nativeDirectoryRuntimeEntryV1():RuntimeEntryDeclarationV1{return Object.freeze({version:1,moduleURL:import.meta.url,...((import.meta as{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL?{physicalSourceURL:(import.meta as{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL}:{}),specifier:"./📜️script.ts"});}
/** 🔐️ Executes only the immutable entry authenticated by its actual loader. */
export function runCapturedBuildSourceV1(args:readonly string[],proof:Parameters<typeof requireCapturedRuntimeEntryProofV1>[0]):void{requireCapturedRuntimeEntryProofV1(proof,(import.meta as{semioCapturedImplementationIdentity?:unknown}).semioCapturedImplementationIdentity,(import.meta as{semioCapturedLoaderIdentity?:unknown}).semioCapturedLoaderIdentity);process.exitCode=runNativeDirectoryScriptV1(args);proof.recheck();}
