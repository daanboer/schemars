mod util;
use schemars::JsonSchema;
use util::*;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct Parent {
    child: Child,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(inline)]
struct Child {
    name: String,
    age: u16,
}

#[test]
fn test_non_referenceable_schema() -> TestResult {
    test_default_generated_schema::<Parent>("inline_derive")
}
