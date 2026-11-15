import {createRequire} from "node:module";

/** 🧰️ Drives Nx's own native HTTP cache client (the third-party oracle of the single-flight front) from a child process, because a refused panic of that client aborts the whole host process. Usage: `<module path> retrieve|store <hash> <cache directory>`; prints one JSON line. */
const [path,mode,hash,directory]=process.argv.slice(2),native=createRequire(import.meta.url)(path),client=new native.HttpRemoteCache(),started=Date.now();
try{
 if(mode==="retrieve"){const result=await client.retrieve(hash,directory);console.log(JSON.stringify({ok:true,found:result!==null&&result!==undefined,milliseconds:Date.now()-started}));}
 else{const stored=await client.store(hash,directory,"built\n",0);console.log(JSON.stringify({ok:true,stored,milliseconds:Date.now()-started}));}
}catch(error){console.log(JSON.stringify({ok:false,error:String(error?.message??error)}));}
