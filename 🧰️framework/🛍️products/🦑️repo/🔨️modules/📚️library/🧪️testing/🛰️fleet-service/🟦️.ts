import {createConnection,createServer,type Socket} from "node:net";
import {networkInterfaces} from "node:os";

/** 🌐️ The variable that names the fleet network (`192.168.178.0/24`) and the one a test file reads its service origin from; the second is forwarded to the machine that runs the file.
 * @see ../../🏗️build/🌐️remote-execution/🚀️offload/🔬️checkout-test/🧬️schema/🔣️.json */
export const FLEET_SERVICE_ENVIRONMENT_V1=Object.freeze({network:"SEMIO_FLEET_NETWORK",origin:"SEMIO_TEST_SERVICE_ORIGIN"});

/** 🧮️ Whether an IPv4 address (also as `::ffff:a.b.c.d`) lies inside a CIDR block. */
export function insideNetworkV1(address:string,cidr:string):boolean{
 const ip=(text:string):number|undefined=>{const parts=text.replace(/^::ffff:/u,"").split(".");if(parts.length!==4||parts.some(part=>!/^\d{1,3}$/u.test(part)||Number(part)>255))return undefined;return parts.reduce((sum,part)=>sum*256+Number(part),0);};
 const [base,bits]=cidr.split("/"),prefix=Number(bits),a=ip(address),b=ip(base??"");
 if(a===undefined||b===undefined||!Number.isInteger(prefix)||prefix<0||prefix>32)return false;
 const size=2**(32-prefix);
 return Math.floor(a/size)===Math.floor(b/size);
}

/** 🔎️ This machine's IPv4 address inside the fleet network, if it has one. */
export function fleetAddressV1(cidr:string):string|undefined{
 return Object.values(networkInterfaces()).flat().find(row=>row!==undefined&&row.family==="IPv4"&&!row.internal&&insideNetworkV1(row.address,cidr))?.address;
}

/** 🛰️ Makes a service that listens on loopback reachable from the other machines of the fleet for as long as a run needs it: a TCP relay on this machine's address inside the fleet network (or `bind`), on a free port, that refuses every peer outside the network and forwards the rest byte for byte to the loopback origin. `close` ends the relay and every relayed connection; the service itself stays private to this machine otherwise. Returns `undefined` when no fleet network is named or this machine has no address in it.
 * @see insideNetworkV1 */
export async function exposeServiceToFleetV1(origin:string,environment:Readonly<Record<string,string|undefined>>,bind?:string):Promise<Readonly<{origin:string;refused:()=>number;close:()=>Promise<void>}>|undefined>{
 const cidr=environment[FLEET_SERVICE_ENVIRONMENT_V1.network];
 if(cidr===undefined||cidr==="")return undefined;
 const address=bind??fleetAddressV1(cidr),target=new URL(origin);
 if(address===undefined)return undefined;
 const open=new Set<Socket>();let refused=0;
 const server=createServer(peer=>{
  if(!insideNetworkV1(peer.remoteAddress??"",cidr)){refused++;peer.destroy();return;}
  const upstream=createConnection({host:target.hostname,port:Number(target.port||80)});
  for(const socket of [peer,upstream]){open.add(socket);socket.on("close",()=>open.delete(socket));socket.on("error",()=>{peer.destroy();upstream.destroy();});}
  peer.pipe(upstream);upstream.pipe(peer);
 });
 await new Promise<void>((resolve,reject)=>{server.once("error",reject);server.listen(0,address,()=>resolve());});
 const port=(server.address() as {port:number}).port;
 return Object.freeze({
  origin:`${target.protocol}//${address}:${port}`,
  refused:()=>refused,
  close:()=>new Promise<void>(resolve=>{for(const socket of open)socket.destroy();server.close(()=>resolve());}),
 });
}
