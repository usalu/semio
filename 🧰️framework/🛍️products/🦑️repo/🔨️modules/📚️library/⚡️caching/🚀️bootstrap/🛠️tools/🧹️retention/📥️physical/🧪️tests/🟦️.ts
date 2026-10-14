import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("managed tools retirement requires Original release and refuses current, foreign or unregistered readers",async()=>{const implementation=await import("../🟦️.ts"),db=new Database(":memory:"),ajv=new Ajv({strict:true});try{for(const row of corpus.cases){const{expected,...observation}=row;expect(ajv.validate(schema.definitions.Observation,observation)).toBe(true);const oracle=db.query<{allowed:number},[number,number,number,number,number]>("select not ?1 and ?2 and ?3 and not ?4 and ?5 allowed").get(Number(row.current),Number(row.authenticated),Number(row.readerProtocol),Number(row.witness),Number(row.exclusive))!;expect(Boolean(oracle.allowed)).toBe(expected);expect(implementation.managedToolingRetirementAllowedV1(observation)).toBe(expected);}}finally{db.close();}});
