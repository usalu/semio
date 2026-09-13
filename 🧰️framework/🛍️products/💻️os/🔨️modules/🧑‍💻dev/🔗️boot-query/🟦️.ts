// #region 🧲️Header
/** @emoji 🔗️ Boot query contract for the OS dev entry — the `?plugin=`/`?role=` axes the two
 * renderers must agree on so ONE url opens the same surface on the React port and the wgpu port. */
// #endregion 🧲️Header

import type { AppRole } from "@semio-tech/framework";

/** @emoji 🔗️ `?plugin=` — the playground variant. The wgpu browser boot switches its whole boot plan on
 * it (`…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` `bootDescriptor`); the React dev server is already
 * built for exactly one variant (`virtual:semio-playground-session`), so the React entry carries the
 * name only to keep the two urls spelled identically. */
export const BOOT_QUERY_PLUGIN_PARAM = "plugin";

/** @emoji 🔗️ `?role=` — the boot-time surface role, same spelling and same `"viewer"`-or-else rule the
 * wgpu `bootDescriptor` applies, so `…/?plugin=generation3d&role=viewer` opens the viewer on 6018 and
 * 6118 alike. Pairs with the `SEMIO_APP_ROLE` → `VITE_SEMIO_APP_ROLE` process-env projection
 * (`🎮️playground/🔒️preferences/🟦️.ts` `SEMIO_APP_ROLE_ENV`): the query is the per-navigation axis, the
 * env is the per-server default. */
export const BOOT_QUERY_APP_ROLE_PARAM = "role";

/** @emoji 📚️ `?example=` — the boot-time example of the OPEN dialect, the third per-navigation axis
 * beside `?plugin=` and `?role=`. Both shells resolve the example list through the ONE dialect-keyed
 * predicate (`manifest::examples_for_app` / `examplesForApp`), so `…/?plugin=generation3d&example=<id>`
 * opens the same document on 6018 and 6118. An id the open dialect does not author is IGNORED — a url
 * is not a place to hard-fail a shell, the same rule `?role=` and `?mode=` already follow. Pairs with
 * the `VITE_SEMIO_DEFAULT_EXAMPLE` per-server seed: the query is the per-navigation axis, the env is
 * the per-server default. */
export const BOOT_QUERY_EXAMPLE_PARAM = "example";

/** @emoji 📏️ Bound on the raw query string before it is parsed — the wgpu boot's
 * `LOCATION_SEARCH_CAPACITY`, restated here so both entries refuse the same oversized url instead of
 * handing an unbounded string to `URLSearchParams`. */
export const BOOT_QUERY_CAPACITY = 8192;

/** @emoji 🔗️ Resolves the boot-time {@link AppRole} from a raw `location.search`.
 *
 * `"viewer"` and `"editor"` are the only accepted spellings; an absent OR unrecognized `?role=` falls
 * back to `fallback` (the React entry passes the `VITE_SEMIO_APP_ROLE` env default, which is itself
 * `"editor"` when unset — so with no env the behaviour is byte-identical to the wgpu boot's
 * `params.get("role") === "viewer" ? "viewer" : "editor"`). Throws on an oversized query rather than
 * silently parsing it.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts */
export function resolveBootQueryAppRole(search: string, fallback: AppRole): AppRole {
  if (search.length > BOOT_QUERY_CAPACITY) throw new Error(`boot-query-overflow: search exceeds ${BOOT_QUERY_CAPACITY} code units`);
  const raw = new URLSearchParams(search).get(BOOT_QUERY_APP_ROLE_PARAM);
  if (raw === "viewer") return "viewer";
  if (raw === "editor") return "editor";
  return fallback;
}

/** @emoji 📚️ Resolves the boot-time example id from a raw `location.search`.
 *
 * An absent or EMPTY `?example=` falls back to `fallback` (the React entry passes its
 * `VITE_SEMIO_DEFAULT_EXAMPLE` seed, `undefined` when unset), so `?example=` spelled with no value is
 * the same as not spelling it at all rather than a request for an example named `""`. Whether the id
 * names a real example is NOT decided here — that is the open dialect's own
 * `manifest::examples_for_app` list, which each shell already consults; an unknown id is dropped there.
 * Throws on an oversized query rather than silently parsing it.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts */
export function resolveBootQueryExampleId(search: string, fallback: string | undefined): string | undefined {
  if (search.length > BOOT_QUERY_CAPACITY) throw new Error(`boot-query-overflow: search exceeds ${BOOT_QUERY_CAPACITY} code units`);
  const raw = new URLSearchParams(search).get(BOOT_QUERY_EXAMPLE_PARAM);
  return raw === null || raw === "" ? fallback : raw;
}
