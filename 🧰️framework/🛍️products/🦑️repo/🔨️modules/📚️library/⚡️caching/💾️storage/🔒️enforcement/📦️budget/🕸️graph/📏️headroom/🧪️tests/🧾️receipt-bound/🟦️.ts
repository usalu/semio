/** 🧾️ Contract: the authority receipt bound of the headroom is the bound its streaming reader enforces (the closure `maximumFileBytes`), not a smaller one, and the receipt is authenticated in bounded memory: the digest reads in `chunkBytes` chunks and the identity check streams records in a child, with no whole-file read or parse. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import headroom from"../../🧬️schema/🔣️.json";
import closures from"../../🧾️closures/🧬️schema/🔣️.json";

const P=headroom.definitions.Protocol.const,source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");

test("the receipt bound equals the closure file bound and the snapshot bound",()=>{
 expect(P.maximumReceiptBytes).toBe(closures.definitions.Protocol.const.maximumFileBytes);
 expect(P.maximumReceiptBytes).toBe(P.maximumSnapshotBytes);
});

test("the receipt is never read or parsed as a whole",()=>{
 const program=source.slice(source.indexOf("const authorityProgramV1="),source.indexOf("`;",source.indexOf("const authorityProgramV1="))+2);
 expect(program).toContain("createAuthorityStreamV1");
 expect(program).not.toContain("readFileSync");
 expect(program).not.toMatch(/JSON\.parse\((?!process\.argv)/u);
 expect(source).toContain("Buffer.allocUnsafe(protocol.chunkBytes)");
});
