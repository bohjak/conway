extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);
extern crate wasm_mdn;
use wasm_mdn::Universe;

#[cfg(test)]
fn input_spaceship() -> Universe {
    let mut universe = Universe::new(6,6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(6,6);
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

#[wasm_bindgen_test]
fn test_tick() {
    let mut input_universe = input_spaceship();
    let expected_universe = expected_spaceship();

    input_universe.next();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
