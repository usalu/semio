import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { useActiveUtilityState } from "../../👇️hover-and-placement/🔶️active-utility/🟦️.ts";
import fixture from "../🧫️fixtures/hover-publication-order.json";

afterEach(cleanup);
for (const row of fixture) it(row.id, async () => {
  const events: unknown[][] = [];
  const requests: (() => void)[] = [];
  const instances = [
    { id:"shape@mesh#0", meshId:"shape", interactionId:"shape@mesh#0" },
    { id:"other@mesh#0", meshId:"other", interactionId:"other@mesh#0" },
  ];
  const core = {
    activeUtility:"select", brushMode:false, interactionGranularity:"object", interactionDomainId:"components", windowInstanceId:"world:preview",
    selection:{}, selectionMode:"object", vortices:[],
    geometryRevisionRef:{ current:{revision:1} },
    hostRef:{ current:null }, instances, instancesRef:{ current:instances }, meshesRef:{ current:[] },
    vorticesRef:{current:[]}, hoveredInstanceInteractionTargetRef:{current:null},
    dispatchSettled:vi.fn((action:string, args:{domainId:string; targets:string})=>{
      expect(action).toBe("interactionHover");
      const target = JSON.parse(args.targets)[0];
      events.push([args.domainId,target?.granularity ?? null,target?.id ?? null]);
      return new Promise<void>(done=>requests.push(done));
    }),
  };
  const hook = renderHook(({next})=>useActiveUtilityState({ core:next as unknown as Parameters<typeof useActiveUtilityState>[0]["core"] }), {initialProps:{ next:core }});
  for (const [step, id] of row.steps) {
    await act(async () => {
      switch (step) {
        case "face": hook.result.current.dispatchComponentHover({ objectId:"shape@mesh#0", mode:"face", id:Number(id) }); break;
        case "instance": hook.result.current.handleInstancePointerMove("shape@mesh#0"); break;
        case "other": hook.result.current.handleInstancePointerMove("other@mesh#0"); break;
        case "leave": hook.result.current.releaseLocalHover(); break;
        case "newGeometry": hook.rerender({ next:{...core,geometryRevisionRef:{current:{revision:2}} } }); break;
        case "newDomain": hook.rerender({ next:{...core,interactionDomainId:"graph"} }); break;
        case "unmount": hook.unmount(); break;
        case "settle": requests.shift()?.(); break;
      }
    });
  }
  expect(events).toEqual(row.events);
});
