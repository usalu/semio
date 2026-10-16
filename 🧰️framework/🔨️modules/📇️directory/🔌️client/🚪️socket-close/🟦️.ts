/** 🚪️ Browser hub socket close contract (`🔣️.json`): the codes the browser closes a hub socket (the store worker's document socket,
 * the shell's directory stream) with. A browser
 * `WebSocket.close(1008)` throws `InvalidAccessError` (only 1000 and 3000..4999 are admissible,
 * https://websockets.spec.whatwg.org/#dom-websocket-close), so the old RFC 6455 codes never closed the socket and surfaced as a page
 * error while the hub kept the document's socket (ticket 26/09/23 S15, catalog B2 gis sweep). */

import contract from "./🔣️.json" with { type: "json" };

export const BROWSER_HUB_SOCKET_CLOSE_V1 = contract;

export type BrowserHubSocketCloseReasonV1 = keyof typeof contract.codes;

/** 🔒️ Whether `code` is one a browser `WebSocket.close(code)` accepts. */
export function browserSocketCloseCodeAdmissibleV1(code: number): boolean {
  return code === contract.admissible.normal || (code >= contract.admissible.min && code <= contract.admissible.max);
}

/** 🚪️ Closes `socket` with the contract's code and reason for `reason`. */
export function closeHubSocketV1(socket: Pick<WebSocket, "close">, reason: BrowserHubSocketCloseReasonV1): void {
  const row = contract.codes[reason];
  socket.close(row.code, row.reason);
}
