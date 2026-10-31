import {mkdirSync,lstatSync} from "node:fs";

/** 🔐 Creates or admits a real private resident runtime directory owned by this account. */
export function preparePrivateDirectoryV1(path){
 const posix=process.platform!=="win32";
 mkdirSync(path,{recursive:true,mode:0o700});
 const status=lstatSync(path);
 if(!status.isDirectory()||status.isSymbolicLink()||posix&&(status.uid!==process.getuid?.()||(status.mode&0o077)!==0))throw Error("Runtime directory "+path+" must be a private directory owned by the current user (mode 0700)");
}
