import {lstatSync} from "node:fs";
import {join} from "node:path";
import type {FleetIgnoreV1} from "../../🚧️boundary/🟦️.ts";

/** 👁️ Decides whether an event is outside the shared worktree. */
export function ignoreWatchEventV1(root:string,path:string,ignore:FleetIgnoreV1,tracked:boolean):boolean{
 if(ignore.ignores(path,false))return true;
 if(tracked||!ignore.ignores(path,true))return false;
 try{return lstatSync(join(root,path)).isDirectory();}catch{return false;}
}
