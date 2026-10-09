import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
test("original shared actor text binding borrows source and refuses missing full authority",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(JSON.parse(JSON.stringify(fixture.source))).toBe(fixture.source);expect(Buffer.byteLength(fixture.source)).toBe(18);
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("impl DslField for semio_framework_value::SharedUtf8");expect(source).toContain("FieldProjectionView::Text(self.as_str())");expect(source).toContain("shared UTF8 construction requires original full retained authority");
});
