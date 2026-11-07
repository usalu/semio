import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {portableMsvcEligibilityV1} from "../🟦️.ts";

test("native Windows eligibility preserves explicit toolchain and target choices",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true}),valid=ajv.compile(schema.definitions.EligibilityCase),db=new Database(":memory:"),p=schema.definitions.Protocol.const;
 try{for(const row of cases.cases){expect(valid(row),row.name).toBe(true);const matches=db.query("select count(*) n from json_each(?1) e where exists(select 1 from json_each(?2) n where lower(n.value)=lower(e.key)) or exists(select 1 from json_each(?3) n where lower(e.key) glob lower(n.value))").get(JSON.stringify(row.environment),JSON.stringify(p.explicitEnvironment),JSON.stringify(p.explicitPatterns)) as {n:number},expected=row.platform!=="win32"?"platform":row.architecture!=="x64"?"architecture":row.target!==null&&row.target!==p.target?"target":matches.n?"override":"eligible";expect(expected,row.name).toBe(row.reason);expect(portableMsvcEligibilityV1(row.environment,{platform:row.platform,architecture:row.architecture,target:row.target}),row.name).toBe(row.reason);}}finally{db.close();}
});

