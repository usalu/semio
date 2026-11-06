import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "./🔣️budget.json" with {type:"json"};
import schema from "./🧬️budget-schema.json" with {type:"json"};
if(!new Ajv().validate(schema,cases))throw Error("invalid Base64 budget corpus");
const db=new Database(":memory:");
const query=db.query("with input(left_bytes,budget,phase,decoded) as (values(?1-?2,?3,?4,?5-(?2/4)*3)), quantum(n) as (select min(4096,left_bytes,(budget/4)*4) from input) select n as work,case when phase='decode' then min(decoded,(n/4)*3) else 0 end as copied from input,quantum");
for(const row of cases){const actual=query.get(row.length,row.offset,row.budget,row.phase,row.output) as {work:number;copied:number};if(actual.work!==row.work||actual.copied!==row.copy)throw Error(JSON.stringify({row,actual}));}
console.log(`Base64 budget oracle: ${cases.length} cases`);
