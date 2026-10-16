import Ajv from"ajv";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{ticketImportantProseReferenceAuthority,typescriptLeadingDocumentationReferenceAuthority}from"../../🟦️.ts";
import corpus from"./🔣️.json"with{type:"json"};
test("neutral checked normalization captures retain actual documentation and truncated EOF behavior",()=>{const db=new Database(":memory:");try{db.exec("create table observations(id text primary key,count integer)");for(const row of corpus.cases){expect(new Ajv({strict:true}).compile(schema)(row)).toBe(true);const actual=row.operation==="ticket"?ticketImportantProseReferenceAuthority(row.source):typescriptLeadingDocumentationReferenceAuthority(row.source);expect(actual.map(value=>value.value)).toEqual(row.expected);for(const value of actual)expect(row.source.slice(value.start,value.end)).toBe(value.value);db.query("insert into observations values(?,?)").run(row.id,actual.length);if(row.operation==="documentation"&&row.expected.length){const file=ts.createSourceFile("fixture.ts",row.source,ts.ScriptTarget.Latest,true),ranges=ts.getLeadingCommentRanges(file.text,0)??[],oracle=ranges.flatMap(range=>[...file.text.slice(range.pos,range.end).matchAll(/`([^`]+)`/gu)].map(value=>value[1]));expect(oracle).toEqual(row.expected);}}expect(db.query("select sum(count) total from observations").get()).toEqual({total:2});}finally{db.close();}});
