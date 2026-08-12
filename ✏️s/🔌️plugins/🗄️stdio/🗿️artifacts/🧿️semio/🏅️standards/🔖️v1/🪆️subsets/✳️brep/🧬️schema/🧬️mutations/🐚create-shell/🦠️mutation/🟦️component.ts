/** mutation payload — mirrors `CreateShell`. */
export interface CreateShell {
  id: string;
  faces: { face: string; orientation: boolean }[];
}
