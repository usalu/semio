import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import rows from"./🧫️fixtures/actor-failure.json"with{type:"json"};
import schema from"../🧬️schema/actor-failure.json"with{type:"json"};
test("only the authenticated ActorFailure recipient can consume retirement funding",()=>{expect(new Ajv({strict:true}).compile(schema)(rows)).toBe(true);const db=new Database(":memory:");try{for(const row of rows){const actual=(db.query("SELECT ? AND ? AND ? AND ? AND ? AND ? AS accepted").get(+row.verified,+row.faultView,+row.exactDescriptor,+row.source,+row.recipient,+row.retirement)as{accepted:number}).accepted;expect(Boolean(actual)).toBe(row.accepted);}}finally{db.close();}});
