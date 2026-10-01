/** 🧫️ Complete handcrafted DWG fixture covering all owned body, entity, record and constraint variants. */
import type { DwgSnapshot,DwgLogicalObjectBody } from "../../🟦️.ts";
import { dwgDocumentFixture } from "./📄️document/🟦️.ts";
import { dwgHeaderFixture } from "./🔧️header/🟦️.ts";
import { dwgDrawingFixture,dwgDrawingBodies } from "./✏️drawing/🟦️.ts";
import { dwgObjectBodies } from "./📦️objects/🟦️.ts";
import { dwgBlockBodies } from "./🧩️blocks/🟦️.ts";
import { dwgLayoutFixture } from "./📃️layout/🟦️.ts";
import { dwgConstraintFixture } from "./📏️constraints/🟦️.ts";
import { dwgRecordFixtures } from "./📇️records/🟦️.ts";
import { dwgEntityFixtures } from "./📐️entities/🟦️.ts";
import { dwgMaterialFixture,dwgMlineFixture,dwgVisualFixture,dwgTableStyleFixture,dwgMLeaderFixture } from "./🖌️styles/🟦️.ts";

const bodies:DwgLogicalObjectBody[]=[...dwgDrawingBodies,...dwgObjectBodies,...dwgBlockBodies,
  {kind:"material",value:dwgMaterialFixture},{kind:"mlineStyle",value:dwgMlineFixture},{kind:"visualStyle",value:dwgVisualFixture},
  {kind:"tableStyle",value:dwgTableStyleFixture},{kind:"mLeaderStyle",value:dwgMLeaderFixture},{kind:"layout",value:dwgLayoutFixture},
  {kind:"assoc2dConstraintGroup",value:dwgConstraintFixture},
  ...dwgRecordFixtures.map(value=>({kind:"tableRecord" as const,value})),...dwgEntityFixtures.map(value=>({kind:"entity" as const,value}))];

export const dwgSnapshotFixture:DwgSnapshot={...dwgDocumentFixture,header:dwgHeaderFixture(0x7ff0000000000001n),drawing:{...dwgDrawingFixture,objects:bodies.map((body,index)=>({handle:18446744073709551615n-BigInt(index),typeCode:65535,className:"owned",category:"custom",ownerHandle:index===0?undefined:0n,extensionDictionaryHandle:18446744073709551615n,reactorHandles:[0n,18446744073709551615n,0n],referencedHandles:[9007199254740993n],extendedData:[{applicationHandle:18446744073709551615n,values:[{kind:"real",groupCode:40,value:{bits:0xfff0123456789abcn}}]}],body}))}};
