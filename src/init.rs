
pub struct Field {
    pub t: f64,
    pub nb_channels: u32,
    pub m: Vec<Vec<u32>>,
}

impl Field {
    // A function to create an empty field
    pub fn new_field(h: usize, l: usize, nb_chan: u32) -> Field {
        Field {
            t: 0.0,
            nb_channels: nb_chan,
            m: vec![vec![0; l]; h],
        }
    }
}