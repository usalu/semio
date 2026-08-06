// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Scope listing API.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { NextRequest, NextResponse } from "next/server";
// #endregion 🔌️Adapters

import { listScopesByFile } from "@/lib";
import { requireAuth, isAuthError } from "@/lib";

export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const file = request.nextUrl.searchParams.get("file");
  if (!file) {
    return NextResponse.json({ error: "file query required" }, { status: 400 });
  }
  const scopes = await listScopesByFile(file);
  return NextResponse.json(scopes);
}
