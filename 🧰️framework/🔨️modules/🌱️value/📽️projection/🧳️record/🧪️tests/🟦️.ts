import deriveLayouts from "../../../✨️derive/📽️projection/🧪️tests/🔣️.json";
import deriveSchema from "../../../✨️derive/📽️projection/🧬️schema/🔣️.json";
import {recordProjectionLayoutAccepted} from "../🟦️.ts";
import {Database} from "bun:sqlite";
import {stableRecordProjectionDemand} from "../🟦️.ts";
import Ajv from 'ajv';
import jsonPatch from 'fast-json-patch';
import corpus from './🧫️fixtures/🔣️.json';
import schema from '../🧬️schema/🔣️.json';
/** 🧳️ Verifies complete original field outputs with independent RFC 6902 and UTF-8 bytes. */
export function verifyOriginalRecordProjectionOracle(){
 const validate=new Ajv({strict:true}).compile(schema);if(!validate(corpus))throw Error(JSON.stringify(validate.errors));
 const validateLayouts=new Ajv({strict:true}).compile(deriveSchema.properties.layouts);if(!validateLayouts(deriveLayouts.layouts))throw Error("record layout schema");
 const db=new Database(":memory:");try{const alignment=db.query("SELECT NOT EXISTS(SELECT 1 FROM json_each(?) WHERE value='packed') AS accepted");for(const row of deriveLayouts.layouts){if(recordProjectionLayoutAccepted(row.representations)!==row.accepted)throw Error("record layout model");const independent=alignment.get(JSON.stringify(row.representations))as{accepted:number};if(Boolean(independent.accepted)!==row.accepted)throw Error("SQL record layout policy");}const query=db.query("SELECT 2*? AS birthCopyBytes,? AS birthCapacityBytes,2*?+? AS takeCopyBytes,2*?+2*? AS placementCopyBytes,? AS placementReleaseBytes");for(const row of corpus.stableOutput){const actual=stableRecordProjectionDemand(row);if(JSON.stringify(actual)!==JSON.stringify(row.expected))throw Error("stable owner demand");const independent=query.get(row.cursorBytes,row.recordBytes,row.pointerBytes,row.metadataBytes,row.fieldBytes,row.pointerBytes,row.fieldBytes);if(JSON.stringify(independent)!==JSON.stringify(row.expected))throw Error("SQL stable owner demand");const fits=Object.values(actual).every(n=>n<=corpus.maximumBytes);if(fits!==row.fits)throw Error("stable projection extent qualification");}}finally{db.close();}
 for(const law of corpus.cases){const operations=law.values.map((value,index)=>({op:'add' as const,path:'/f'+index.toString().padStart(2,'0'),value:Buffer.from(value,'utf8').toString('utf8')}));const output=jsonPatch.applyPatch({} as Record<string,string>,operations,true,false).newDocument;for(let index=0;index<law.expected.length;index++)if(output['f'+index.toString().padStart(2,'0')]!==law.expected[index])throw Error(law.id+' independent original field '+index);}
}
