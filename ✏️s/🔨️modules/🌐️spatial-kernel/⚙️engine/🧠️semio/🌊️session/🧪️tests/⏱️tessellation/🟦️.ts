import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {BoxGeometry} from "three";
import {applyPatch} from "fast-json-patch";
import law from "../🧫️fixtures/⏱️tessellation/🎟️owners.json";
import schema from "../../🧬️schema/⏱️tessellation/🎟️owners.json";

test("original analytic session source has independent closed volume and strict zero-effect denial authority",()=>{
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,denied:{...law.denied,maximumCapacityBytes:1}})).toBe(false);expect(validate({...law,normalDemandFields:[]})).toBe(false);const source=law.source,mesh=new BoxGeometry(source.width,source.depth,source.height),positions=mesh.getAttribute("position"),indices=mesh.index!;let volume=0;
  for(let index=0;index<indices.count;index+=3){const points=[0,1,2].map(offset=>{const vertex=indices.getX(index+offset);return[positions.getX(vertex),positions.getY(vertex),positions.getZ(vertex)];});const[a,b,c]=points;volume+=(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6;}
  expect(volume).toBe(source.volume);expect(indices.count).toBe(36);mesh.dispose();
});

test("original admission source text agrees with independent UTF8 bytes and JSON Patch events",()=>{
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);
  expect(law.inlineSourceReceiptItems).toBe(1);expect(law.admission.source).toBe("samePointerLengthAndTolerance");
  const rows=[{geometry:"solid-17",toleranceBits:"0x3fa999999999999a",operation:17,generation:5},{geometry:"solid-17",toleranceBits:"0x3fa999999999999a",...law.admission.peerIdentity}];
  const independent=new Map(rows.map(row=>[JSON.stringify(law.admission.registryKey.map(field=>row[field as keyof typeof row])),row]));expect(independent.size).toBe(2);
  const expectedPeer=rows[1],retained={rows};expect(applyPatch(retained,[{op:"remove",path:"/rows/0"}],true).newDocument.rows).toEqual([expectedPeer]);expect(retained.rows[0].operation).toBe(law.admission.peerIdentity.operation);
  for(const row of law.admission.texts){
    const bytes=Buffer.from(row.source,"utf8");expect(bytes.byteLength).toBe(row.utf8Bytes);
    for(const credit of row.credits){
      let offset=0;const target={value:""};
      while(offset<bytes.length){let end=Math.min(offset+credit,bytes.length);while(end>offset&&(bytes[end]&0xc0)===0x80)end--;expect(end).toBeGreaterThan(offset);const chunk=bytes.subarray(offset,end).toString("utf8");expect(applyPatch(target,[{op:"replace",path:"/value",value:target.value+chunk}],true).newDocument).toBe(target);expect(Buffer.byteLength(chunk,"utf8")).toBeLessThanOrEqual(credit);offset=end;}
      expect(target.value).toBe(row.source);expect(Buffer.from(target.value,"utf8")).toEqual(bytes);
    }
  }
});


test("original admitted geometry failure agrees with independent missing-source lookup and whole receipt event",()=>{
  expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
  const db=new Database(":memory:");db.exec("CREATE TABLE original_geometry(handle TEXT PRIMARY KEY); INSERT INTO original_geometry VALUES ('solid-17');");
  const source=Buffer.from(law.failure.missingHandle),moved=Buffer.from(source);expect(source.equals(moved)).toBe(true);expect(source).not.toBe(moved);
  const row=db.query("SELECT handle FROM original_geometry WHERE handle = ?").get(source.toString());expect(row).toBeNull();
  const original={primary:{kind:law.failure.kind,message:law.failure.message},work:{retained:{items:0,copy:0,capacity:0,release:0},consumedUnits:law.failure.beforeAdmissionConsumedUnits}};
  const result=applyPatch(original,[{op:"replace",path:"/work/consumedUnits",value:law.failure.admittedConsumedUnits}],true).newDocument;
  expect(result).toBe(original);expect(result.primary).toEqual({kind:"invariantViolated",message:"original tessellation source handle is not live"});expect(result.work).toEqual({retained:{items:0,copy:0,capacity:0,release:0},consumedUnits:1});
  expect(law.failure.boundedTextScalarsPerEvent).toBe(1);expect(law.failure.cancelEventsPerTurn).toBe(1);db.close();
});
