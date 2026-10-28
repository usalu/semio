/** 🏛️ Authored horizontal BIM frames for implementation and independent comparisons. */
export const ifcBimAxisFixture=(rotation:number)=>({schema:'s.bim.model@1',project:{name:'Frame',description:'',author:'',organization:'',phase_names:[]},sites:{site:{name:'Site',latitude:47.5,longitude:8.5,elevation:400,true_north:.2,boundary:[]}},buildings:{building:{site:'site',name:'Building',origin:{x:1,y:2},rotation,elevation:4}}});

