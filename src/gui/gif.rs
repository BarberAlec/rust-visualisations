use gif::{Frame, Encoder, Repeat, EncodingError};
use std::fs::File;
use std::borrow::Cow;


/// Create gif in out_path location with given states
pub fn create_gif<T: Into<u8> + Copy> (out_path: &str, states: Vec<Vec<Vec<T>>>) -> Result<(), EncodingError> {
    if states.is_empty() {panic!("Cannot have empty states")}

    // 2 RGB values in series... black and white here
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];

    // presumeing square shape...
    let dims = states[0].len() as u16;
    let (width, height) = (dims, dims);


    let mut image = File::create(out_path).unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();


    for state in &states {

        // flatten and convert 2d vec to 1d vec of type u8
        let myframe = state.into_iter().flatten().map(|x| Into::<u8>::into(*x)).collect::<Vec<u8>>();
        
        // create mutable frame with default values
        let mut frame = Frame::default();

        frame.width = width;
        frame.height = height;
        frame.delay = 10; // default to 0.1 seconds per frame
        frame.buffer = Cow::Borrowed(&*myframe);

        encoder.write_frame(&frame)?;
    }
    Ok(())
}