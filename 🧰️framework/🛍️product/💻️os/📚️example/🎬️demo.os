active-plugin-id=cad active-alternative-id=alt-timber-roof programs=[ cad norm ]
workflow {
  schema=s.workflow
  nodes [id:TEXT instance-id:TEXT x:NUM y:NUM width:NUM height:NUM inputs:LIST outputs:LIST] {
    node-cad-1 app-cad-1 40 80 220 92 [ ] [ id="app-cad-1:cad.out:out" artifact-kind=cad.scene direction=out ]
    node-en1995-1 app-en1995-1 320 80 220 92 [ id="app-en1995-1:cad.in:in" artifact-kind=cad.scene direction=in ] [ ]
  }
  edges [id:TEXT source-node-id:TEXT source-port-id:TEXT target-node-id:TEXT target-port-id:TEXT contract:BLOCK] {
    edge-1 node-cad-1 "app-cad-1:cad.out:out" node-en1995-1 "app-en1995-1:cad.in:in" {
      kind_id=cad.scene class=threeD form=brep wire_kind=document wire_schema=cad.scene
    }
  }
}
app-instances [id:TEXT plugin-id:TEXT app-id:TEXT label:TEXT yields:TEXT document:BLOCK] {
  app-cad-1 cad cad "Roof Beam \"B12\"" cad.scene {
    document-id=doc-cad-1 schema=cad.scene
  }
  app-en1995-1 norm en1995 "EN 1995 Timber Check" en1995.report {
    document-id=doc-en1995-1 schema=en1995.report
  }
}
parameter-bindings [parameter-id:TEXT instance-id:TEXT field-path:TEXT] {
  p-span app-cad-1 "/span"
  p-grade app-cad-1 "/grade"
}
numeric id=p-span name=Span value=6 min=2 max=16 step=0.25
categorical id=p-grade name="Timber Grade" value=GL24h options=[ GL24h GL28h C24 ]
toggle id=p-fire name="Fire Rating Required" value=true
text id=p-project name="Project Name" value="Alnus Pavilion \"East Wing\"\nPhase 2"
