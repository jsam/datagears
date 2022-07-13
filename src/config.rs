use tract_onnx::tract_hir::infer::InferenceFact;

#[derive(Debug, Default, Clone)]
pub struct DGConfig {
    pub auto_load_input_facts: bool,
    pub default_input_fact_shape: Option<InferenceFact>,
}

impl DGConfig {
    pub fn new() -> Self {
        DGConfig { ..Self::default() }
    }

    pub fn with_auto_load_input_facts(mut self) -> Self {
        self.auto_load_input_facts = true;
        self
    }

    pub fn with_default_input_fact_shape(mut self, shape: InferenceFact) -> Self {
        self.default_input_fact_shape = Option::from(shape);
        self
    }
}
