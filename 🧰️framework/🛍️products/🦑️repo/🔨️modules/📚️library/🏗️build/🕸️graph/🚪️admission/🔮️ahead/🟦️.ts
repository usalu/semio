/** 🔮️ Starts the discovery of the owners that wait behind the one the admission closure works on, so several children discover at once while the closure still consumes the owners in its own order: the answer of an owner is the same promise whether the closure asks for it before, while or after the flight runs, an owner whose request changed meanwhile (its targets or inputs grew) is simply another flight, and `follow` names the requests an early answer makes known (the targets of an owner are only known once its bare facts are), so the closure result is exactly the one of a sequential walk.
 * @see ../🟦️.ts `closeBuildOwnerAdmissionV1` */
export function createDiscoveryAheadV1<Request,Facts>(input:{readonly key:(request:Request)=>string;readonly start:(request:Request)=>Promise<Facts>;readonly width:()=>number}){
  const flights=new Map<string,Promise<Facts>>();let running=0;
  const launch=(request:Request,key:string):Promise<Facts>=>{running++;const flight=input.start(request),done=()=>{running--;};flights.set(key,flight);flight.then(done,done);return flight;};
  const ahead=(requests:readonly Request[],follow?:(request:Request,facts:Facts)=>readonly Request[]):number=>{
    let started=0;
    for(const request of requests){
      if(running>=input.width())break;
      const key=input.key(request);
      if(flights.has(key))continue;
      launch(request,key).then(facts=>{if(follow!==undefined)ahead(follow(request,facts),follow);},()=>undefined);started++;
    }
    return started;
  };
  return Object.freeze({
    take(request:Request):Promise<Facts>{const key=input.key(request);return flights.get(key)??launch(request,key);},
    ahead,
    get running():number{return running;},
  });
}
