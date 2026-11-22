/** 🧾️ Contract: a dependency store that holds a name which is no generation or capsule never fails `repo tools setup all` (retiring obsolete generations is best effort): the refusal is recognised, also inside an AggregateError, and nothing else is.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {rosterRefusalV1} from "../../🟦️.ts";

test("only the roster refusal is recognised, plain or inside an AggregateError",()=>{
 const roster=Error('Exact known current dependency generation roster required: [".DS_Store"] is not a generation or capsule name');
 expect(rosterRefusalV1(roster)).toBe(true);
 expect(rosterRefusalV1(Error("Exact known current dependency generation roster required"))).toBe(true);
 expect(rosterRefusalV1(new AggregateError([Error("other"),roster]))).toBe(true);
 for(const other of [Error("Closed current dependency plan request required"),new AggregateError([Error("x")]),"text",null,undefined])expect(rosterRefusalV1(other)).toBe(false);
});
