/** 🗄️ Local session-broker contract shared by its issuer and development clients. */
export const LOCAL_SESSION_BROKER_SCHEMA = "semio.hub.local-session-broker/v1";
export const LOCAL_SESSION_REQUEST_SCHEMA = "semio.hub.local-session-request/v1";
export const LOCAL_SESSION_SCHEMA = "semio.hub.local-session/v1";
/** 📄️ The broker record's file name inside the hub data root. */
export const LOCAL_SESSION_BROKER_FILE = "local-session-broker.json";
export const LOCAL_SESSION_BROKER_REQUEST_MAX_BYTES = 256;
const LOCAL_SESSION_TOKEN_PATTERN = /^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u;
/** 👤️ The bounded profile identifier shared by broker requests and development contributions. */
export function isLocalSessionProfileIdV1(value: unknown): value is string {
  return typeof value === "string" && value.length <= 64 && /^[a-z0-9](?:[a-z0-9.-]*[a-z0-9])?$/u.test(value);
}

export type LocalSessionBrokerRecordV1 = Readonly<{ schema: typeof LOCAL_SESSION_BROKER_SCHEMA; hubOrigin: string; runId: string; port: number; secret: string; profiles: readonly string[] }>;
export type LocalSessionV1 = Readonly<{ schema: typeof LOCAL_SESSION_SCHEMA; profileId: string; token: string; userId: string }>;
/** 🧾️ Parses one broker record exactly; anything else is not a broker this launcher trusts. */
export function parseLocalSessionBrokerRecordV1(value: unknown): LocalSessionBrokerRecordV1 {
  const record = value as Record<string, unknown> | null;
  if (
    record === null ||
    typeof record !== "object" ||
    Object.keys(record).sort().join(",") !== "hubOrigin,port,profiles,runId,schema,secret" ||
    record.schema !== LOCAL_SESSION_BROKER_SCHEMA ||
    typeof record.hubOrigin !== "string" ||
    !/^http:\/\/127\.0\.0\.1:\d{1,5}$/u.test(record.hubOrigin) ||
    typeof record.runId !== "string" ||
    !/^[0-9a-f]{32}$/u.test(record.runId) ||
    !Number.isSafeInteger(record.port) ||
    (record.port as number) < 1 ||
    (record.port as number) > 65_535 ||
    typeof record.secret !== "string" ||
    !/^[0-9a-f]{64}$/u.test(record.secret) ||
    !Array.isArray(record.profiles) ||
    record.profiles.length < 1 ||
    record.profiles.length > 8 ||
    record.profiles.some((profile) => !isLocalSessionProfileIdV1(profile))
  )
    throw new Error("local session broker record invalid");
  return Object.freeze({ ...(record as LocalSessionBrokerRecordV1), profiles: Object.freeze([...(record.profiles as string[])]) });
}

/** 🧾️ Parses one broker request exactly and answers its profile id. */
export function parseLocalSessionRequestV1(value: unknown): string {
  const request = value as Record<string, unknown> | null;
  if (request === null || typeof request !== "object" || Object.keys(request).sort().join(",") !== "profileId,schema" || request.schema !== LOCAL_SESSION_REQUEST_SCHEMA || !isLocalSessionProfileIdV1(request.profileId))
    throw new Error("local session request invalid");
  return request.profileId;
}

/** 🧾️ Parses one issued session exactly. */
export function parseLocalSessionV1(value: unknown): LocalSessionV1 {
  const session = value as Record<string, unknown> | null;
  if (
    session === null ||
    typeof session !== "object" ||
    Object.keys(session).sort().join(",") !== "profileId,schema,token,userId" ||
    session.schema !== LOCAL_SESSION_SCHEMA ||
    typeof session.profileId !== "string" ||
    session.profileId.length > 64 ||
    !isLocalSessionProfileIdV1(session.profileId) ||
    typeof session.token !== "string" ||
    !LOCAL_SESSION_TOKEN_PATTERN.test(session.token) ||
    typeof session.userId !== "string" ||
    session.userId.length === 0 ||
    session.userId.length > 128
  )
    throw new Error("local session invalid");
  return Object.freeze({ ...(session as LocalSessionV1) });
}


/** 🪪️ The session capability grammar used by broker and administrator clients. */
export function isLocalSessionTokenV1(value: string): boolean { return LOCAL_SESSION_TOKEN_PATTERN.test(value); }
