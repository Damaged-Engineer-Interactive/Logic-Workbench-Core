struct GateDescription {
    id: String,  // Gate ID | 12#34#56#78
    gate: String, // Gate Type | IO.INPUT.#
    priority: u32, // whoever exceeds this limit, congrats!
}