/** ⚙️ update-match-params mutation payload — full-record replace of one `ReconstructionParams` sub-struct. */
export interface UpdateMatchParams {
  params: { matcher: "brute-force" | "kd-tree"; ratioTest: number; crossCheck: boolean; sequentialWindow: number; maxPairsPerFrame: number; loopClosure: boolean; };
}
