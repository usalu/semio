/** 🪪️ Closed authenticated session identity returned by the Hub through the local broker. */

export const DIRECTORY_SESSION_AUTHORITY_SCHEMA_V1 = "semio.directory.session-authority.v1";
export const DIRECTORY_SESSION_AUTHORITY_MAX_BYTES = 2048;

export type DirectorySessionAuthorityV1 = Readonly<{
  schema: typeof DIRECTORY_SESSION_AUTHORITY_SCHEMA_V1;
  sessionBindingSha256: string;
  authorizationGeneration: number;
  userId: string;
  email: string;
  displayName: string;
  expiresAt: number;
  sessionKind: "external" | "development-local";
}>;



/** 🛡️ Parses only the canonical bounded response; session capabilities and ids cannot cross it. */

