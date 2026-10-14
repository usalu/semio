import {randomBytes} from "node:crypto";
import {constants,closeSync,existsSync,fstatSync,lstatSync,mkdirSync,openSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {acquireQueuedResourceLease,checkResourceLeaseControl,requireMaterializedPathV1,type LeaseWait,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🎯️ Binds one physical Cargo output directory to its immutable compiler store before shared writers begin. */
export async function bindCargoTargetStoreV1(options:Readonly<{directory:string;buildDirectory:string;targetDirectory:string;signal:AbortSignal;control?:ResourceLeaseControl;onWait?:(progress:LeaseWait)=>void|Promise<void>}>):Promise<void>{
    const check=()=>{options.signal.throwIfAborted();if(options.control)checkResourceLeaseControl(options.control)};
    const protocol=schema.definitions.Protocol.const,receipt=requireMaterializedPathV1(join(options.targetDirectory,protocol.receipt)),canonical=JSON.stringify({version:1,buildDirectory:options.buildDirectory,targetDirectory:options.targetDirectory})+"\n";
    const lease=await acquireQueuedResourceLease({directory:options.directory,resource:JSON.stringify([protocol.namespace,options.targetDirectory]),mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:options.signal,control:options.control,onWait:options.onWait});
    try{
        check();if(existsSync(options.targetDirectory)){const stat=lstatSync(options.targetDirectory);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Cargo target directory is not a physical owner")}
        mkdirSync(options.targetDirectory,{recursive:true});check();
        if(!existsSync(receipt)){
            const temporary=requireMaterializedPathV1(join(options.targetDirectory,".c-"+randomBytes(4).toString("hex")));
            try{writeFileSync(temporary,canonical,{flag:"wx",mode:0o600});check();if(existsSync(receipt))throw Error("Cargo target store receipt changed during its original admission");renameSync(temporary,receipt)}finally{rmSync(temporary,{force:true})}
        }
        const stat=lstatSync(receipt);if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==1||stat.size>protocol.maximumReceiptBytes)throw Error("Cargo target store receipt is not an immutable physical owner");
        const descriptor=openSync(receipt,constants.O_RDONLY|(constants.O_NOFOLLOW??0));
        try{const current=fstatSync(descriptor);if(current.dev!==stat.dev||current.ino!==stat.ino||current.size!==stat.size||current.nlink!==1||readFileSync(descriptor,"utf8")!==canonical)throw Error("Cargo target belongs to a different compiler store")}finally{closeSync(descriptor)}
        check();
    }finally{lease.release()}
}
