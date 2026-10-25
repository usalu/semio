import { createHash } from "node:crypto";
import { resolve } from "node:path";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

function writerShardV1(kind,identity){
 if(!["selection","dataset","canonical"].includes(kind)||typeof identity!=="string"||!(kind==="canonical"?identity==="":/^[a-f0-9]{64}$/u.test(identity)))throw Error("Closed graph namespace writer required");return createHash("sha256").update(kind+":"+identity).digest()[0]%schema.definitions.OptionalLedger.protocol.writerShards;
}

/** 🧭️ Maps admitted inference namespaces into the existing finite fact-writer catalog. */
export function buildGraphWriterResourceV1(workspace,kind,identity){return "repository-graph-facts:"+resolve(workspace)+":shard:"+writerShardV1(kind,identity);}

/** 🔒️ Names the complete selected publication writer shared by readers, publishers and retirement. */
export function buildGraphPublicationWriterResourceV1(graphRoot,scope){
 if(typeof graphRoot!=="string"||graphRoot.length===0||/[\u0000-\u001f]/u.test(graphRoot))throw Error("Closed graph publication root required");return "build-graph-state:"+graphRoot+":selected:"+writerShardV1("selection",scope);
}
