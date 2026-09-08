// 🧪️ Generated from 📇️fixed-operation-registry-law.json by the root verifier.
#[test]
fn language_neutral_fixed_operation_registry_laws_execute_production_registry() {
    {
        let mut registry = FixedOperationRegistry::<Owner, 1>::new(1);
        let mut output = Vec::new();
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_assert("empty", &mut registry, output, &["close:complete"]);
    }
    {
        let mut registry = FixedOperationRegistry::<Owner, 1>::new(1);
        let mut output = Vec::new();
        fixture_admit(&mut registry, &mut output, OperationId(1), Generation(1), 1, 100, "single-owner");
        fixture_take(&mut registry, &mut output, OperationId(1), Generation(1));
        fixture_inspect(&registry, &mut output);
        fixture_assert("single", &mut registry, output, &["admit:accepted:single-owner", "take:single-owner", "state:0:0:0"]);
    }
    {
        let mut registry = FixedOperationRegistry::<Owner, 4>::new(8);
        let mut output = Vec::new();
        fixture_admit(&mut registry, &mut output, OperationId(0), Generation(0), 1, 101, "slot-0");
        fixture_admit(&mut registry, &mut output, OperationId(1), Generation(0), 1, 102, "slot-1");
        fixture_admit(&mut registry, &mut output, OperationId(2), Generation(0), 1, 103, "slot-2");
        fixture_admit(&mut registry, &mut output, OperationId(3), Generation(0), 1, 104, "slot-3");
        fixture_admit(&mut registry, &mut output, OperationId(4), Generation(0), 1, 105, "slot-plus-one");
        fixture_cancel(&mut registry, &mut output, OperationId(0), Generation(0));
        fixture_cancel(&mut registry, &mut output, OperationId(1), Generation(0));
        fixture_cancel(&mut registry, &mut output, OperationId(2), Generation(0));
        fixture_cancel(&mut registry, &mut output, OperationId(3), Generation(0));
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_assert("maximum-plus-one", &mut registry, output, &["admit:accepted:slot-0", "admit:accepted:slot-1", "admit:accepted:slot-2", "admit:accepted:slot-3", "admit:rejected:slot-plus-one", "cancel:true", "cancel:true", "cancel:true", "cancel:true", "close:pending", "close:pending", "close:pending", "close:complete", "close:complete"]);
    }
    {
        let mut registry = FixedOperationRegistry::<Owner, 4>::new(2);
        let mut output = Vec::new();
        fixture_admit(&mut registry, &mut output, OperationId(0), Generation(0), 1, 106, "collision-live");
        fixture_admit(&mut registry, &mut output, OperationId(4), Generation(0), 1, 107, "collision-returned");
        fixture_cancel(&mut registry, &mut output, OperationId(0), Generation(0));
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_assert("collision", &mut registry, output, &["admit:accepted:collision-live", "admit:rejected:collision-returned", "cancel:true", "close:complete"]);
    }
    {
        let mut registry = FixedOperationRegistry::<Owner, 1>::new(8);
        let mut output = Vec::new();
        fixture_admit(&mut registry, &mut output, OperationId(0), Generation(0), 8, 108, "byte-max");
        fixture_admit(&mut registry, &mut output, OperationId(1), Generation(0), 1, 109, "byte-plus-one");
        fixture_cancel(&mut registry, &mut output, OperationId(0), Generation(0));
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_assert("byte-maximum-plus-one", &mut registry, output, &["admit:accepted:byte-max", "admit:rejected:byte-plus-one", "cancel:true", "close:pending", "close:pending", "close:pending", "close:pending", "close:pending", "close:pending", "close:pending", "close:complete"]);
    }
    {
        let mut registry = FixedOperationRegistry::<Owner, 2>::new(2);
        let mut output = Vec::new();
        fixture_admit(&mut registry, &mut output, OperationId(0), Generation(1), 1, 110, "stale-owner");
        fixture_cancel_stale(&mut registry, &mut output, OperationId(0), Generation(2));
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_assert("cancel-stale", &mut registry, output, &["admit:accepted:stale-owner", "stale:true", "close:pending", "close:complete"]);
    }
    {
        let mut registry = FixedOperationRegistry::<Owner, 1>::new(2);
        let mut output = Vec::new();
        fixture_admit(&mut registry, &mut output, OperationId(9), Generation(1), 1, 111, "aba-old");
        fixture_take(&mut registry, &mut output, OperationId(9), Generation(2));
        fixture_take(&mut registry, &mut output, OperationId(9), Generation(1));
        fixture_admit(&mut registry, &mut output, OperationId(9), Generation(2), 1, 112, "aba-fresh");
        fixture_cancel(&mut registry, &mut output, OperationId(9), Generation(1));
        fixture_cancel(&mut registry, &mut output, OperationId(9), Generation(2));
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_assert("aba", &mut registry, output, &["admit:accepted:aba-old", "take:none", "take:aba-old", "admit:accepted:aba-fresh", "cancel:false", "cancel:true", "close:complete"]);
    }
    {
        let mut registry = FixedOperationRegistry::<Owner, 1>::new(2);
        let mut output = Vec::new();
        fixture_admit(&mut registry, &mut output, OperationId(3), Generation(4), 2, 113, "close-owner");
        fixture_cancel(&mut registry, &mut output, OperationId(3), Generation(4));
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_inspect(&registry, &mut output);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_close(&mut registry, &mut output, 1, 1);
        fixture_assert("interrupted-repeated-close", &mut registry, output, &["admit:accepted:close-owner", "cancel:true", "close:pending", "state:1:2:1", "close:complete", "close:complete"]);
    }
}
