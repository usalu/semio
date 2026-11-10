/** 🏠 Where the one graph service of this host and user lives; one socket per sealed implementation digest so an old and a new service coexist while one retires. */
import {createHash} from "node:crypto";
import {tmpdir,userInfo} from "node:os";
import {join} from "node:path";
import {canonicalRootV1,worktreeIdV1} from "../🟨️.mjs";

/** 🔢 The workspace identity of a checkout: the twelve hex digits of its canonical root. */
export function workspaceIdV1(root){return worktreeIdV1(canonicalRootV1(root));}

/** 📍 Runtime directory, socket or pipe, key, descriptor and start lock of the host service of one implementation; paths stay below 96 bytes so they fit `sockaddr_un`. See https://pkg.go.dev/os#TempDir */
export function hostEndpointsV1(implementation,environment=process.env,platform=process.platform,user=undefined){
 const owner=user??(()=>{try{return String(userInfo().uid);}catch{return"0";}})(),tag=createHash("sha256").update("semio-repo-host-v1\0"+implementation).digest("hex").slice(0,12);
 if(platform==="win32"){
  const directory=join(environment.LOCALAPPDATA||tmpdir(),"semio","repo","host");
  return{tag,directory,endpoint:"\\\\.\\pipe\\semio-repo-host-"+owner+"-"+tag,key:join(directory,"key"),descriptor:join(directory,"host-"+tag+".json"),lock:join(directory,"start-"+tag+".lock"),log:join(directory,"host-"+tag+".log")};
 }
 const base=environment.XDG_RUNTIME_DIR||(environment.TMPDIR||"/tmp").replace(/\/+$/u,"");
 let directory=join(base,"semio-repo",owner);
 if(Buffer.byteLength(join(directory,"h-"+tag+".sock"))>96)directory=join("/tmp","semio-repo",owner);
 return{tag,directory,endpoint:join(directory,"h-"+tag+".sock"),key:join(directory,"key"),descriptor:join(directory,"host-"+tag+".json"),lock:join(directory,"start-"+tag+".lock"),log:join(directory,"host-"+tag+".log")};
}
