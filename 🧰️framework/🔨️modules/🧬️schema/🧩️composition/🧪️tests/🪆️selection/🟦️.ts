/** 🧪️ Shared selection witnesses compare the first-party contract with independent SQLite and GraphQL. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {parse} from "graphql";
import {childSlotAcceptsArtifactKind,GRAPHQL_COMPOSITION_PREAMBLE,type ChildSlotSelection} from "../../../🟦️.ts";
import corpus from "../🧫️fixtures/🪆️selection/🔣️.json";

test("explicit child selection matches independent SQL without deciding a capability's serialization",()=>{
 const oracle=new Database(":memory:");
 try{const query=oracle.query("SELECT CASE ?1 WHEN 'artifactKind' THEN ?2 = ?3 WHEN 'capability' THEN 1 ELSE 0 END AS accepted");for(const row of corpus.cases){const selection=row.selection as ChildSlotSelection;expect(childSlotAcceptsArtifactKind(selection,row.artifactKind),row.name).toBe(row.accepted);expect((query.get(selection.kind,selection.id,row.artifactKind)as{accepted:number}).accepted===1,row.name).toBe(row.accepted);}}finally{oracle.close();}
});

test("GraphQL declares both explicit selection arguments without an implicit concrete kind",()=>{
 const document=parse(GRAPHQL_COMPOSITION_PREAMBLE);
 const directive=document.definitions.find(definition=>definition.kind==="DirectiveDefinition"&&definition.name.value==="child");
 if(!directive||directive.kind!=="DirectiveDefinition")throw Error("Child directive missing");
 expect(directive.arguments?.map(argument=>[argument.name.value,argument.type.kind])).toEqual([["kind","NamedType"],["capability","NamedType"]]);
});
