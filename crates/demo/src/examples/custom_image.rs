use yakui::image;

use crate::ExampleState;

pub fn run(state: &mut ExampleState) {
    image(state.custom, [512.0, 512.0]);
}
