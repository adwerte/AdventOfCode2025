mod battery_joltage;
mod paper_rolls;
mod safeopening;
mod shopping_ids;
fn main() {
    // Day 1
    crate::safeopening::main();

    // Day 2
    //crate::invalid_ids::main();

    // Day 3
    crate::battery_joltage::main();

    //Day 4
    crate::paper_rolls::main();

    //Day 5
    crate::shopping_ids::main();
}
