import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{fileURLToPath}from"node:url";
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};
import{createRowPackerV1,packRowListsV1,packedRowsPolicyV1,sharedPrefixV1,unpackRowListsV1,type PackedRowV1,type PackedRowsV1}from"../🟦️.ts";

/** 🗜️ The packed row form stores every distinct row once, writes each list as numbers and shortens paths and long repeated strings; it is the same bytes for the same lists on every machine and in every language. */
type Case={name:string;lists:PackedRowV1[][];extras:unknown[];packed:PackedRowsV1};
const cases=fixture.cases as unknown as Case[];
const plain=(value:unknown)=>JSON.parse(JSON.stringify(value));

test("every case of the language-agnostic fixture packs to its stored form and unpacks back to its lists",()=>{
 for(const row of cases){
  expect(plain(packRowListsV1(row.lists,row.extras)),row.name).toEqual(row.packed);
  const back=unpackRowListsV1(row.packed);
  expect(plain(back.lists),row.name).toEqual(plain(row.lists));
  expect(plain(back.extras),row.name).toEqual(plain(row.extras));
 }
});

test("the Python implementation packs and unpacks the same fixture to the same forms",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️oracle.py"),here("./🧫️fixtures/🔣️.json")],{encoding:"utf8",maxBuffer:2**28});
 expect(run.status,run.stderr).toBe(0);
 const oracle=JSON.parse(run.stdout) as {name:string;packed:PackedRowsV1;unpacked:{lists:unknown;extras:unknown}}[];
 expect(oracle.map(row=>row.name)).toEqual(cases.map(row=>row.name));
 expect(oracle.map(row=>row.packed)).toEqual(cases.map(row=>plain(packRowListsV1(row.lists,row.extras))));
 expect(oracle.map(row=>row.unpacked)).toEqual(cases.map(row=>plain(unpackRowListsV1(row.packed))));
});

test("a row that several lists name is one object after unpacking, and rows that differ only in their fact stay apart",()=>{
 const row=cases.find(item=>item.name==="runs-and-repeats")!,back=unpackRowListsV1(row.packed);
 expect(back.lists[0]![0]).toBe(back.lists[1]![0]);expect(back.lists[0]![2]).toBe(back.lists[2]![0]);
 const apart=unpackRowListsV1(cases.find(item=>item.name==="same-row-different-fact")!.packed).lists[0]!;
 expect(new Set(apart).size).toBe(4);expect("fact" in apart[2]!).toBe(false);expect(apart[3]!.fact).toBeNull();
});

test("a prefix never ends inside a surrogate pair and counts code units",()=>{
 expect(sharedPrefixV1("/🧰x","/🧬x")).toBe(1);expect(sharedPrefixV1("/🧬x","/🧬y")).toBe(3);expect(sharedPrefixV1("","a")).toBe(0);expect(sharedPrefixV1("abc","abc")).toBe(3);
});

test("the form is a pure function of the lists: equal lists give equal bytes, whatever objects hold them",()=>{
 const make=()=>cases.find(item=>item.name==="pooled-strings")!;
 const a=JSON.stringify(packRowListsV1(structuredClone(make().lists),structuredClone(make().extras))),b=JSON.stringify(packRowListsV1(make().lists,make().extras));
 expect(a).toBe(b);
});

test("a form that does not hold together is refused",()=>{
 const good=cases.find(item=>item.name==="runs-and-repeats")!.packed;
 const bad=(change:(copy:any)=>void)=>{const copy=structuredClone(good) as any;change(copy);return()=>unpackRowListsV1(copy);};
 expect(bad(copy=>{copy.table[0][1]=5;})).toThrow(/prefix/u);
 expect(bad(copy=>{copy.lists[0].push(99);})).toThrow(/row number/u);
 expect(bad(copy=>{copy.lists[0][0]="x";})).toThrow(/number/u);
 expect(bad(copy=>{copy.table[1]=["x"];})).toThrow(/table row/u);
 expect(bad(copy=>{copy.pool=["a"];copy.table[0][3]={[packedRowsPolicyV1.poolKey]:7};})).toThrow(/pool index/u);
 expect(()=>unpackRowListsV1({} as never)).toThrow(/shape/u);
});

test("law: the packed size grows with the distinct rows, not with the number of lists that repeat them",()=>{
 const base:PackedRowV1[]=Array.from({length:400},(_,at)=>({kind:"lstat",path:`⟦semio-root⟧/🧰️framework/dir${Math.floor(at/10)}/file${at}.ts`,fact:"file"}));
 const layers=Array.from({length:1500},(_,at)=>[...base,{kind:"exists",path:`⟦semio-root⟧/probe${at}/package.json`,fact:false}]);
 const expanded=JSON.stringify(layers).length,packer=createRowPackerV1();
 for(const layer of layers)packer.add(layer);
 const packed=packer.finish(),size=JSON.stringify(packed).length;
 expect(packer.size()).toEqual({rows:400+1500,references:1500*401});
 expect(packed.table.length).toBe(1900);
 expect(size,`packed ${size} of ${expanded}`).toBeLessThan(expanded/25);
 expect(plain(unpackRowListsV1(packed).lists)).toEqual(plain(layers));
});

test("law: pseudo-random lists survive a round trip with their order",()=>{
 let seed=7;const next=(limit:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed%limit;};
 const kinds=["lstat","exists","file","field","resolve"],alphabet=["a","é","🧬","/","b","🧰"];
 const text=()=>Array.from({length:1+next(8)},()=>alphabet[next(alphabet.length)]).join("");
 const rows:PackedRowV1[]=Array.from({length:120},()=>{const kind=kinds[next(5)]!,path="/"+text(),pick=next(4);return pick===0?{kind,path}:pick===1?{kind,path,fact:text().repeat(20)}:pick===2?{kind,path,fact:{a:[next(9),text()],b:next(2)===0}}:{kind,path,fact:next(2)===0};});
 const lists=Array.from({length:30},()=>Array.from({length:next(40)},()=>rows[next(rows.length)]!));
 const extras=[text().repeat(30),[text().repeat(30),text().repeat(30)]];
 const packed=packRowListsV1(lists,extras),back=unpackRowListsV1(JSON.parse(JSON.stringify(packed)));
 expect(plain(back.lists)).toEqual(plain(lists));expect(plain(back.extras)).toEqual(plain(extras));
});
