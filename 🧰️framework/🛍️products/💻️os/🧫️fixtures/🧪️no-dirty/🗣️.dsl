name=no-dirty
graph {
  schema=workflow.graph nodes=[ id=node-1 plugin-id=draw app-id=draw label=node-1 yields="2d.drawing" document-ref=documents/node-1 config-ref=config/node-1 x=0 y=0 width=160 height=72 inputs=[ id="node-1:in" port_id=in label=Port direction=in class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] outputs=[ id="node-1:out" port_id=out label=Port direction=out class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] id=node-2 plugin-id=draw app-id=draw label=node-2 yields="2d.drawing" document-ref=documents/node-2 config-ref=config/node-2 x=200 y=0 width=160 height=72 inputs=[ id="node-2:in" port_id=in label=Port direction=in class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] outputs=[ id="node-2:out" port_id=out label=Port direction=out class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] ] edges=[ id=edge-1 source-node-id=node-1 source-port-id="node-1:out" target-node-id=node-2 target-port-id="node-2:in"
  contract {
    kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
  }
  ]
}
dirty-node-ids=[ ]
expected-deliveries [edge-id:TEXT producer-node-id:TEXT producer-port-id:TEXT consumer-node-id:TEXT consumer-port-id:TEXT] {
}
