import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./retirement-poll.json";
import schema from "./retirement-poll-schema.json";
test("every generated original close family retains Pending as a distinct result",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("create table polls(family text,port text,output text)");for(const row of law.cases)for(const port of law.ports)db.run("insert into polls values(?,?,?)",row.family,port,law.output);expect(db.query("select count(*) ports,count(distinct family) families,count(distinct output) outputs from polls").get()).toEqual({ports:6,families:2,outputs:1});expect(law.pending).toEqual({completed:false,keepsOriginalAction:true,replaysChild:false})}finally{db.close()}});
