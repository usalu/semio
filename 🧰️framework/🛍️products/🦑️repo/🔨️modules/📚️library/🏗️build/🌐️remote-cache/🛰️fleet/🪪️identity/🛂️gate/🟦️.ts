import {createHash,timingSafeEqual} from "node:crypto";
import {FLEET_TIME_HEADER_V1,fleetPeerAllowedV1,type FleetRoleV1,type FleetVerifierV1} from "../🟦️.ts";

/** 🪪️ Who a request was proven to come from: the origin and role that signed it and the content hash it declared, or `static` for an operator token used on the machine itself. */
export type FleetAccessV1=Readonly<{origin:string;group?:string;role:FleetRoleV1|"static";content:string}>;

/** 🏁️ The decision on one request: let it through (with `access`, absent only on an open loopback server) or the status to answer. */
export type FleetGateVerdictV1=Readonly<{ok:true;access:FleetAccessV1|undefined}>|Readonly<{ok:false;status:401|403|503;serverTime:number;readOnly?:true;sealed?:true}>;

/** ⚙️ What a gate is made of: the signature verifier, operator tokens that are honoured from the machine itself only, and two optional policies. `networks` is an additional policy of the topology (`access.enforceNetwork`): when given, a peer outside these CIDR networks is refused before anything is read; without it, who may talk to the server is decided by the verified fleet identity alone, never by an address. `home` names the group of the server: an origin of another group of the fleet holds only the `foreignRoles` (read the cache, submit work; never write, never work for it). A gate without verifier and without tokens is open. */
export type FleetGateOptionsV1=Readonly<{verifier?:FleetVerifierV1|undefined;networks?:readonly string[]|undefined;home?:string|undefined;foreignRoles?:readonly FleetRoleV1[];inGroup?:"sealed"|"trusted-lan";tokens?:Readonly<Record<string,string|undefined>>;now?:()=>number}>;

/** 🛂️ The door of a fleet server: first the network (a peer outside the fleet CIDR is refused before anything is read), then the credentials. */
export interface FleetGateV1{
 readonly open:boolean;
 network(peer:string|undefined):boolean;
 check(input:Readonly<{method:string;target:string;authorization:string|null|undefined;content:string|null|undefined;peer:string|undefined;roles:readonly FleetRoleV1[];tokens:readonly string[];refused?:readonly string[];sealed?:boolean}>):Promise<FleetGateVerdictV1>;
 respond(verdict:Extract<FleetGateVerdictV1,{ok:false}>):Response;
}

/** 🌍️ What an origin of another group of the fleet may do on a server: read the cache and submit work to the scheduler. */
export const FLEET_FOREIGN_ROLES_V1:readonly FleetRoleV1[]=Object.freeze(["cache-read","submit"] as const);

const digest=(value:string):Buffer=>createHash("sha256").update(value).digest();

/** 🛂️ Creates a gate. A bearer token never opens the door from another machine: static tokens are compared in constant time and accepted only from loopback, so they cannot be collected from the LAN; every other caller must present a signature whose role is one of the allowed ones. */
export function createFleetGateV1(options:FleetGateOptionsV1):FleetGateV1{
 const clock=options.now??Date.now,tokens=options.tokens??{},open=options.verifier===undefined&&Object.values(tokens).every(token=>token===undefined);
 return Object.freeze<FleetGateV1>({
  open,
  network:peer=>options.networks===undefined||fleetPeerAllowedV1(peer,options.networks),
  async check(input){
   if(open)return {ok:true,access:undefined};
   const bearer=input.authorization?.startsWith("Bearer ")?input.authorization.slice(7):undefined;
   if(bearer!==undefined&&bearer!==""){
    const local=fleetPeerAllowedV1(input.peer,[]);
    for(const name of input.tokens){const expected=tokens[name];if(expected!==undefined&&expected!==""&&(local||options.verifier===undefined)&&timingSafeEqual(digest(expected),digest(bearer)))return {ok:true,access:{origin:"static",role:"static",content:"-"}};}
    for(const name of input.refused??[]){const expected=tokens[name];if(expected!==undefined&&expected!==""&&timingSafeEqual(digest(expected),digest(bearer)))return {ok:false,status:403,serverTime:clock(),readOnly:true};}
    return {ok:false,status:401,serverTime:clock()};
   }
   if(options.verifier===undefined)return {ok:false,status:401,serverTime:clock()};
   const verdict=await options.verifier.verify({method:input.method,target:input.target,authorization:input.authorization,content:input.content});
   if(!verdict.ok)return {ok:false,status:verdict.status,serverTime:verdict.serverTime};
   if(!input.roles.includes(verdict.role))return {ok:false,status:403,serverTime:clock()};
   if(options.home!==undefined&&verdict.group!==options.home&&!(options.foreignRoles??FLEET_FOREIGN_ROLES_V1).includes(verdict.role))return {ok:false,status:403,serverTime:clock()};
   if(options.home!==undefined&&input.sealed!==true&&(verdict.group!==options.home||options.inGroup==='sealed'))return {ok:false,status:403,serverTime:clock(),sealed:true};
   return {ok:true,access:{origin:verdict.origin,group:verdict.group,role:verdict.role,content:verdict.content}};
  },
  respond:verdict=>new Response(verdict.status===403?(verdict.readOnly?"Forbidden: read-only token\n":verdict.sealed?"Forbidden: sealed transport required\n":"Forbidden\n"):verdict.status===503?"Busy\n":"Unauthorized\n",{status:verdict.status,headers:{"content-type":"text/plain",[FLEET_TIME_HEADER_V1]:String(verdict.serverTime),...(verdict.status===401?{"www-authenticate":options.verifier===undefined?"Bearer":"Semio-Fleet"}:{})}}),
 });
}
