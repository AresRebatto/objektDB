use std::convert::TryInto;

pub trait FromBytes {
    fn from_bytes(data: &[u8]) -> Self;
}

macro_rules! impl_from_bytes {
    ($($t:ty),*) => {
        $(
            impl FromBytes for $t {
                fn from_bytes(data: &[u8]) -> Self {
                    <$t>::from_le_bytes(data.try_into().unwrap())
                }
            }
        )*
    };
}

impl_from_bytes!(i8, i16, i32, i64, i128,
                 u8, u16, u32, u64, u128,
                 f32, f64);


impl FromBytes for bool {
    fn from_bytes(data: &[u8]) -> Self {
        data[0] != 0
    }
}

impl FromBytes for char {
    fn from_bytes(data: &[u8]) -> Self {
        let val = u32::from_le_bytes(data.try_into().unwrap());
        char::from_u32(val).unwrap()
    }
}

impl FromBytes for String {
    fn from_bytes(data: &[u8]) -> Self {
        String::from_utf8(data.to_vec()).unwrap()
    }
}


impl FromBytes for usize {
    fn from_bytes(data: &[u8]) -> Self {
        u64::from_le_bytes(data.try_into().unwrap()) as usize
    }
}

impl FromBytes for isize {
    fn from_bytes(data: &[u8]) -> Self {
        i64::from_le_bytes(data.try_into().unwrap()) as isize
    }
}
