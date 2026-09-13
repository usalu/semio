import { GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

/** 🧭 Admits one encoded document execution-target asset route. */
export function localRelayExecutionTargetAsset(path: string): "manifest" | "component" | "descriptor" | "browser-actor" | undefined {
  const matched = /^\/spaces\/([^/]+)\/documents\/([^/]+)\/execution-target\/(manifest|component|descriptor|browser-actor)$/u.exec(path);
  if (!matched) return undefined;
  try {
    for (const encoded of [matched[1]!, matched[2]!]) {
      const id = decodeURIComponent(encoded);
      if (!id || id === "." || id === ".." || encodeURIComponent(id) !== encoded || /[\/\\\u0000-\u0020\u007f%?#]/u.test(id)) return undefined;
    }
    return matched[3] as "manifest" | "component" | "descriptor" | "browser-actor";
  } catch {
    return undefined;
  }
}

/** 🧭 Admits exact space artifact-creation collection, receipt, and cancellation routes. */
export function localRelaySpaceArtifactCreationPath(method: string, path: string): boolean {
  const matched = /^\/spaces\/([^/]+)\/artifact-creations(?:\/([0-9a-f]{32})(\/cancel)?)?$/u.exec(path);
  if (!matched) return false;
  try {
    const spaceId = decodeURIComponent(matched[1]!);
    if (!/^[A-Za-z0-9][A-Za-z0-9._:-]{0,255}$/u.test(spaceId) || encodeURIComponent(spaceId) !== matched[1]) return false;
  } catch {
    return false;
  }
  const requestId = matched[2];
  if (requestId === "0".repeat(32)) return false;
  const cancel = matched[3] !== undefined;
  return (method === "POST" && requestId === undefined && !cancel) || (method === "GET" && !cancel) || (method === "POST" && requestId !== undefined && cancel);
}

/** 🧭 Admits exact GIS-map inference lifecycle routes within the schema-owned cursor bound. */
export function localRelayInferencePath(method: string, path: string): boolean {
  const matched = /^\/spaces\/([^/]+)\/documents\/([^/]+)\/inference\/gis-map\/(.+)$/u.exec(path);
  if (!matched) return false;
  try {
    for (const encoded of [matched[1]!, matched[2]!]) {
      const id = decodeURIComponent(encoded);
      if (!/^[A-Za-z0-9._:-]{1,96}$/u.test(id) || id === "." || id === ".." || encodeURIComponent(id) !== encoded) return false;
    }
  } catch {
    return false;
  }
  const suffix = matched[3]!;
  if (method === "POST") return /^(?:jobs|jobs\/reconcile|jobs\/[0-9a-f]{32}\/(?:cancel|approval)|approval-undos)$/u.test(suffix);
  const events = /^jobs\/[0-9a-f]{32}\/events\?after=(0|[1-9][0-9]*)$/u.exec(suffix);
  return method === "GET" && events !== null && Number(events[1]) <= GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR;
}

/** 🧭 Projects an admitted local-relay URL to its bounded Hub upstream path. */
export function localRelayUpstreamPath(method: string, url: URL): string | undefined {
  if (!url.pathname.startsWith("/_semio/hub/")) return undefined;
  const upstream = url.pathname.slice("/_semio/hub".length);
  const noQuery = url.search === "";
  if (method === "GET" && upstream === "/auth/sessions/me" && noQuery) return upstream;
  if (method === "GET" && (upstream === "/directory/spaces" || /^\/directory\/spaces\/[^/]+$/u.test(upstream)) && noQuery) return upstream;
  if (method === "GET" && upstream === "/directory/events" && [...url.searchParams].length === 1 && /^\d+$/u.test(url.searchParams.get("since") ?? "")) return `${upstream}?since=${url.searchParams.get("since")}`;
  if (method === "POST" && (upstream === "/directory/commands" || upstream === "/directory/socket-grants") && noQuery) return upstream;
  if (method === "POST" && /^\/directory\/spaces\/[^/]+\/documents\/[^/]+\/socket-grants$/u.test(upstream) && noQuery) return upstream;
  if (method === "POST" && /^\/spaces\/[^/]+\/documents\/[^/]+\/open-plan$/u.test(upstream) && noQuery) return upstream;
  if (method === "POST" && /^\/spaces\/[^/]+\/documents\/[^/]+\/socket-grants$/u.test(upstream) && noQuery) return upstream;
  if (method === "POST" && localRelayExecutionTargetAsset(upstream) && noQuery) return upstream;
  if (noQuery && localRelaySpaceArtifactCreationPath(method, upstream)) return upstream;
  if (localRelayInferencePath(method, upstream + url.search)) return upstream + url.search;
  return undefined;
}
