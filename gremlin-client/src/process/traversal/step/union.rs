use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct UnionStep {
    params: Vec<GValue>,
}

impl UnionStep {
    fn new(params: Vec<GValue>) -> Self {
        UnionStep { params }
    }
}

impl From<UnionStep> for Vec<GValue> {
    fn from(step: UnionStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for UnionStep {
    fn from(param: TraversalBuilder) -> Self {
        UnionStep::new(vec![param.bytecode.into()])
    }
}

impl From<Vec<TraversalBuilder>> for UnionStep {
    fn from(param: Vec<TraversalBuilder>) -> Self {
        UnionStep::new(param.into_iter().map(|s| s.bytecode.into()).collect())
    }
}

macro_rules! impl_into_coalesce {
    ($n:expr) => {
        impl From<[TraversalBuilder; $n]> for UnionStep {
            fn from(param: [TraversalBuilder; $n]) -> UnionStep {
                UnionStep::new(param.iter().map(|s| s.bytecode.clone().into()).collect())
            }
        }
    };
}

impl_into_coalesce!(1);
impl_into_coalesce!(2);
impl_into_coalesce!(3);
impl_into_coalesce!(4);
impl_into_coalesce!(5);
impl_into_coalesce!(6);
impl_into_coalesce!(7);
impl_into_coalesce!(8);
impl_into_coalesce!(9);
impl_into_coalesce!(10);
