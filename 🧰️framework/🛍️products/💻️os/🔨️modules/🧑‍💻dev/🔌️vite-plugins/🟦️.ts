/** 🔌️ The static serve guards among the dev server's own Vite plugins: the descriptor route guard, the service worker scope and the local hub session endpoint. The backbone, blob, activation, source freshness, agent bridge and credential plugins live in the sibling folders, kept apart from `📜️script.ts`'s task router so `⚙️vite.config.ts` can mount them without pulling the repository library's discovery walk into Vite's config bundle. */
import { VITE_RUNTIME_CHANNELS_ENV_V1, readViteRuntimeChannelsV1 } from "@semio-tech/repo-lib/dev";
import { existsSync } from "node:fs";
import { join } from "node:path";
import {readDevLocalHubProxyV1,requestDevLocalHubProxyV1,devLocalHubSessionPolicyV1} from "../🚀️local-hub/🎫️session/📋️data/🟦️.ts";
import {parseLocalSessionV1} from "@semio-tech/framework-os/modules/directory/local-session/broker/schema";

import { LOCAL_HUB_SESSION_ENDPOINT_V1, localHubSessionAnswerV1 } from "@semio-tech/framework/directory/local-session";
import type { BackboneServerRequest, BackboneServerResponse } from "./🧱️backbone-vite-plugin/🟦️.ts";

export type DescriptorRouteGuardSpec = {
  readonly route: string;
  readonly root: string;
  readonly directoryNames: ReadonlySet<string>;
};

export type DescriptorRouteDecision = { readonly kind: "pass" } | { readonly kind: "missing"; readonly moduleDirectory: string };

/** 🛂️ Resolves only canonical declared module descriptor requests, without decoding arbitrary filesystem paths. */
export function descriptorRouteDecision(url: string | undefined, specs: readonly DescriptorRouteGuardSpec[]): DescriptorRouteDecision {
  if (!url) return { kind: "pass" };
  let pathname: string;
  try {
    pathname = decodeURIComponent(new URL(url, "http://127.0.0.1").pathname);
  } catch {
    return { kind: "pass" };
  }
  for (const spec of specs) {
    const route = spec.route.endsWith("/") ? spec.route : `${spec.route}/`;
    if (!pathname.startsWith(route)) continue;
    const parts = pathname.slice(route.length).split("/");
    if (parts.length !== 2 || parts[1] !== "🔣️.json") return { kind: "pass" };
    const moduleDirectory = parts[0] ?? "";
    if (!spec.directoryNames.has(moduleDirectory) || !existsSync(join(spec.root, moduleDirectory, "🔣️.json"))) return { kind: "missing", moduleDirectory };
    return { kind: "pass" };
  }
  return { kind: "pass" };
}

/** 🚫️ Prevents a missing plugin descriptor from falling through to Vite's HTML SPA response. */
export function semioDescriptorRouteGuardVitePlugin(specs: readonly DescriptorRouteGuardSpec[]) {
  return {
    name: "semio-descriptor-route-guard",
    enforce: "pre" as const,
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        const decision = descriptorRouteDecision(req.url, specs);
        if (decision.kind === "pass") return next();
        res.statusCode = 404;
        res.setHeader("content-type", "application/json");
        res.end(`${JSON.stringify({ error: "descriptor-not-found", moduleDirectory: decision.moduleDirectory })}\n`);
      });
    },
  };
}

/** 👷️ Lets a service worker served from the module graph (the plugin module store's, `🌎️hub-source/👷️service-worker`)
 * control the whole shell: every script a browser fetches AS a service worker (`Service-Worker: script`) is allowed scope
 * `/`. A deployment answers its service worker script with the same header. */
export function semioServiceWorkerScopeVitePlugin() {
  return {
    name: "semio-service-worker-scope",
    enforce: "pre" as const,
    configureServer(server: { middlewares: { use: (handler: (req: { headers?: Record<string, string | string[] | undefined> }, res: { setHeader: (name: string, value: string) => void }, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (req.headers?.["service-worker"] === "script") res.setHeader("Service-Worker-Allowed", "/");
        next();
      });
    },
  };
}

/** 🎫️ Serves the shell a FRESH development session from the local hub owner's broker on every request, for the
 * profile this serve signs in as — so a shell whose 15-minute local session lapsed, or a second user's serve, claims one
 * with no manual sign-in. Always `200` with a `LocalHubSessionAnswerV1`: the typed "not offered" when the serve joined a
 * hub without a broker (the shell's own sign-in stays available).
 * @see ../🚀️local-hub/🏃️execution/🟦️.ts */
export function semioLocalHubSessionVitePlugin() {
  const wire=process.env[VITE_RUNTIME_CHANNELS_ENV_V1],channels=readViteRuntimeChannelsV1(wire?JSON.parse(wire):[]),channel=channels.find(row=>row.name===devLocalHubSessionPolicyV1().runtimeChannelName),proxy=channel?readDevLocalHubProxyV1({version:1,endpoint:channel.endpoint,secret:channel.secret}):undefined;
  return {name:"semio-local-hub-session",configureServer(server:{middlewares:{use:(handler:(req:{readonly url?:string;readonly method?:string},res:{statusCode:number;setHeader:(key:string,value:string)=>void;end:(body?:string)=>void},next:()=>void)=>void)=>void}}){
    server.middlewares.use((req,res,next)=>{if(req.method!=="GET"||req.url?.split("?")[0]!==LOCAL_HUB_SESSION_ENDPOINT_V1)return next();const signal=AbortSignal.timeout(devLocalHubSessionPolicyV1().maximumRequestMilliseconds),work=proxy?requestDevLocalHubProxyV1(proxy,signal):Promise.resolve(null);void work.then(value=>{const session=value===null?null:parseLocalSessionV1(value);res.setHeader("cache-control","no-store");res.statusCode=200;res.setHeader("content-type","application/json");res.end(JSON.stringify(localHubSessionAnswerV1(session===null?null:{token:session.token,userId:session.userId})));}).catch(()=>{res.statusCode=503;res.end();});});
  }};
}
