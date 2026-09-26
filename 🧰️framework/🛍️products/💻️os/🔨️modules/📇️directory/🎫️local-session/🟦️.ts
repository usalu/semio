/** 🎫️ Development local-session contract — `GET /_semio/dev/local-session`, the endpoint a dev serve answers with a FRESH
 * hub session from the local hub owner's broker, so a shell signs in with no manual step. Pure and zero-import: the dev
 * serve (`🧑‍💻dev/🔌️vite-plugins` `semioLocalHubSessionVitePlugin`) encodes with {@link localHubSessionAnswerV1} and the
 * shell (`🏛️ShellHost`) decodes with {@link parseLocalHubSessionAnswerV1}.
 *
 * The endpoint always answers `200`: a session, or the typed "not offered" when the serve joined a hub without a broker.
 * The shell asks on every boot, and the former `404` put a failed request into the console of every canonical zero-touch
 * session (ticket 26/09/23 U5). Language-neutral rows: `🔣️.json`, shape: `🧬️.schema.json`. */

export const LOCAL_HUB_SESSION_ENDPOINT_V1 = "/_semio/dev/local-session";
export const LOCAL_HUB_SESSION_SCHEMA_V1 = "semio.os.dev-local-hub-session/v1";

export type LocalHubSessionV1 = { readonly token: string; readonly userId: string };

export type LocalHubSessionAnswerV1 =
  | { readonly schema: typeof LOCAL_HUB_SESSION_SCHEMA_V1; readonly offered: true; readonly token: string; readonly userId: string }
  | { readonly schema: typeof LOCAL_HUB_SESSION_SCHEMA_V1; readonly offered: false };

/** 📤️ The serve's answer for what its broker produced — a session, or none. */
export function localHubSessionAnswerV1(session: LocalHubSessionV1 | null): LocalHubSessionAnswerV1 {
  return session === null ? { schema: LOCAL_HUB_SESSION_SCHEMA_V1, offered: false } : { schema: LOCAL_HUB_SESSION_SCHEMA_V1, offered: true, token: session.token, userId: session.userId };
}

/** 📥️ The session an answer offers, or `null` for the typed "not offered" and for any body that is not exactly an answer
 * of this schema — a shell never adopts a token from a malformed body. */
export function parseLocalHubSessionAnswerV1(body: unknown): LocalHubSessionV1 | null {
  if (typeof body !== "object" || body === null || Array.isArray(body)) return null;
  const answer = body as { schema?: unknown; offered?: unknown; token?: unknown; userId?: unknown };
  if (answer.schema !== LOCAL_HUB_SESSION_SCHEMA_V1 || answer.offered !== true || Object.keys(answer).length !== 4) return null;
  if (typeof answer.token !== "string" || answer.token.length === 0 || typeof answer.userId !== "string" || answer.userId.length === 0) return null;
  return { token: answer.token, userId: answer.userId };
}
