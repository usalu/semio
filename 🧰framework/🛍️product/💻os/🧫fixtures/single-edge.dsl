name=single-edge
graph {
  schema=s.workflow
  nodes [id:TEXT instance-id:TEXT x:NUM y:NUM width:NUM height:NUM inputs:LIST outputs:LIST] {
    node-1 app-1 0 0 160 72 [ id="app-1:in" artifact-kind="2d.drawing" direction=in ] [ id="app-1:out" artifact-kind="2d.drawing" direction=out ]
    node-2 app-2 200 0 160 72 [ id="app-2:in" artifact-kind="2d.drawing" direction=in ] [ id="app-2:out" artifact-kind="2d.drawing" direction=out ]
  }
  edges [id:TEXT source-node-id:TEXT source-port-id:TEXT target-node-id:TEXT target-port-id:TEXT contract:BLOCK] {
    edge-1 node-1 "app-1:out" node-2 "app-2:in" {
      kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
    }
  }
}
dirty-instance-ids=[ app-1 ]
expected-deliveries [edge-id:TEXT producer-instance-id:TEXT producer-port-id:TEXT consumer-instance-id:TEXT consumer-port-id:TEXT] {
  edge-1 app-1 "app-1:out" app-2 "app-2:in"
}
