use crate::process::traversal::{Order, TraversalBuilder};
use crate::structure::{GValue, T};

pub struct FilterStep {
    params: Vec<GValue>,
}

impl FilterStep {
    fn new(params: Vec<GValue>) -> Self {
        FilterStep { params }
    }
}

impl From<FilterStep> for Vec<GValue> {
    fn from(step: FilterStep) -> Self {
        step.params
    }
}

impl From<()> for FilterStep {
    fn from(_: ()) -> Self {
        FilterStep::new(vec![])
    }
}

impl From<&str> for FilterStep {
    fn from(param: &str) -> Self {
        FilterStep::new(vec![String::from(param).into()])
    }
}

impl From<Order> for FilterStep {
    fn from(param: Order) -> Self {
        FilterStep::new(vec![param.into()])
    }
}

impl From<T> for FilterStep {
    fn from(param: T) -> Self {
        FilterStep::new(vec![param.into()])
    }
}

impl From<(&str, Order)> for FilterStep {
    fn from(param: (&str, Order)) -> Self {
        FilterStep::new(vec![param.0.into(), param.1.into()])
    }
}

impl From<(String, Order)> for FilterStep {
    fn from(param: (String, Order)) -> Self {
        FilterStep::new(vec![param.0.into(), param.1.into()])
    }
}

impl From<(TraversalBuilder, Order)> for FilterStep {
    fn from(param: (TraversalBuilder, Order)) -> Self {
        FilterStep::new(vec![param.0.bytecode.into(), param.1.into()])
    }
}

impl From<TraversalBuilder> for FilterStep {
    fn from(param: TraversalBuilder) -> Self {
        FilterStep::new(vec![param.bytecode.into()])
    }
}
