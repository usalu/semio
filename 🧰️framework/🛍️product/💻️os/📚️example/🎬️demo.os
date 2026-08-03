active-plugin-id=cad active-alternative-id=alt-timber-roof programs=[ cad norm ]
workflow {
  schema=workflow.graph nodes=[ id=app-cad-1 plugin-id=cad app-id=cad label="Roof Beam \"B12\"" yields=cad.scene document-ref=documents/app-cad-1 config-ref=config/app-cad-1 x=40 y=80 width=220 height=92 inputs=[ ] outputs=[ id="app-cad-1:cad.out:out" port_id=cad.out label=Out direction=out class=threeD form=brep kind_id=cad.scene required=false multiplicity=one ] id=app-en1995-1 plugin-id=norm app-id=en1995 label="EN 1995 Timber Check" yields=en1995.report document-ref=documents/app-en1995-1 config-ref=config/app-en1995-1 x=320 y=80 width=220 height=92 inputs=[ id="app-en1995-1:cad.in:in" port_id=cad.in label=In direction=in class=threeD form=brep kind_id=cad.scene required=false multiplicity=one ] outputs=[ ] ] edges=[ id=edge-1 source-node-id=app-cad-1 source-port-id="app-cad-1:cad.out:out" target-node-id=app-en1995-1 target-port-id="app-en1995-1:cad.in:in"
  contract {
    kind_id=cad.scene class=threeD form=brep wire_kind=document wire_schema=cad.scene
  }
  ]
}
parameter-bindings [parameter-id:TEXT node-id:TEXT field-path:TEXT] {
  p-span app-cad-1 "/span"
  p-grade app-cad-1 "/grade"
}
numeric id=p-span name=Span value=6 min=2 max=16 step=0.25
categorical id=p-grade name="Timber Grade" value=GL24h options=[ GL24h GL28h C24 ]
toggle id=p-fire name="Fire Rating Required" value=true
text id=p-project name="Project Name" value="Alnus Pavilion \"East Wing\"\nPhase 2"
