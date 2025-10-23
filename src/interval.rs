/*

    Responsible for creating a struct that represents
    ranges from a to b and functionality to check if
    x is in range [a,b] or (a,b).

    See also associated constants of Interval class:
    - EMPTY: (inf, -inf)
    - UNIVERSE: (-inf, inf)
    - NONNEGATIVE: (0*, inf)
    - UNIT: (0*, 1*) 
    
    *with epsilon

    @author: Bartu
    @date: Sept 2025

*/

use crate::numeric::{Float};

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: Float,
    pub max: Float,
}

impl Interval {

    pub const EMPTY: Self = Self {
        min: FloatConst::INF,
        max: FloatConst::NEG_INF,
    };

    pub const UNIVERSE: Self = Self {
        min: FloatConst::NEG_INF,
        max: FloatConst::INF,
    };

    pub const NONNEGATIVE: Self = Self {
        min: FloatConst::SLIGHTLY_POSITIVE_ZERO,
        max: FloatConst::INF,
    };

    pub const UNIT:  Self = Self {
        min: FloatConst::SLIGHTLY_POSITIVE_ZERO,
        max: FloatConst::ALMOST_ONE,
    };

    pub fn new(t_min: Float, t_max: Float) -> Interval {
        Interval{min: t_min, max: t_max}
    }

    pub fn size(&self) -> Float {
        self.max - self.min
    }

    pub fn contains(&self, x: Float) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: Float) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: Float) -> Float {
        if x < self.min { self. min }
        else if x > self.max { self.max }
        else { x }
    }

}



// TODO: Allow epsilons to be set by outside of the crate
// Perhaps with use of enums or remove 0* 1* from consts and 
// add functions to construct such epsilon intervals
pub trait FloatConst: Copy {
    const PI: Self;
    const INF: Self;
    const NEG_INF: Self;
    const SLIGHTLY_POSITIVE_ZERO: Self;
    const ALMOST_ONE: Self;
}

impl FloatConst for f32 {
    const PI: Self = std::f32::consts::PI;
    const INF: Self = std::f32::INFINITY;
    const NEG_INF: Self = std::f32::NEG_INFINITY;
    const SLIGHTLY_POSITIVE_ZERO: Self = 0.0001 as f32;
    const ALMOST_ONE: Self = 0.99999 as f32;
}

impl FloatConst for f64 {
    const PI: Self = std::f64::consts::PI;
    const INF: Self = std::f64::INFINITY;
    const NEG_INF: Self = std::f64::NEG_INFINITY;
    const SLIGHTLY_POSITIVE_ZERO: Self = 0.0001 as f64;
    const ALMOST_ONE: Self = 0.99999 as f64;
}