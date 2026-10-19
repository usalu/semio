#!/usr/bin/env bun
import {mkdir} from "node:fs/promises";
import {spawn} from "node:child_process";
import {join} from "node:path";
import {BundleScript,ScriptRouter,scriptInvocationBudget,type ScriptCommand,type ScriptInvocation} from "../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {runScriptMain} from "../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import {runOwnedCommand} from "../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {executeDeveloperRequest,parseDeveloperArguments,developerDebuggers,type DeveloperPlan,type DeveloperProcess} from "./🟦️.ts";

const messages={nativePath:{en:"This Windows GDB profile requires ASCII native program and working-directory paths; the installed MinGW build cannot launch Unicode target paths. Check the LLDB installation in Debugger Availability before selecting it for Unicode native targets; missing LLDB still needs setup.",de:"Dieses Windows-GDB-Profil erfordert ASCII-Pfade für natives Programm und Arbeitsverzeichnis; der installierte MinGW-Build kann Unicode-Zielpfade nicht starten. Vor der Wahl von LLDB für native Unicode-Ziele dessen Installation mit Debugger-Verfügbarkeit prüfen; fehlendes LLDB muss zuerst eingerichtet werden."},debugMissing:{en:"The debugger is unavailable on PATH. Run Debugger Availability, choose an available profile, or install the named debugger and restart this task.",de:"Der Debugger ist auf PATH nicht verfügbar. Debugger-Verfügbarkeit ausführen, ein verfügbares Profil wählen oder den genannten Debugger installieren und diese Aufgabe neu starten."},available:{en:"Executable installed; target support depends on the selected program",de:"Programm installiert; Zielunterstützung hängt vom gewählten Programm ab"},unavailable:{en:"Unavailable: install this program or choose another profile",de:"Nicht verfügbar: Dieses Programm installieren oder ein anderes Profil wählen"},inventory:{en:"Installed debugger profiles",de:"Installierte Debugger-Profile"},argumentLimit:{en:"Program arguments must be a JSON list of up to 64 strings and 8192 characters in total.",de:"Programmargumente müssen eine JSON-Liste aus höchstens 64 Zeichenketten und insgesamt 8192 Zeichen sein."},invalid:{en:"Invalid or duplicate parameters. Check the required values and supported choices.",de:"Ungültige oder doppelte Parameter. Erforderliche Werte und unterstützte Optionen prüfen."},path:{en:"Choose an existing workspace-relative file or directory without traversal or symlink escape.",de:"Eine vorhandene workspace-relative Datei oder ein Verzeichnis ohne Pfadtraversierung oder Symlink-Ausbruch wählen."},revision:{en:"Choose a revision or range without leading options or command separators.",de:"Eine Revision oder einen Bereich ohne führende Optionen oder Befehlstrennzeichen wählen."},opening:{en:"Opening source file",de:"Quelldatei wird geöffnet"},running:{en:"Running developer command",de:"Entwicklerbefehl wird ausgeführt"},missing:{en:"The selected program is unavailable. Choose the system editor or shell, or an already installed program.",de:"Das gewählte Programm ist nicht verfügbar. Wähle den Systemeditor oder die System-Shell, oder ein bereits installiertes Programm."},line:{en:"A line requires VSCode, Codium or Sublime; the system file association has no line capability.",de:"Eine Zeile erfordert VSCode, Codium oder Sublime; die Systemdateizuordnung unterstützt keine Zeilennavigation."}};
function text(id:keyof typeof messages):string {const language=process.env.SEMIO_LOCALE;return language==="en"||language==="de"?messages[id][language]:`${messages[id].en} / ${messages[id].de}`;}

function detachedDesktop(plan:DeveloperPlan,signal:AbortSignal):Promise<void> {
 signal.throwIfAborted();return new Promise((accept,reject)=>{const child=spawn(plan.command,plan.args,{cwd:plan.cwd,env:process.env,stdio:"ignore",detached:true,windowsHide:false});const cancel=():void=>{child.kill();reject(signal.reason);};signal.addEventListener("abort",cancel,{once:true});child.once("error",error=>{signal.removeEventListener("abort",cancel);reject(error);});child.once("spawn",()=>{signal.removeEventListener("abort",cancel);if(signal.aborted){child.kill();reject(signal.reason);return;}child.unref();accept();});});
}

