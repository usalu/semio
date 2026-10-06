/** 🧫️ Handcrafted DWG typed boundary fixture shared by component and complete snapshot laws. */
import type { DwgLogicalObjectBody,DwgBlockElement,DwgBlockGrip,DwgBlockTwoPointParameter,DwgBlockAction,DwgBlockActionConnection } from "../../../../../../🧬️schema/🟦️.ts";

const nan={bits:0x7ff0123456789abcn},zero={bits:0x8000000000000000n},infinity={bits:0xfff0000000000000n},max=18446744073709551615n;
const element:DwgBlockElement={name:"same",evaluationExpression:{parentId:-2147483648,majorVersion:4294967295,minorVersion:0,nodeId:4294967295,value:{kind:"double",value:nan}}};
const properties=[{connections:[{code:4294967295,name:"same"},{code:0,name:"same"}]},{connections:[]}],reference={nodeId:4294967295,expressionName:"\0"};
const grip:DwgBlockGrip={element,location:[nan,zero,infinity],insertionCycling:true,insertionCyclingWeight:-2147483648,updatedX:reference,updatedY:{nodeId:0,expressionName:""}};
const parameter:DwgBlockTwoPointParameter={element,showProperties:true,chainActions:false,definitionBase:[nan,zero],definitionEnd:[],properties,propertyExpressionReferences:[{propertyIndex:4294967295,nodeId:0},{propertyIndex:0,nodeId:4294967295}],baseLocation:"midpoint"};
const action:DwgBlockAction={evaluationExpression:element.evaluationExpression,name:"action",displayLocation:[],dependencies:[{objectHandle:max},{objectHandle:0n}],actionNodeIds:[4294967295,0,4294967295]};
const connection:DwgBlockActionConnection={nodeId:4294967295,name:"\0"};
const bodies:DwgLogicalObjectBody[]=[
  {kind:"blockFlipParameter",value:{evaluationExpression:element.evaluationExpression,name:"flip",showProperties:false,chainActions:true,definitionBase:[nan],definitionEnd:[zero,infinity],properties,baseLocation:"startPoint",label:"label",description:"desc",valueSet:{baseLabel:"",flippedLabel:"\0"},labelPoint:[],updatedFlip:reference}},
  {kind:"blockVisibilityParameter",value:{evaluationExpression:element.evaluationExpression,elementName:"visibility",showProperties:true,chainActions:false,definitionPoint:[nan],properties,updatedVisibilityNodeId:4294967295,initialized:false,name:"states",description:"",evaluationHistory:"required",eligibleEntityHandles:[max,0n,max],states:[{name:"same",visibleEntityHandles:[],controlledExpressionHandles:[max]},{name:"same",visibleEntityHandles:[0n,max],controlledExpressionHandles:[]}]}},
  {kind:"blockLinearParameter",value:{parameter,distanceName:"name",distanceDescription:"desc",labelOffset:nan,allowedValues:[zero,infinity,nan]}},{kind:"blockLinearGrip",value:{grip,orientation:[zero]}},{kind:"blockFlipGrip",value:{grip,updatedFlip:reference,orientation:[]}},{kind:"blockVisibilityGrip",value:{grip}},
  {kind:"blockAlignmentParameter",value:{parameter,updatedGripNodeId:4294967295,alignPerpendicular:true}},{kind:"blockAlignmentGrip",value:{grip,firstLocationNodeId:0,secondLocationNodeId:4294967295,orientation:[infinity,nan]}},
  {kind:"blockBasePointParameter",value:{parameter:{element,showProperties:false,chainActions:true,definitionPoint:[],properties},point:[nan],basePoint:[zero,infinity]}},
  {kind:"blockVerticalConstraintParameter",value:{parameter,displacementGripNodeId:4294967295,dependencyHandle:max,expressionName:"",expressionDescription:"",value:nan,allowedValues:{values:[infinity,zero]}}},{kind:"blockHorizontalConstraintParameter",value:{parameter,displacementGripNodeId:0,dependencyHandle:0n,expressionName:"h",expressionDescription:"d",value:zero,allowedValues:{values:[]}}},
  {kind:"blockMoveAction",value:{action,xConnection:connection,yConnection:{nodeId:0,name:""},distanceMultiplier:nan,angleOffset:zero,coordinateMode:"cartesianXy"}},
  {kind:"blockStretchAction",value:{action,xConnection:connection,yConnection:connection,points:[[],[nan,zero,infinity]],selections:[{objectHandle:max,vertexIndices:[4294967295,0]},{objectHandle:0n,vertexIndices:[]}],selectors:[{nodeId:4294967295,pointIndices:[0,4294967295]},{nodeId:0,pointIndices:[]}],distanceMultiplier:nan,angleOffset:infinity,coordinateMode:"cartesianXy"}},
  {kind:"blockScaleAction",value:{base:{action,offset:[zero],xBaseConnection:connection,yBaseConnection:connection,dependent:true,basePoint:[nan,infinity]},uniformScaleConnection:connection,xScaleConnection:connection,yScaleConnection:connection,mode:"xy"}},
  {kind:"blockFlipAction",value:{action,flipConnection:connection,updatedFlipConnection:connection,updatedBaseConnection:connection,updatedEndConnection:connection}}
];

export { bodies as dwgBlockBodies };
