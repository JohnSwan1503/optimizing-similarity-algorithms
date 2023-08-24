use super::Packed;

pub enum FlattenAxis {
    Row,
    Column,
}

pub fn flatten<T, const N: usize>( data: Vec<Vec<Option<T>>>
                                 , axis: FlattenAxis ) -> Vec<Packed<T, N>> 
    where [(); N / std::mem::size_of::<T>()]: Sized,
          T: Sized + Copy + Default {
    match axis {
        FlattenAxis::Column => {

            let chunk_count = data[0].len() / (N / std::mem::size_of::<T>()) + 1;
            let units_per_chunk = N / std::mem::size_of::<T>();
            let out = Vec::with_capacity((chunk_count - 1) * data.len() + 1);
            (0..chunk_count).fold(out, |out, i|  {
                let end_index = std::cmp::min(data[0].len(), (i + 1) * units_per_chunk);
                data.iter().fold(out, |mut out, row| {
                    let chunk = row[(i * units_per_chunk)..end_index]
                        .iter()
                        .map(|x| x.unwrap_or_default())
                        .collect::<Vec<T>>();
                    out.push(Packed::from(chunk));
                    out
                })
                })
            },
        FlattenAxis::Row => {

            data.iter()
                .map(|row| Packed::from(row.clone()))
                .collect()

        }
    }
}