#[derive(Debug, Clone)]
pub enum Tag {
    Real {
        key: String,
        description: String,
        value: f64,
        unit: String,
        status: String,
    },
    Int {
        key: String,
        description: String,
        value: f64,
        unit: String,
        status: String,
    },
}
