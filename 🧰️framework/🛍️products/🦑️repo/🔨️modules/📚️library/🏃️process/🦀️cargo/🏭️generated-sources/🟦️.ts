import {readFileSync} from "node:fs";
import {dirname,isAbsolute,join,relative,sep} from "node:path";
import {manifestClosureV1} from "../../../⚡️caching/🦀️cargo/🧮️manifest-closure/🟦️.ts";
import {runRepositoryCommand} from "../../🎛️owned-execution/🟦️.ts";
import {nativeCommandPolicyV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {GeneratorContract} from "../../../🔍️discovery/📦️package-projection-contracts/🟦️.ts";

/** 🏭️ One owner script that regenerates the ignored Rust sources a compiled crate mounts. */
export type GeneratedSourceProducerV1=Readonly<{id:string;target:string;cwd:string;args:readonly string[]}>;
type Contracts=Readonly<Record<string,GeneratorContract>>;
const posix=(path:string):string=>path.split(sep).join("/");
const argument=/^[^\s"'`$&|;<>\\]+$/u;

/** 📜️ Reads the generator contracts of the repository taxonomy without validating the rest of it. */
export function readGeneratorContractsV1(workspaceRoot:string):Contracts{
 return (JSON.parse(readFileSync(join(workspaceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),"utf8")) as {generatorContracts:Contracts}).generatorContracts;
}

/** 🔎️ Resolves the `repo ./📜️script.ts <args>` command the owner project declares for the producer target of one contract. */
function producerV1(workspaceRoot:string,id:string,contract:GeneratorContract):GeneratedSourceProducerV1{
 const target=contract.target!,separator=target.lastIndexOf(":"),name=target.slice(separator+1),project=JSON.parse(readFileSync(join(workspaceRoot,contract.ownerPath!,"📋️project.json"),"utf8")) as {name:string;targets:Record<string,{executor?:string;options?:{cwd?:string;command?:string}}>};
 const declared=project.targets[name],tokens=declared?.options?.command?.split(" ")??[];
 if(project.name!==target.slice(0,separator)||declared?.executor!=="repo:script"||typeof declared.options?.cwd!=="string"||tokens[0]!=="repo"||!tokens[1]?.startsWith("./")||!tokens.slice(1).every(token=>argument.test(token)))throw Error(`Generated source producer ${JSON.stringify(id)} needs a plain "repo ./<script> <args>" repo:script target: ${target}`);
 return Object.freeze({id,target,cwd:declared.options.cwd,args:Object.freeze(tokens.slice(1))});
}

/** 🧭️ Selects the producers of every ingress-prepared contract whose native consumer lies in the local crate closure of the workspace of a manifest, since Cargo may compile any member; a manifest outside the repository selects none.
 * @see ../../../🔣️taxonomy.json `ingressPreparation` */
export function generatedSourceProducersV1(workspaceRoot:string,manifest:string,contracts:Contracts=readGeneratorContractsV1(workspaceRoot)):readonly GeneratedSourceProducerV1[]{
 const directory=posix(relative(workspaceRoot,dirname(manifest)));
 if(isAbsolute(directory)||directory===".."||directory.startsWith("../"))return[];
 const crates=new Set(manifestClosureV1(workspaceRoot,[directory],{scope:"workspace",tests:true}).crates.map(crate=>crate.directory));
 return Object.entries(contracts).filter(([,contract])=>contract.ingressPreparation===true&&contract.ownership==="owned"&&contract.nativeConsumers?.some(consumer=>crates.has(consumer))).map(([id,contract])=>producerV1(workspaceRoot,id,contract)).sort((left,right)=>left.id.localeCompare(right.id));
}

/** 🏗️ Regenerates the ignored Rust sources a `repo cargo` compile mounts before Cargo reads them; the graph already scheduled the same producers for a task, so a task child runs none.
 * @see ../../../../../📓️build-guide.md */
export async function prepareGeneratedSourcesV1(invocation:ScriptInvocation,workspaceRoot:string,manifest:string,environment:Readonly<Record<string,string|undefined>>=process.env,contracts?:Contracts):Promise<readonly GeneratedSourceProducerV1[]>{
 if(environment.SEMIO_TASK_PROJECT!==undefined)return[];
 const producers=generatedSourceProducersV1(workspaceRoot,manifest,contracts);
 for(const producer of producers)await runRepositoryCommand(invocation,"repo",[...producer.args],join(workspaceRoot,producer.cwd),`generated-source:${producer.id}`,nativeCommandPolicyV1({maximumBytes:1048576,maximumLines:4096}));
 return producers;
}
