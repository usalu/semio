name=diamond
graph {
  schema=workflow.graph nodes=[ id=node-a plugin-id=draw app-id=draw label=node-a yields="2d.drawing" document-ref=documents/node-a config-ref=config/node-a x=0 y=0 width=160 height=72 inputs=[ id="node-a:in" port_id=in label=Port direction=in class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] outputs=[ id="node-a:out" port_id=out label=Port direction=out class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] id=node-b plugin-id=draw app-id=draw label=node-b yields="2d.drawing" document-ref=documents/node-b config-ref=config/node-b x=200 y=-80 width=160 height=72 inputs=[ id="node-b:in" port_id=in label=Port direction=in class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] outputs=[ id="node-b:out" port_id=out label=Port direction=out class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] id=node-c plugin-id=draw app-id=draw label=node-c yields="2d.drawing" document-ref=documents/node-c config-ref=config/node-c x=200 y=80 width=160 height=72 inputs=[ id="node-c:in" port_id=in label=Port direction=in class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] outputs=[ id="node-c:out" port_id=out label=Port direction=out class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] id=node-d plugin-id=draw app-id=draw label=node-d yields="2d.drawing" document-ref=documents/node-d config-ref=config/node-d x=400 y=0 width=160 height=72 inputs=[ id="node-d:in" port_id=in label=Port direction=in class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] outputs=[ id="node-d:out" port_id=out label=Port direction=out class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] ] edges=[ id=edge-ab source-node-id=node-a source-port-id="node-a:out" target-node-id=node-b target-port-id="node-b:in"
  contract {
    kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
  }
  id=edge-ac source-node-id=node-a source-port-id="node-a:out" target-node-id=node-c target-port-id="node-c:in"
  contract {
    kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
  }
  id=edge-bd source-node-id=node-b source-port-id="node-b:out" target-node-id=node-d target-port-id="node-d:in"
  contract {
    kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
  }
  id=edge-cd source-node-id=node-c source-port-id="node-c:out" target-node-id=node-d target-port-id="node-d:in"
  contract {
    kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
  }
  ]
}
dirty-node-ids=[ node-a ]
expected-deliveries [edge-id:TEXT producer-node-id:TEXT producer-port-id:TEXT consumer-node-id:TEXT consumer-port-id:TEXT] {
  edge-ab node-a "node-a:out" node-b "node-b:in"
  edge-ac node-a "node-a:out" node-c "node-c:in"
  edge-cd node-c "node-c:out" node-d "node-d:in"
  edge-bd node-b "node-b:out" node-d "node-d:in"
}
