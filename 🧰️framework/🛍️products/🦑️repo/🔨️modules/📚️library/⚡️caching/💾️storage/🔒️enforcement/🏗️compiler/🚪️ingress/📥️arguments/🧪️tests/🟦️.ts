import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {repositoryCargoStorageArgumentsV1,type RepositoryCargoStorageArgumentsV1} from "../🟦️.ts";

test("Cargo writer arguments match neutral fixtures and independent SQLite token observation",()=>{
 const validate=new Ajv({strict:true}),database=new Database(":memory:"),context=corpus.context as RepositoryCargoStorageArgumentsV1;expect(validate.validate(schema.definitions.Context,context)).toBe(true);
 try{for(const row of corpus.cases){expect(validate.validate(schema.definitions.Case,row)).toBe(true);const oracle=database.query<{refused:number},[string,string,string,string]>(`with tokens as(select key,value,lead(value)over(order by key) next from json_each(?1)), head as(select * from tokens where key<coalesce((select min(key) from tokens where value='--'),999999)) select count(*) refused from head where value in('--artifact-dir','--lockfile-path','--root') or substr(value,1,7)='--root=' or substr(value,1,15)='--artifact-dir=' or substr(value,1,16)='--lockfile-path=' or (value='--target-dir' and coalesce(next,'')<>?2) or (substr(value,1,13)='--target-dir=' and substr(value,14)<>?2) or (value='--config' and coalesce(next,'') not in(?3,?4,'build.target-dir='||json_quote(?2))) or (substr(value,1,9)='--config=' and substr(value,10) not in(?3,?4,'build.target-dir='||json_quote(?2)))`).get(JSON.stringify(row.args),context.targetDirectory,context.compilerConfiguration,'build.build-dir='+JSON.stringify(context.buildDirectory))!;expect(oracle.refused===0,row.name).toBe(row.accepted);if(row.accepted)expect(repositoryCargoStorageArgumentsV1(row.args,context)).toEqual(row.args);else expect(()=>repositoryCargoStorageArgumentsV1(row.args,context)).toThrow();}}finally{database.close();}
});
