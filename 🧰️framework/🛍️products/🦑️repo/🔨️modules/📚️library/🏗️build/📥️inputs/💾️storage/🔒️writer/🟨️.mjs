import { createHash } from "node:crypto";
import { resolve } from "node:path";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

function writerShardV1(kind,identity){
 if(!["selection","dataset","canonical"].includes(kind)||typeof identity!=="string"||!(kind==="canonical"?identity==="":/^[a-f0-9]{64}$/u.test(identity)))throw Error("Closed graph namespace writer required");return createHash("sha256").update(kind+":"+identity).digest()[0]%schema.definitions.OptionalLedger.definitions.Protocol.const.writerShards;
}

/** 🧭️ Maps admitted inference namespaces into the existing finite fact-writer catalog. */
export function buildGraphWriterResourceV1(workspace,kind,identity){return "repository-graph-facts:"+resolve(workspace)+":shard:"+writerShardV1(kind,identity);}

/** 🔒️ Names the complete selected publication writer shared by readers, publishers and retirement. */
export function buildGraphPublicationWriterResourceV1(graphRoot,scope){
 if(typeof graphRoot!=="string"||graphRoot.length===0||/[\u0000-\u001f]/u.test(graphRoot))throw Error("Closed graph publication root required");return "build-graph-state:"+graphRoot+":selected:"+writerShardV1("selection",scope);
}

/** 🪪️ Names the exclusive claim on one selected scope for the whole of its load, in a fixed catalog of buckets so that two different scopes only meet by hash chance and never share the fact shard's hours-long hold. */
export function buildGraphScopeWriterResourceV1(graphRoot,scope){
 if(typeof graphRoot!=="string"||graphRoot.length===0||/[\u0000-\u001f]/u.test(graphRoot)||typeof scope!=="string"||!/^[a-f0-9]{64}$/u.test(scope))throw Error("Closed graph scope writer required");return "build-graph-state:"+graphRoot+":scope:"+createHash("sha256").update("scope:"+scope).digest().readUInt16BE(0)%schema.definitions.GraphScopeWriterProtocol.const.buckets;
}

/** 🔎️ Reads the lock a graph writer resource names: its kind (`facts` for the fact shard, `publication` for the selected publication, `scope` for the scope claim) and its number, or undefined for any other resource. */
export function describeGraphWriterResourceV1(resource){
 if(typeof resource!=="string")return undefined;const facts=/^repository-graph-facts:.*:shard:(\d{1,2})$/u.exec(resource),published=/^build-graph-state:.*:selected:(\d{1,2})$/u.exec(resource),scope=/^build-graph-state:.*:scope:(\d{1,4})$/u.exec(resource),row=facts?["facts",facts[1]]:published?["publication",published[1]]:scope?["scope",scope[1]]:undefined;
 return row?Object.freeze({kind:row[0],number:Number(row[1])}):undefined;
}
