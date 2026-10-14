(component
  (import "semio:framework/pure@1.0.0" (instance $pure
    (type $return-kind (enum "bytes" "string-bytes" "u64-list" "ui-patch-list" "patch-op-list" "effect-list" "presence-update-list" "string-pair-list"))
    (export "operation-return-allocation" (type $receiving-return-kind (eq $return-kind)))
    (export "operation-reserve-return" (func async (param "kind" $receiving-return-kind) (param "count" u64) (result u32)))
    (export "operation-begin" (func async (result (result u64 (error u32)))))
    (export "operation-allocation" (func async (param "bytes" u64) (param "owned" u64) (param "next" u64) (param "maximum" u64) (result u32)))
    (export "operation-finish" (func async (param "owned" u64) (result u32)))
  ))
  (alias export $pure "operation-begin" (func $begin))
  (alias export $pure "operation-reserve-return" (func $reserve-return))
  (core func $reserve-return-lowered (canon lower (func $reserve-return)))
  (core module $memory-owner (memory (export "memory") 1) (data (i32.const 64) "\00\01\02\03\04\05\06\07\08\09\0a"))
  (core instance $memory-instance (instantiate $memory-owner))
  (alias core export $memory-instance "memory" (core memory $memory))
  (core func $begin-lowered (canon lower (func $begin) (memory $memory)))
  (alias export $pure "operation-allocation" (func $allocation))
  (alias export $pure "operation-finish" (func $finish))
  (core func $allocation-lowered (canon lower (func $allocation)))
  (core func $finish-lowered (canon lower (func $finish)))
  (core module $guest
    (import "" "memory" (memory 1))
    (import "" "begin" (func $begin (param i32)))
    (import "" "reserve-return" (func $reserve-return (param i32 i64) (result i32)))
    (import "" "allocation" (func $allocation (param i64 i64 i64 i64) (result i32)))
    (import "" "finish" (func $finish (param i64) (result i32)))
    (func (export "begin") (result i32) i32.const 0 call $begin i32.const 0)
    (global $physical (mut i32) (i32.const 0))
    (func (export "run") (param $bytes i64) (param $maximum i64) (result i32)
      (local $code i32)
      local.get $bytes
      i64.const 0
      local.get $bytes
      local.get $maximum
      call $allocation
      local.tee $code
      if
        i64.const 0
        call $finish
        drop
        local.get $code
        return
      end
      global.get $physical
      i32.const 1
      i32.add
      global.set $physical
      local.get $bytes
      call $finish
    )
    (func $refused (param $code i32) (result i32)
      i32.const 32 i32.const 1 i32.store8
      i32.const 36 local.get $code i32.store
      i32.const 32
    )
    (func (export "return-bytes") (param $bytes i64) (param $maximum i64) (param $count i64) (result i32)
      (local $code i32)
      local.get $bytes i64.const 0 local.get $bytes local.get $maximum call $allocation local.tee $code
      if i64.const 0 call $finish drop local.get $code call $refused return end
      i32.const 0 local.get $count call $reserve-return local.tee $code
      if local.get $bytes call $finish drop local.get $code call $refused return end
      local.get $bytes call $finish local.tee $code
      if local.get $code call $refused return end
      global.get $physical i32.const 1 i32.add global.set $physical
      i32.const 32 i32.const 0 i32.store8
      i32.const 36 i32.const 64 i32.store
      i32.const 40 local.get $count i32.wrap_i64 i32.store
      i32.const 32
    )
    (func (export "physical") (result i32) global.get $physical)
  )
  (core instance $guest-instance (instantiate $guest (with "" (instance
    (export "memory" (memory $memory))
    (export "begin" (func $begin-lowered))
    (export "reserve-return" (func $reserve-return-lowered))
    (export "allocation" (func $allocation-lowered))
    (export "finish" (func $finish-lowered))
  ))))
  (func (export "begin") async (result (result u64 (error u32))) (canon lift (core func $guest-instance "begin") (memory $memory)))
  (func (export "run") async (param "bytes" u64) (param "maximum" u64) (result u32) (canon lift (core func $guest-instance "run")))
  (func (export "return-bytes") async (param "bytes" u64) (param "maximum" u64) (param "count" u64) (result (result (list u8) (error u32))) (canon lift (core func $guest-instance "return-bytes") (memory $memory)))
  (func (export "physical") (result u32) (canon lift (core func $guest-instance "physical")))
)
