name=dead-end
graph {
  schema=workflow.graph nodes=[ id=node-1 plugin-id=draw app-id=draw label=node-1 yields="2d.drawing" document-ref=documents/node-1 config-ref=config/node-1 x=0 y=0 width=160 height=72 inputs=[ ] outputs=[ id="node-1:out" port_id=out label=Port direction=out class=twoD form=vector kind_id="2d.drawing" required=false multiplicity=one ] ] edges=[ ]
}
dirty-node-ids=[ node-1 ]
expected-deliveries [edge-id:TEXT producer-node-id:TEXT producer-port-id:TEXT consumer-node-id:TEXT consumer-port-id:TEXT] {
}
