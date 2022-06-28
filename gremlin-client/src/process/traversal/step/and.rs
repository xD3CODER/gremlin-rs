use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct AndStep {
    params: Vec<GValue>,
}

impl AndStep {
    fn new(params: Vec<GValue>) -> Self {
        AndStep { params }
    }
}

impl From<AndStep> for Vec<GValue> {
    fn from(step: AndStep) -> Self {
        step.params
    }
}

impl From<()> for AndStep {
    fn from(_: ()) -> Self {
        AndStep::new(vec![])
    }
}

impl From<TraversalBuilder> for AndStep {
    fn from(param: TraversalBuilder) -> Self {
        AndStep::new(vec![param.bytecode.into()])
    }
}

impl From<Vec<TraversalBuilder>> for AndStep {
    fn from(param: Vec<TraversalBuilder>) -> Self {
        AndStep::new(param.into_iter().map(|s| s.bytecode.into()).collect())
    }
}

macro_rules! impl_into_and {
    ($n:expr) => {
        impl From<[TraversalBuilder; $n]> for AndStep {
            fn from(param: [TraversalBuilder; $n]) -> AndStep {
                AndStep::new(param.iter().map(|s| s.bytecode.clone().into()).collect())
            }
        }
    };
}

impl_into_and!(1);
impl_into_and!(2);
impl_into_and!(3);
impl_into_and!(4);
impl_into_and!(5);
impl_into_and!(6);
impl_into_and!(7);
impl_into_and!(8);
impl_into_and!(9);
impl_into_and!(10);
