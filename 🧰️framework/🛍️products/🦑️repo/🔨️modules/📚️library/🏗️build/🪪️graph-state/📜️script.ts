import{machinePathV1}from"../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {join,resolve} from "node:path";
import {isMainModuleV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {GRAPH_STATE_POLICY_V1 as policy,graphPlatformV1,graphStateOfV1,type GraphStateKindV1} from "./🟦️.ts";
import {generatedInputsDigestV1} from "./🧾️generated/🟦️.ts";
import {readReplicaFilesV1,replicaHealthyV1} from "./📥️replica/🟦️.ts";
import {readTreeFilesV1,treeMemoFileV1} from "./🌳️tree/🟦️.ts";

const usage=`Usage: bun script.ts <command> [--root <directory>]
  state [--kind portable-graph|nx-graph] [--source auto|replica|tree] [--tooling <hex>] [--implementation <hex>] [--json]
      Prints the identity of the graph inputs of the worktree and how it was reached.
  digest-generated <directory>... [--root <directory>]
      Prints the digest of the bytes of every file below the directories, ignored by git or not (a runtime input of Nx targets).
Verwendung: state gibt die Identität der Graph-Eingaben aus; digest-generated gibt den Digest erzeugter, von Git ignorierter Dateien aus.`;

function parse(args:readonly string[]):{positional:string[];options:Map<string,string>}{
 const positional:string[]=[],options=new Map<string,string>();
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(argument==="--json"){options.set("json","true");continue;}
  if(!argument.startsWith("--")){positional.push(argument);continue;}
  const value=args[++index];
  if(value===undefined||value.startsWith("--"))throw Error(`Option ${argument} needs a value / Option ${argument} braucht einen Wert`);
  options.set(argument.slice(2),value);
 }
 return {positional,options};
}

/** 🧭️ Runs one command of the graph state script and returns the text to print. */
export async function runGraphStateScriptV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{
 const [command,...rest]=args,{positional,options}=parse(rest),root=resolve(options.get("root")??environment.SEMIO_WORKSPACE_ROOT??process.cwd());
 if(command==="digest-generated"){
  if(positional.length===0)throw Error("Name at least one directory / Mindestens ein Verzeichnis angeben");
  return (await generatedInputsDigestV1(root,positional,machinePathV1(root,".cache","f","graph-state-generated.json"))).digest;
 }
 if(command==="state"){
  const kind=(options.get("kind")??"portable-graph") as GraphStateKindV1,source=options.get("source")??"auto",started=performance.now();
  const frame={kind,tooling:options.get("tooling")??"0".repeat(64),implementation:options.get("implementation")??"0".repeat(64),platform:graphPlatformV1(),extra:{}};
  let used="tree",state;
  const replica=source!=="tree"&&await replicaHealthyV1(root)?await readReplicaFilesV1(root).catch(()=>undefined):undefined;
  if(replica!==undefined){used="replica";state=graphStateOfV1(frame,replica.files());}
  else if(source==="replica")throw Error("The replica of this worktree is not healthy / Das Replikat dieses Arbeitsverzeichnisses ist nicht gesund");
  else state=graphStateOfV1(frame,(await readTreeFilesV1(root,{memoFile:treeMemoFileV1(root)})).files);
  const milliseconds=Math.round(performance.now()-started);
  return options.has("json")?JSON.stringify({...state,source:used,milliseconds}):`${state.digest}  source=${used} files=${state.files} content=${state.contentFiles} ${milliseconds} ms (${policy.domain})`;
 }
 throw Error(usage);
}

if(isMainModuleV1(import.meta.url)){
 try{process.stdout.write(await runGraphStateScriptV1(process.argv.slice(2))+"\n");}
 catch(error){process.stderr.write(String(error instanceof Error?error.message:error)+"\n");process.exitCode=1;}
}
