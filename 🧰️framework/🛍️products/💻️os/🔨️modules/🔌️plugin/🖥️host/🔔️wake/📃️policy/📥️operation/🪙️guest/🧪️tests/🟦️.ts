import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
test("guest holds, observed work and producer forfeits stay disjoint from host cleanup and transit",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){const oracle=fixture.total.map((total,index)=>(db.query("SELECT ?-?-?-?-?-? AS available").get(total,fixture.cleanup[index],fixture.transit[index],row.observed[index],row.forfeited[index],row.held?fixture.hold[index]:0)as{available:number}).available);expect(oracle,row.id).toEqual(row.expected);}}finally{db.close();}
});

test("purpose scopes share one monotonically decreasing total without spending client quota on infrastructure",()=>{
 const db=new Database(":memory:");try{for(const row of fixture.purposes.cases){const remaining=(purpose:"semantic"|"infrastructure")=>fixture.purposes[purpose].map((v,i)=>v-row[purpose==="semantic"?"semanticSpent":"infrastructureSpent"][i]);const semantic=remaining("semantic"),infrastructure=remaining("infrastructure");const header=semantic.map((v,i)=>v+infrastructure[i]);const oracle=semantic.map((_,i)=>(db.query("SELECT ?-?+?-? AS total").get(fixture.purposes.semantic[i],row.semanticSpent[i],fixture.purposes.infrastructure[i],row.infrastructureSpent[i])as{total:number}).total);expect(header,row.id).toEqual(oracle);expect(header,row.id).toEqual(row.expectedHeader);expect(remaining(row.purpose==="semantic"?"semantic":"infrastructure"),row.id).toEqual(row.expectedScope);}}finally{db.close();}
});
