import{runNativeDirectoryActionV1}from"./🟦️.ts";
/** 🐧️ Runs one closed directory-partition action as the owned unprivileged replacement of the former image tools. */
export function runNativeDirectoryScriptV1(argv:readonly string[]):number{try{process.stdout.write(runNativeDirectoryActionV1(argv));return 0;}catch(error){process.stderr.write((error instanceof Error?error.message:String(error))+"\n");return 1;}}
if(import.meta.main)process.exitCode=runNativeDirectoryScriptV1(process.argv.slice(2));
