// #region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — Repo server library: PostgreSQL, auth, events, parsing (Next.js API routes).
// #endregion 🧲️Header


import { createHash } from "node:crypto";
import { createOwnedJsonResponse, isOwnedServerResponse, type OwnedServerRequest, type OwnedServerResponse } from "../../🎛️coordinator/🔌️ports/🟦️.ts";
import { getDeveloperByApiKeyHash, type Developer } from "../🗄️persistence/🟦️.ts";

// #region 🔖️auth
// 🔷️#region 🩻️Hashing
export function hashApiKey(key: string): string {
  return createHash("sha256").update(key).digest("hex");
}
// #endregion 🩻️Hashing

// #region 📎️Auth
// Authenticate a request by extracting the Bearer token and resolving to a developer.

export async function authenticateRequest(request: OwnedServerRequest): Promise<Developer | null> {
  const authHeader = request.headers.get("Authorization");
  if (!authHeader) return null;
  const parts = authHeader.split(" ");
  if (parts.length !== 2 || parts[0] !== "Bearer") return null;
  const apiKey = parts[1];
  if (!apiKey) return null;
  const keyHash = hashApiKey(apiKey);
  const developer = await getDeveloperByApiKeyHash(keyHash);
  if (!developer) return null;
  if (!developer.active || !developer.trusted) return null;
  return developer;
}

export function unauthorizedResponse(message: string = "unauthorized"): OwnedServerResponse {
  return createOwnedJsonResponse({ error: message }, 401);
}

export function forbiddenResponse(message: string = "forbidden"): OwnedServerResponse {
  return createOwnedJsonResponse({ error: message }, 403);
}

// 🔐️Require authentication and trusted developer status.
export async function requireAuth(request: OwnedServerRequest): Promise<{ developer: Developer } | OwnedServerResponse> {
  const developer = await authenticateRequest(request);
  if (!developer) return unauthorizedResponse();
  return { developer };
}

// 👑️Require admin or owner role.
export async function requireAdmin(request: OwnedServerRequest): Promise<{ developer: Developer } | OwnedServerResponse> {
  const developer = await authenticateRequest(request);
  if (!developer) return unauthorizedResponse();
  if (developer.role !== "admin" && developer.role !== "owner") {
    return forbiddenResponse("admin access required");
  }
  return { developer };
}

// 📩️Type guard to check if auth result is an error response.
export function isAuthError(result: { developer: Developer } | OwnedServerResponse): result is OwnedServerResponse {
  return isOwnedServerResponse(result);
}
// #endregion 📎️Auth
// #endregion 🔖️auth
