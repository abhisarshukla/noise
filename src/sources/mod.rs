mod sine;
mod square;

pub use sine::{
    SineParams,
    SineWaveSource,
    generate_sine_wave,
};
pub use square::{
    SquareParams,
    SquareWaveSource,
    generate_square_wave,
};
