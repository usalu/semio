name=diamond
graph {
  schema=s.workflow
  nodes [id:TEXT instance-id:TEXT x:NUM y:NUM width:NUM height:NUM inputs:LIST outputs:LIST] {
    node-1 app-a 0 0 160 72 [ id="app-a:in" artifact-kind="2d.drawing" direction=in ] [ id="app-a:out" artifact-kind="2d.drawing" direction=out ]
    node-2 app-b 200 -80 160 72 [ id="app-b:in" artifact-kind="2d.drawing" direction=in ] [ id="app-b:out" artifact-kind="2d.drawing" direction=out ]
    node-3 app-c 200 80 160 72 [ id="app-c:in" artifact-kind="2d.drawing" direction=in ] [ id="app-c:out" artifact-kind="2d.drawing" direction=out ]
    node-4 app-d 400 0 160 72 [ id="app-d:in" artifact-kind="2d.drawing" direction=in ] [ id="app-d:out" artifact-kind="2d.drawing" direction=out ]
  }
  edges [id:TEXT source-node-id:TEXT source-port-id:TEXT target-node-id:TEXT target-port-id:TEXT contract:BLOCK] {
    edge-ab node-1 "app-a:out" node-2 "app-b:in" {
      kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
    }
    edge-ac node-1 "app-a:out" node-3 "app-c:in" {
      kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
    }
    edge-bd node-2 "app-b:out" node-4 "app-d:in" {
      kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
    }
    edge-cd node-3 "app-c:out" node-4 "app-d:in" {
      kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
    }
  }
}
dirty-instance-ids=[ app-a ]
expected-deliveries [edge-id:TEXT producer-instance-id:TEXT producer-port-id:TEXT consumer-instance-id:TEXT consumer-port-id:TEXT] {
  edge-ab app-a "app-a:out" app-b "app-b:in"
  edge-ac app-a "app-a:out" app-c "app-c:in"
  edge-cd app-c "app-c:out" app-d "app-d:in"
  edge-bd app-b "app-b:out" app-d "app-d:in"
}
