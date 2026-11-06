import Ajv from "ajv";
import {createHash} from "node:crypto";
import cases from "./🔣️digest.json" with {type:"json"};
import schema from "./🧬️digest-schema.json" with {type:"json"};
if(!new Ajv().validate(schema,cases))throw Error("invalid original digest corpus");
for(const row of cases){const domain=Buffer.from("edit-id"),dl=Buffer.alloc(8),il=Buffer.alloc(8);dl.writeBigUInt64BE(BigInt(domain.length));il.writeBigUInt64BE(BigInt(row.count));const digest=createHash("sha256").update("semio.artifact.cursor.v2").update(dl).update(domain).update(il).update(row.byte.repeat(row.count)).digest("hex");if(digest!==row.sha256)throw Error("original digest oracle mismatch");}
console.log(`original digest oracle: ${cases.length} cases`);