/** 🛠️ Every developer operation keeps the original task's streamed output and cancellation. */
function developerCommand(command:string):ScriptCommand {
 return class extends BundleScript {
  async run(segments:string[]):Promise<void>{
   let request;try{request=parseDeveloperArguments([command,...segments]);}catch(error){throw Error(text(error instanceof Error&&error.message==="developer.debug.arguments-too-large"?"argumentLimit":"invalid"));}const environment={...process.env};
   if(command.startsWith("git-")){for(const key of Object.keys(environment))if(key.startsWith("GIT_"))delete environment[key];environment.GIT_OPTIONAL_LOCKS="0";environment.GIT_TERMINAL_PROMPT="0";}
   const run=async(plan:DeveloperPlan,signal:AbortSignal):Promise<void>=>{if(!Bun.which(plan.command,{PATH:environment.PATH??environment.Path??""}))throw Object.assign(Error(plan.command),{code:"ENOENT"});return runOwnedCommand(plan.command,plan.args,plan.cwd,"developer-tools",scriptInvocationBudget(this.invocation,(command==="terminal"||command==="debug")?0:command==="open-source"?5_000:30_000),{signal,env:environment,output:{maximumBytes:16*1024*1024,maximumLines:200_000},onProgress:command==="terminal"||command==="debug"?()=>{}:undefined});};
   const port:DeveloperProcess={run,open:async(plan,signal)=>{console.log(`[developer-tools] ${text("opening")}`);await(plan.detached?detachedDesktop(plan,signal):run(plan,signal));}};
   console.log(`[developer-tools] ${text("running")}: ${command}`);
   try{await executeDeveloperRequest(this.repoRoot,request,port,this.invocation.control.signal,process.platform,environment);}catch(error){if((error as {code?:string}).code==="ENOENT"&&request.command==="debug")throw Error(`${text("debugMissing")} (${request.profile}: ${(error as Error).message})`);if((error as {code?:string}).code==="ENOENT")throw Error(`${text("missing")} (${request.command==="open-source"?request.editor:request.command==="terminal"?request.shell:"git"})`);if(error instanceof Error&&error.message==="developer.debug.windows-gdb-unicode")throw Error(text("nativePath"));if(error instanceof Error&&error.message==="developer.editor.line-requires-capability")throw Error(text("line"));if(error instanceof Error&&error.message.startsWith("developer.path."))throw Error(text("path"));if(error instanceof Error&&error.message==="developer.git.revision-invalid")throw Error(text("revision"));throw error;}
  }
 };
}

/** 🔎️ Lists fixed debugger profiles and their actual installed executable locations. */
class DebuggersScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error(text("invalid"));console.log(`[developer-tools] ${text("inventory")}`);
  for(const row of developerDebuggers(process.platform,program=>Bun.which(program,{PATH:process.env.PATH??process.env.Path??""}))){const language=process.env.SEMIO_LOCALE,title=language==="en"||language==="de"?row.title[language]:`${row.title.en} / ${row.title.de}`;console.log(`${row.profile}: ${title} — ${row.installed?text("available"):text("unavailable")} (${row.path??row.program})`);if(row.constraint&&row.installed)console.log(`  ${text("nativePath")}`);}
 }
}

class TestScript extends BundleScript {
 async run():Promise<void>{const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR??join(this.repoRoot,".🧬semio/🦑️repo/⚡️cache/tests/developer-tools");await mkdir(artifacts,{recursive:true});await runOwnedCommand(process.execPath,["test",join(this.root,"🧪️tests/🟦️.ts"),"--timeout","300000"],this.repoRoot,"developer-tools:test",scriptInvocationBudget(this.invocation,300_000),{signal:this.invocation.control.signal,env:{...process.env,SEMIO_TEST_ARTIFACT_DIR:artifacts}});}
}

/** 🚪️ The calling composition supplies its exact workspace and original operation authority. */
export async function runDeveloperTools(root:string,argv:readonly string[],invocation:ScriptInvocation):Promise<void>{
 const router=new ScriptRouter(import.meta.dir,root);for(const command of ["open-source","git-status","git-diff","git-history","git-show","git-blame","terminal","debug"])router.register(command,developerCommand(command));router.register("debuggers",DebuggersScript).register("test",TestScript);await runScriptMain(router,{invocation,argv});
}
if(import.meta.main){const root=new ScriptRouter(import.meta.dir).repoRoot;await receiveScriptProcessInvocation(process.env,invocation=>runDeveloperTools(root,process.argv.slice(2),invocation));}
