
use std::collections::*;
use std::marker::PhantomData;

//// --------------------------------------------------------------------
//// Types
//
///// Implemented on types.
//pub trait Type {
//    const BITS: usize;
//}
//
//pub struct Bool;
//impl Type for Bool {
//    const BITS: usize = 1;
//}
//
//pub struct UInt<const N: usize>;
//impl <const N: usize> Type for UInt<N> {
//    const BITS: usize = N;
//}
//
//
//pub struct ReadPort<const ADDR_N: usize, const DATA_N: usize> {
//    addr: UInt<ADDR_N>,
//    data: UInt<DATA_N>,
//}
//impl <const ADDR_N: usize, const DATA_N: usize> Type 
//    for ReadPort<ADDR_N, DATA_N> 
//{
//    const BITS: usize = ADDR_N + DATA_N;
//}
//
//pub struct WritePort<const ADDR_N: usize, const DATA_N: usize> {
//    addr: UInt<ADDR_N>,
//    data: UInt<DATA_N>,
//    en:   Bool,
//}
//impl <const ADDR_N: usize, const DATA_N: usize> Type 
//    for WritePort<ADDR_N, DATA_N> 
//{
//    const BITS: usize = ADDR_N + DATA_N;
//}
//
//// --------------------------------------------------------------------
//// Components
//
//#[derive(Copy, Clone)]
//pub enum Dir { In, Out, InOut }
//pub trait Component {
//    fn bits() -> usize;
//    fn dir(&self) -> Dir;
//    fn ty(&self) -> dyn Type;
//}
//
//
//pub struct Port<T: Type> {
//    dir: Dir,
//    _t: PhantomData<T>,
//}
//impl <T: Type> Component for Port<T> { 
//    fn bits(&self) -> usize { T::bits() } 
//    fn dir(&self) -> Dir { self.dir }
//}
//
//pub struct Wire<T: Type> {
//    _t: PhantomData<T>,
//}
//impl <T: Type> Component for Wire<T> { 
//    fn bits(&self) -> usize { T::bits() }
//    fn dir(&self) -> Dir { Dir::InOut }
//}
//
//pub struct Reg<T: Type> {
//    _t: PhantomData<T>,
//}
//impl <T: Type> Component for Reg<T> { 
//    fn bits(&self) -> usize { T::BITS }
//    fn dir(&self) -> Dir { Dir::InOut }
//}





















