mod battery_joltage;
mod cellapod_math;
//mod electrical_fire;
mod cabel_connections;
mod factory_floor;
mod movie_teather;
mod paper_rolls;
mod safeopening;
mod shopping_ids;
mod taychon_map;

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

    //Day 6
    crate::cellapod_math::main();

    //Day 7
    crate::taychon_map::main();

    //Day 8
    //crate::electrical_fire::main();

    //Day 9
    crate::movie_teather::main();

    //Day 10
    crate::factory_floor::main();

    //Day 11
    crate::cabel_connections::main();
}
