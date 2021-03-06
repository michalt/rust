#![feature(existential_type)]

trait IterBits {
    type BitsIter: Iterator<Item = u8>;
    fn iter_bits(self, n: u8) -> Self::BitsIter;
}

existential type IterBitsIter<T, E, I>: std::iter::Iterator<Item = I>;
//~^ ERROR could not find defining uses

impl<T, E> IterBits for T
where
    T: std::ops::Shr<Output = T>
        + std::ops::BitAnd<T, Output = T>
        + std::convert::From<u8>
        + std::convert::TryInto<u8, Error = E>,
    E: std::fmt::Debug,
{
    type BitsIter = IterBitsIter<T, E, u8>;
    fn iter_bits(self, n: u8) -> Self::BitsIter {
    //~^ ERROR type parameter `E` is part of concrete type but not used
        (0u8..n)
            .rev()
            .map(move |shift| ((self >> T::from(shift)) & T::from(1)).try_into().unwrap())
    }
}
