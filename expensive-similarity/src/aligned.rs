use std::mem::size_of;

pub trait Packable<T: Sized, const N: usize>
where [(); N / size_of::<T>()]: 
{
    type Packed;
    fn consume_vec(vec: Vec<T>) -> Vec<Packed<T, N>> 
    where T: Copy + Default;
    
    fn consume_option_vec(vec: Vec<Option<T>>) -> Vec<Packed<T, N>>
    where T: Copy + Default;
}



#[repr(simd)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Packed<T: Sized, const N: usize>
where [(); N / size_of::<T>()]: 
{
    pub data: [T; N / size_of::<T>()],
}

impl<T: Sized + Copy + Default, const N: usize> Packed<T, N>
where [(); N / size_of::<T>()]: 
{
    #[inline]
    pub fn new() -> Self {
        let _new = Self {
            data: [T::default(); N / size_of::<T>()],
        };
        _new
    }

    #[inline]
    pub fn from_value(value: T) -> Self {
        let _new = Self {
            data: [value; N / size_of::<T>()],
        };

        // confirm that the output will be aligned to N
        assert_eq!(_new.data.as_ptr() as usize % N, 0);

        _new
    }

    #[inline]
    pub fn cmp_default(&self) -> T {
        T::default()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

impl <T: Sized, const N: usize> Packable<T, N> for Packed<T, N>
where [(); N / size_of::<T>()]: 
{
    type Packed = Packed<T, N>;
    fn consume_vec(vec: Vec<T>) -> Vec<Packed<T, N>> 
    where T: Copy + Default {
        
        vec.into_iter()
           .collect::<Vec<T>>()
           .chunks(N / size_of::<T>())
           .map(|x| Packed::<T, N>::from(x))
           .collect::<Vec<Packed<T, N>>>()
    }
    
    fn consume_option_vec(vec: Vec<Option<T>>) -> Vec<Packed<T, N>>
    where T: Copy + Default {
        vec.into_iter()
           .collect::<Vec<Option<T>>>()
           .chunks(N / size_of::<T>())
           .map(|x| Packed::<T, N>::from(x))
           .collect::<Vec<Packed<T, N>>>()
    }
}

impl<T: Sized, const N: usize> From<&[T]> for Packed<T, N>
where [(); N / size_of::<T>()]: Sized,
      T: Copy + Default
{
    #[inline]
    fn from(slice: &[T]) -> Self {
        let mut slice: Vec<T> = slice.iter()
                         .take(N / size_of::<T>())
                         .cloned()
                         .collect::<Vec<T>>();

        slice.resize( N / size_of::<T>()
                    , T::default());

        Self { 
            data: slice.as_slice()
                       .try_into()
                       .unwrap(),
        }
    }
}

impl<T: Sized, const N: usize> From<[T]> for Packed<T, N>
where [(); N / size_of::<T>()]: Sized,
      T: Copy + Default,
      [T]: Sized
{
    #[inline]
    fn from(slice: [T]) -> Self {
        let mut slice: Vec<T> = slice.iter()
                                     .take(N / size_of::<T>())
                                     .cloned()
                                     .collect::<Vec<T>>();

        slice.resize( N / size_of::<T>()
                    , T::default());

        Self { 
            data: slice.as_slice()
                       .try_into()
                       .unwrap(),
        }
    }
}

impl<T: Sized, const N: usize> From<&[Option<T>]> for Packed<T, N>
where [(); N / size_of::<T>()]: Sized,
      T: Copy + Default,
      Option<T>: Sized
{
    #[inline]
    fn from(slice: &[Option<T>]) -> Self {
        let mut slice = slice.iter()
                                     .map(|x| x.unwrap_or_default())
                                     .take(N / size_of::<T>())
                                     .collect::<Vec<T>>();
        
        slice.resize( N / size_of::<T>()
                    , T::default());

        Self { 
            data: slice.as_slice()
                       .try_into()
                       .unwrap(),
        }
    }
}

impl<T: Copy + Default, const N: usize> From<[Option<T>]> for Packed<T, N>
where [(); N / size_of::<T>()]: Sized,
      [Option<T>]: Sized
{
    #[inline]
    fn from(slice: [Option<T>]) -> Self {
        let mut slice = slice.iter()
                                     .map(|x| x.unwrap_or_default())
                                     .take(N / size_of::<T>())
                                     .collect::<Vec<T>>();
        
        slice.resize( N / size_of::<T>()
                    , T::default());

        Self { 
            data: slice.as_slice()
                       .try_into()
                       .unwrap(),
        }
    }
}


impl<T: Sized, const N: usize> From<Vec<T>> for Packed<T, N>
where [(); N / size_of::<T>()]: Sized,
      T: Copy + Default
{
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        let mut slice = 
            vec.into_iter()
               .take(N / size_of::<T>())
               .collect::<Vec<T>>();
        
        slice.resize( N / size_of::<T>()
                    , T::default());

        Self {
            data: slice.as_slice()
                .try_into()
                .unwrap(),
        }
    }
}

impl<T: Sized, const N: usize> From<&Vec<T>> for Packed<T, N>
where [(); N / size_of::<T>()]: Sized,
      T: Copy + Default
{
    #[inline]
    fn from(vec: &Vec<T>) -> Self {
        let mut slice = 
            vec.iter()
               .take(N / size_of::<T>())
               .cloned()
               .collect::<Vec<T>>();
        
        slice.resize( N / size_of::<T>()
                    , T::default());

        Self {
            data: slice.as_slice()
                .try_into()
                .unwrap(),
        }
    }
}

impl<T: Sized, const N: usize> From<Vec<&T>> for Packed<T, N>
where [(); N / size_of::<T>()]: Sized,
      T: Copy + Default 
{
    #[inline]
    fn from(vec: Vec<&T>) -> Self {
        let mut slice = 
            vec.into_iter()
               .take(N / size_of::<T>())
               .cloned()
               .collect::<Vec<T>>();
        
        slice.resize( N / size_of::<T>()
                    , T::default());

        Self {
            data: slice.as_slice()
                .try_into()
                .unwrap(),
        }
    }
}

impl <T:Sized, const N: usize> From<Vec<Option<T>>> for Packed<T, N>
where [(); N / size_of::<T>()]: Sized,
      T: Copy + Default
{
    #[inline]
    fn from(vec: Vec<Option<T>>) -> Self {
        let data: [T; N / size_of::<T>()] = 
            vec.into_iter()
               .map(|x| x.unwrap_or_default())
               .take(N / size_of::<T>())
               .collect::<Vec<T>>()
               .as_slice()
               .try_into()
               .unwrap();
        Self {
            data,
        }
    }
}
