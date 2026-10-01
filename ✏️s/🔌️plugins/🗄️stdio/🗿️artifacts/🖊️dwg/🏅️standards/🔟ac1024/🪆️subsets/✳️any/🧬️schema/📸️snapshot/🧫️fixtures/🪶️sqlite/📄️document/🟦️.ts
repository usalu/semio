/** 🧫️ Handcrafted DWG typed boundary fixture shared by component and complete snapshot laws. */
import fixture from "../../../🧫️fixtures/🪶️sqlite/📄️document/🔣️.json";
import { type DwgDocumentState } from "../../../🪶️sqlite/📄️document/🟦️.ts";

const input:DwgDocumentState={
  schema:fixture.schema,version:fixture.version,maintenanceVersion:fixture.maintenanceVersion,codepage:fixture.codepage,
  summary:{title:fixture.summaryTitle,subject:"typed subject",author:"typed author",keywords:"one,two",comments:"preserved comment",lastSavedBy:"last",revisionNumber:"R17",hyperlinkBase:"relative/base",totalEditingTime:BigInt(fixture.unsignedMaximum),createdAt:{days:4294967295,milliseconds:17},modifiedAt:{days:23,milliseconds:29},customProperties:fixture.customProperties.map(pair=>({key:pair[0]!,value:pair[1]!}))},
  application:{name:fixture.applicationName,versionChecksum:"version digest",version:"noncanonical version",commentChecksum:"comment digest",comment:"typed comment",productChecksum:"product digest",product:"typed product",applicationVersion:"app version"},
  template:{description:"typed template",measurement:"metric"},
  auxiliaryHeader:{totalSaves:4294967295,savePartitionOne:65535,savePartitionTwo:13,saveGeneration:17,legacyStampOne:{version:19,maintenance:23},legacyStampTwo:{version:29,maintenance:31},compatibilityProfile:"autocad2009",createdAt:{days:37,milliseconds:41},updatedAt:{days:43,milliseconds:47},handleSeed:BigInt(fixture.unsignedMaximum),terminalSaveGeneration:53},
  classes:[{number:65535,proxyFlags:4294967295,applicationName:"class app",cppClassName:"CppClass",dxfName:"DXF_CLASS",wasZombie:true,itemClassId:61,objectCount:67,dwgVersion:71,maintenanceVersion:73,reservedValues:[79,4294967295]}],
  dependencies:[{feature:"reference",fullPath:"/typed/🌠",relativePath:"relative",fingerprint:"fingerprint",version:"version",timestamp:4294967295,fileSize:4294967295,affectsGraphics:true,referenceCount:83}],
  applicationHistory:{historyIdentifierOne:"id1",historyIdentifierTwo:"id2",classVersion:4294967295,applicationVersionDigest:"av digest",applicationVersion:"av",trustCommentDigest:"tc digest",trustComment:"tc",propertySetDigest:"ps digest",propertyFormatIdentifier:"format",properties:[{id:4294967295,kind:"dateTime",value:"noncanonical timestamp"},{id:4294967295,kind:"string",value:"duplicate property id"}],productDigest:"product digest",product:{name:"product name",buildVersion:"build",registryVersion:"registry",installId:"install",localeId:"locale"}},
  revisionHistory:{formatMajor:89,formatMinor:97,revisions:fixture.revisionValues},
  preview:{width:fixture.preview.width,height:fixture.preview.height,origin:"bottomUp",palette:fixture.preview.palette.map(row=>({red:row[0]!,green:row[1]!,blue:row[2]!,alpha:row[3]!})),pixelIndices:fixture.preview.pixelIndices,backgroundPaletteIndex:17},
};

export { input as dwgDocumentFixture };
