import{test,expect}from"bun:test";
import{mkdtempSync,mkdirSync,writeFileSync,existsSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{removeEmptyNativeReceiverShellV1}from"../🟦️.ts";
const make=()=>mkdtempSync(join(tmpdir(),"receiver-shell-"));
test("empty receiver shell with or without empty mount directory is removed",()=>{const a=make(),b=make();mkdirSync(join(b,"m"));removeEmptyNativeReceiverShellV1(a,join(a,"m"));removeEmptyNativeReceiverShellV1(b,join(b,"m"));expect(existsSync(a)||existsSync(b)).toBe(false);});
test("receiver shell with leftover mount content is refused with path, entries and remedy and left intact",()=>{const a=make();mkdirSync(join(a,"m","vite","deps"),{recursive:true});expect(()=>removeEmptyNativeReceiverShellV1(a,join(a,"m"))).toThrow(new RegExp(a.replaceAll("/","\\/")+".*\"m\":\\[\"vite\"\\].*manually"));expect(existsSync(join(a,"m","vite","deps"))).toBe(true);rmSync(a,{recursive:true});});
test("receiver shell with stray sibling files is refused",()=>{const a=make();writeFileSync(join(a,"fixed.dmg.partial"),"x");expect(()=>removeEmptyNativeReceiverShellV1(a,join(a,"m"))).toThrow(/fixed\.dmg\.partial/);expect(existsSync(join(a,"fixed.dmg.partial"))).toBe(true);rmSync(a,{recursive:true});});
