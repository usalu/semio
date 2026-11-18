/** 🔖️ Contract: the recipe and the owner mark of a stored closure document are read from its first bytes, so naming the keys of the documents of an entry parses none of their rows: the answer equals the recipe of the fully parsed document (an independent `JSON.parse`), for recipes with braces, quotes, escapes and emoji inside strings, and a document that does not start the way the packer writes it is read by the full parse.
 * @see ../../🟦️.ts `closureDocumentHeadV1` */
import{expect,test}from"bun:test";
import{closureDocumentHeadV1,closureKeyV1,hostOfRootV1,packBundleV1,portableRowsPolicyV1}from"../../🟦️.ts";

const root="/Users/ueli/Documents/semio",host=hostOfRootV1(root),token=portableRowsPolicyV1.rootToken;
const recipes=[
 {fn:"native-target",projectRoot:"libs/a"},
 {fn:"x",text:'a "quoted" } brace { and \\ backslash \\" and 📦️ emoji',list:[1,[2,{"k}":"v{"}]],path:root+"/a"},
 {fn:"deep",a:{b:{c:{d:[{e:"}}}}"}]}}}},
];
const documentOf=(recipe:unknown,own:string)=>{
 const moved=packBundleV1({recipeKey:closureKeyV1(recipe),bundle:{recipe,rows:[],layers:[],value:{list:[own+"/p.ts"]},sidecar:{facts:[null],members:[null]}} as never},host,new Map());
 return Buffer.from(JSON.stringify(moved!.document));
};

test("the head of a packed document equals the recipe of its full parse, whatever the recipe holds",()=>{
 for(const recipe of recipes){
  const bytes=documentOf(recipe,root),full=JSON.parse(bytes.toString("utf8"));
  const head=closureDocumentHeadV1(bytes);
  expect(head?.recipe).toEqual(full.recipe);
  expect(head?.owned).toBe(false);
 }
});

test("the owner mark is found anywhere in the document and a view of a larger buffer is read without a copy",()=>{
 const bytes=documentOf(recipes[0]!,root),text=bytes.toString("utf8").replace('"list"','"'+portableRowsPolicyV1.ownerToken+'","list"'),owned=Buffer.from(text);
 expect(closureDocumentHeadV1(owned)?.owned).toBe(true);
 const padded=Buffer.concat([Buffer.from("xxxx"),bytes,Buffer.from("yyyy")]),view=new Uint8Array(padded.buffer,padded.byteOffset+4,bytes.length);
 expect(closureDocumentHeadV1(view)?.recipe).toEqual(recipes[0]);
});

test("a document that does not start as the packer writes it, one that is no document and one that is cut are read by the full parse or refused",()=>{
 const reordered=Buffer.from(JSON.stringify({rest:{},recipe:recipes[1],version:2,rows:{table:[],pool:[],lists:[],extras:[]}}));
 expect(closureDocumentHeadV1(reordered)?.recipe).toEqual(recipes[1]);
 expect(closureDocumentHeadV1(Buffer.from("not json"))).toBeUndefined();
 expect(closureDocumentHeadV1(Buffer.from('{"version":2,"recipe":{"a":'))).toBeUndefined();
 expect(closureDocumentHeadV1(Buffer.from('{"version":2,"recipe":3,"rest":{}}'))).toBeUndefined();
 expect(closureDocumentHeadV1(Buffer.from('{"version":1,"recipe":{"a":1}}'))).toBeUndefined();
 expect(token.length).toBeGreaterThan(0);
});
