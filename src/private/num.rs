// just be used in this crate

#[derive(Copy, Clone, PartialEq)]
pub enum Num{
    PosInt(u32),
    NegInt(i32),
    Float(f32),
}

impl Num {
    fn mut_check(&mut self) {
        if let Some(x) = self.check() {
            *self = x;
        }
    }
    
    fn check(&self) -> Option<Self> {
        match *self {
            Num::NegInt(x) => {if x < 0 {None} else {Some(Num::PosInt(x as u32))}}
            Num::Float(x) => {if (x as i32) as f32 != x {None} else {Num::NegInt(x as i32).check()}}
            _ => None,
        }
    }
}

pub trait PrivateIntoNum {
    fn into(self) -> Num;
}

macro_rules! FromPosInt {
    ($t: ty) => {
        impl From<$t> for Num {
            fn from(x: $t) -> Self {
                Num::PosInt(x as u32)
            }
        }
    }
}

FromPosInt!(u8);
FromPosInt!(u16);
FromPosInt!(u32);

macro_rules! FromNegInt {
    ($t: ty) => {
        impl From<$t> for Num {
            fn from(x: $t) -> Self {
                if x < 0 {
                    Num::NegInt(x as i32)
                } else {
                    Num::PosInt(x as u32)
                }
            }
        }
    }
}

FromNegInt!(i8);
FromNegInt!(i16);
FromNegInt!(i32);

impl From<f32> for Num {
    fn from(x: f32) -> Self {
        if let Some(num) = Num::Float(x).check() {
            num
        } else {
            Num::Float(x)
        }
    }
}

macro_rules! RulesPrivateIntoNum {
    ($t: ty) => {
        impl PrivateIntoNum for $t {
            fn into(self) -> Num {
                Num::from(self)
            }
        }
    }
}

RulesPrivateIntoNum!(u8);
RulesPrivateIntoNum!(u16);
RulesPrivateIntoNum!(u32);
RulesPrivateIntoNum!(i8);
RulesPrivateIntoNum!(i16);
RulesPrivateIntoNum!(i32);
RulesPrivateIntoNum!(f32);

macro_rules! IntoNum {
    ($t: ty) => {
        impl Into<$t> for Num {
            fn into(self) -> $t {
                match self {
                    Num::NegInt(x) => x as $t,
                    Num::PosInt(x) => x as $t,
                    Num::Float(x) => x as $t,
                }
            }
        }
    }
}




IntoNum!(u32);
IntoNum!(u64);
IntoNum!(usize);

IntoNum!(i32);
IntoNum!(i64);
IntoNum!(isize);

IntoNum!(f32);
IntoNum!(f64);
