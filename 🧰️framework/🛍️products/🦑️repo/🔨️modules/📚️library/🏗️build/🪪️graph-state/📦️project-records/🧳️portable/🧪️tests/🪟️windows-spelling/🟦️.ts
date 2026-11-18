/** 🪟️ Law: a record made under `C:\semio` and one made under `/Users/ueli/Documents/semio` from the same owner inputs are byte-identical in the neutral form. Paths inside the JSON list of a `resolve` row (every element, not only the first) and inside the fact object of a `field` row are workspace-relative with forward slashes; a text that still names the root with backslashes inside a list is refused (the record is not published) instead of being published raw.
 * @see ../../🟦️.ts `neutralRowsV1` */
import{expect,test}from"bun:test";
import{UnportableV1,hostOfRootV1,localizeRowsV1,neutralRowsV1}from"../../🟦️.ts";

const windows=hostOfRootV1("C:\\semio"),mac=hostOfRootV1("/Users/ueli/Documents/semio");
const under=(root:string,separator:string,...parts:string[])=>root+separator+parts.join(separator);
const rowsOf=(root:string,separator:string)=>[
 {kind:"resolve",path:JSON.stringify([under(root,separator,"a","b.ts"),"./c",under(root,separator,"d","e.ts"),{candidates:[under(root,separator,"f")]}]),fact:under(root,separator,"a","c.ts")},
 {kind:"field",path:under(root,separator,"x","🔣️.json"),fact:{format:"script-provider-designation",pointer:under(root,separator,"x","schema.json"),paths:[under(root,separator,"p","q.json")],digest:"abc"}},
] as never[];

test("the neutral rows of the Windows and the POSIX producer are byte-identical",()=>{
 const fromWindows=JSON.stringify(neutralRowsV1(rowsOf("C:\\semio","\\"),windows)),fromMac=JSON.stringify(neutralRowsV1(rowsOf("/Users/ueli/Documents/semio","/"),mac));
 expect(fromWindows).toBe(fromMac);
 expect(fromWindows).not.toContain("C:");expect(fromWindows).not.toContain("/Users/");expect(fromWindows).not.toContain("\\\\");
});

test("a Windows producer that spells the root with forward slashes gives the same bytes",()=>{
 expect(JSON.stringify(neutralRowsV1(rowsOf("C:/semio","/"),windows))).toBe(JSON.stringify(neutralRowsV1(rowsOf("/Users/ueli/Documents/semio","/"),mac)));
});

test("a taker on either host replays the neutral rows under its own root",()=>{
 const neutral=neutralRowsV1(rowsOf("C:\\semio","\\"),windows);
 const onMac=localizeRowsV1(neutral,mac) as never as {kind:string;path:string;fact:unknown}[];
 expect(JSON.parse(onMac[0]!.path)[0]).toBe("/Users/ueli/Documents/semio/a/b.ts");
 expect((onMac[1]!.fact as {pointer:string}).pointer).toBe("/Users/ueli/Documents/semio/x/schema.json");
 const onWindows=localizeRowsV1(neutral,windows) as never as {kind:string;path:string;fact:unknown}[];
 expect(JSON.parse(onWindows[0]!.path)[0]).toBe("C:\\semio\\a\\b.ts");
 expect((onWindows[1]!.fact as {pointer:string}).pointer).toBe("C:/semio/x/schema.json");
});

test("a text that names the root with backslashes inside a list element is refused, not published raw",()=>{
 const row={kind:"resolve",path:JSON.stringify(["C:\\semio\\a.ts","see C:\\semio\\b and more"]),fact:"C:\\semio\\a.ts"} as never;
 expect(()=>neutralRowsV1([row],windows)).toThrow(UnportableV1);
});
