use crate::values::value::Value;

struct Gate {
    pub id: String,  // Gate ID | 12#34#56#78
    gate: String, // Gate Type | IO.INPUT.#
    priority: u32, // whoever exceeds this limit, congrats!

    pub inputs: Vec<Box<dyn Value>>,
    pub outputs: Vec<Box<dyn Value>>
}