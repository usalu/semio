import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {readdir} from "node:fs/promises";
import {homedir} from "node:os";
import {join} from "node:path";
import type {CasAreaSourceV1} from "../🧹️collector/🟦️.ts";
import {holdCasPinsV1} from "../📌️pins/🟦️.ts";
import {CAS_GRAPH_CLOSURE_AREA_V1,CAS_POLICY_V1} from "../🧬️schema/🟦️.ts";
import {casRootV1} from "../🗃️store/🟦️.ts";

/** 📂️ The loose areas of the user that count against the host cap whatever workspace asks: every graph memo directory of the user's worktrees below `<home>/.cache/semio/repo/<worktree>/journal/graph-memo`. */
export async function casHostAreasV1(home:string=homedir(),unreadable?:()=>void):Promise<readonly CasAreaSourceV1[]>{
 const base=join(home,".cache","semio","repo"),areas:CasAreaSourceV1[]=[];
 let worktrees:string[]=[];try{worktrees=(await readdir(base)).sort();}catch(error){if((error as NodeJS.ErrnoException)?.code!=="ENOENT")unreadable?.();}
 for(const worktree of worktrees)areas.push({id:"graph-memo",directory:join(base,worktree,"journal","graph-memo")});
 return Object.freeze(areas);
}

/** 📂️ The loose areas a workspace contributes to the host cap: its graph-owned build-input closures, then the host areas of the user. */
export async function casAreasOfWorkspaceV1(workspace:string,home:string=homedir()):Promise<readonly CasAreaSourceV1[]>{
 return Object.freeze([{id:CAS_GRAPH_CLOSURE_AREA_V1,directory:machinePathV1(workspace,".cache","b","g","m",CAS_GRAPH_CLOSURE_AREA_V1)},...await casHostAreasV1(home)]);
}

/** 🧷️ Pins the build-input closures a loaded graph generation references (`digests` are the file names without `.json`) in the host store of the environment's user, so that neither `cache gc` nor `cache cas gc` collects a leaf the live graph service would read next; without a user home there is no host store and nothing to pin. */
export async function casPinClosuresV1(digests:Iterable<string>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<void>{
 const home=environment.HOME||environment.USERPROFILE;if(home===undefined)return;
 await holdCasPinsV1(casRootV1(environment,process.platform,home),CAS_POLICY_V1,"graph-closures."+process.pid,digests);
}
