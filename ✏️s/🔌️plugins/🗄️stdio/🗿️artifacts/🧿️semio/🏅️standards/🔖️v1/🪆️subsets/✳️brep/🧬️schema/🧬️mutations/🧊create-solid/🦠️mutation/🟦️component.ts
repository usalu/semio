/** mutation payload — mirrors `CreateSolid`. */
export interface CreateSolid {
  id: string;
  shells: { shell: string; isVoid: boolean }[];
}
