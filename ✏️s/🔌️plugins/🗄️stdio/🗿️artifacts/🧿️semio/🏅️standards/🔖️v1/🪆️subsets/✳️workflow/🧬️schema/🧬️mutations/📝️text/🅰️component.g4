// ANTLR4 grammar for the `s.stdio.semio.workflow` mutation text wire format (`SemioWorkflowMutation`'s
// hand-rolled `protocol::OpText::print_op`/`parse_op` — see 🧬️mutations/🦀️component.rs).
grammar Stdio_semio_workflow_mutations;

op: 'no-mutation' | keyword (WS arg)*;
keyword
    : 'set-snapshot' | 'insert-node' | 'remove-node' | 'set-node-kind' | 'set-node-label'
    | 'set-node-position' | 'set-node-param' | 'remove-node-param' | 'insert-edge' | 'remove-edge'
    | 'set-edge-endpoints' | 'set-edge-kind'
    ;
arg: NAME '=' VALUE;

NAME: [a-z] [a-z-]*;
VALUE: ~[ ]+;
WS: [ ]+;
