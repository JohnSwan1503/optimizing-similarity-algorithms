use super::aligned::Packed;

use std::collections::HashMap;
use std::default;
use std::hash::Hash;
use std::rc::Rc;

#[inline]
pub fn distance_self<T, const N: usize>
    ( data: &Packed<T, N>
    , value_count: &Vec<HashMap<T, u128>>
    , kk: usize
    , take: usize) -> u128
    where [(); N / std::mem::size_of::<T>()]: Sized,
          T: Sized + Copy + Default + Eq + Hash + PartialEq<T>{
    let default = data.cmp_default();
    data.iter()
        .take(take)
        .enumerate()
        .filter(|(_, x)| x != &&T::default())
        .fold(0, |mut acc, (i, val)| {
        match val == &default {
            true => acc += 0,
            _ => acc += value_count[kk * (N / std::mem::size_of::<T>()) + i][val],
        };
        acc
    })
}

#[inline]
pub fn distance_other<T, const N: usize>
    ( data_a: &Packed<T, N>
    , data_b: &Packed<T, N>
    , value_count: &Vec<HashMap<T, u128>>
    , kk: usize
    , take: usize
    , default: u128) -> u128
    where [(); N / std::mem::size_of::<T>()]: Sized,
          T: Sized + Clone + Copy + Default + Eq  + Hash + PartialEq<T>
           {
    data_a.iter()
        .zip(data_b.iter())
        .take(take)
        .enumerate()
        .filter(|(_, (a, b))| a != &&T::default() || b != &&T::default())
        .fold(0, |distance, (k, (a, b))| {
            distance + match a == b {
                true => value_count[kk * (N / std::mem::size_of::<T>()) + k][a],
                false => default,
            }
        })
}
