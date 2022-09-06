
use std::marker::PhantomData;

/// Marker trait for types used as module definitions.
pub trait DslModuleDef {
}
/// Marker trait for types used as bundle definitions.
pub trait DslBundleDef {

    ///// The exact number of bits needed to represent this bundle. 
    //const BITS: usize;

}

/// An input port of type T.
pub struct  Input<T>(PhantomData<T>) where T: PrimType;
/// An output port of type T.
pub struct Output<T>(PhantomData<T>) where T: PrimType;
/// An input/output port of type T.
pub struct  InOut<T>(PhantomData<T>) where T: PrimType;

/// A type representing an instance of a bundle definition T.
pub struct Bundle<T>(PhantomData<T>) where T: DslBundleDef;
/// A type representing a fixed-size array of some other type T.
pub struct Array<T, const LEN: usize>(PhantomData<T>) where T: PrimType;
/// An unsigned integer type of some fixed size (in bits).
pub struct UInt<const BITS: usize>;
/// A boolean type.
pub struct Bool;

/// A marker trait for types that are meaningful in the DSL. 
pub trait PrimType { 
    /// The exact number of bits needed to represent this type.
    const BITS: usize;
}
impl <const N: usize> PrimType for UInt<N> {
    const BITS: usize = N;
}
impl <T> PrimType for Bundle<T> where T: DslBundleDef {
    const BITS: usize = 0;
}
impl PrimType for Bool {
    const BITS: usize = 1;
}
impl <T, const LEN: usize> PrimType for Array<T, LEN> where T: PrimType {
    const BITS: usize = T::BITS + LEN;
}


#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn foo() {
        let x: Input<UInt<5>>;
        let out_uint_5: Output<UInt<5>>;
        let out_uint_6: Output<UInt<6>>;
    }
}



