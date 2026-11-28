/** 🛰️ The fleet relay: membership vectors agree with Python's `ipaddress`, a loopback service answers through the relay byte for byte, a peer outside the network is refused, and close ends it. */
import {spawnSync} from "node:child_process";
import {createServer} from "node:http";
import {describe,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import vectors from "./🔣️.json" with {type:"json"};
import {exposeServiceToFleetV1,insideNetworkV1} from "../🟦️.ts";

const oracle=`import ipaddress,json,sys
def inside(a,c):
    try:
        net=ipaddress.ip_network(c,strict=False);ip=ipaddress.ip_address(a)
        ip=ip.ipv4_mapped or ip if ip.version==6 else ip
        return ip.version==4 and ip in net
    except ValueError: return False
print(json.dumps([inside(r["address"],r["cidr"]) for r in json.load(sys.stdin)["inside"]]))`;

describe("fleet service relay",()=>{
 test("membership agrees with Python ipaddress",()=>{
  const python=JSON.parse(spawnSync(testPythonV1()??"python3",["-I","-c",oracle],{input:JSON.stringify(vectors),encoding:"utf8"}).stdout);
  vectors.inside.forEach((row,index)=>{
   expect(insideNetworkV1(row.address,row.cidr),JSON.stringify(row)).toBe(row.inside);
   expect(python[index],JSON.stringify(row)+" python").toBe(row.inside);
  });
 });
 test("relays a loopback service to fleet peers and refuses others",async()=>{
  const service=createServer((request,response)=>{response.end(`ok ${request.url}`);});
  await new Promise<void>(resolve=>service.listen(0,"127.0.0.1",()=>resolve()));
  const local=`http://127.0.0.1:${(service.address() as {port:number}).port}`;
  try{
   expect(await exposeServiceToFleetV1(local,{})).toBeUndefined();
   const relay=(await exposeServiceToFleetV1(local,{SEMIO_FLEET_NETWORK:"127.0.0.0/8"},"127.0.0.1"))!;
   expect(relay.origin).toMatch(/^http:\/\/127\.0\.0\.1:\d+$/u);
   expect(await (await fetch(relay.origin+"/forms?x=1")).text()).toBe("ok /forms?x=1");
   await relay.close();
   await expect(fetch(relay.origin+"/",{signal:AbortSignal.timeout(2000)})).rejects.toThrow();
   const closed=(await exposeServiceToFleetV1(local,{SEMIO_FLEET_NETWORK:"10.0.0.0/8"},"127.0.0.1"))!;
   await expect(fetch(closed.origin+"/",{signal:AbortSignal.timeout(2000)})).rejects.toThrow();
   expect(closed.refused()).toBe(1);
   await closed.close();
  }finally{service.close();}
 });
});
