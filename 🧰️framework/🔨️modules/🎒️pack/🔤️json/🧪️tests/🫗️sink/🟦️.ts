import Ajv from"ajv";import fixture from"./🔣️.json";import schema from"./🧬️.json";
import{expect,test}from"bun:test";import{resolve}from"node:path";
test("shared borrowed JSON traversal admits retained fixed-sink cursor",async()=>{const p=resolve(import.meta.dir,"../../🎛️controlled-and-borrowed-bridge/🫗️sink/🦀️.rs");const file=Bun.file(p);expect(await file.exists()).toBe(true);if(!await file.exists())return;const source=await file.text();for(const port of["JsonBorrowedSinkCursor","RetainedWorkOutcome","advance_source","ControlledRetirement","maximum_work","written_bytes"])expect(source).toContain(port);for(const bypass of["NativeEncodeControl::new(","String::new(","to_value("])expect(source).not.toContain(bypass);});

test("fixed sink corpus agrees with independent JSON",()=>{const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture)).toBe(true);for(const row of fixture.cases)expect(JSON.stringify(row.input)).toBe(row.expected);});
