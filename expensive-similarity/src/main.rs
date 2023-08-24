// turn off incompatible features
#![allow(incomplete_features, unused_imports, dead_code)]
#![feature(repr_simd, stmt_expr_attributes, portable_simd, generic_const_exprs)]
pub mod aligned;
pub mod flatten;
pub mod metric;

use aligned::{ Packed
             , Packable };
             
use flatten::{ FlattenAxis
             , flatten };

use metric::{ distance_other
            , distance_self };
use std::{ rc::Rc
         , ops::Add
         , fmt::Debug, default};
use rand::distributions::{ Distribution
                         , Uniform
                         , uniform::SampleUniform };

pub fn generate_random_data<T>( n: usize
                              , m: usize ) -> Vec<Vec<Option<T>>> 
    where T: TryFrom<usize> + Debug
        , T::Error: Debug, T: {
    
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..8);
    let mut arr = Vec::new();
    for _ in 0..n {
        let mut inner_vec = Vec::new();
        for _ in 0..m {
            let random_number = die.sample(&mut rng);
            if random_number % 7 == 0 {
                inner_vec.push(None);
            } else {
                inner_vec.push(Some(T::try_from(random_number).unwrap()));
            }
        }
        arr.push(inner_vec);
    }
    arr
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = if args.len() > 1 { args[1].parse::<usize>().unwrap() as u128 } else { 1000 as u128};
    let m = if args.len() > 2 { args[2].parse::<usize>().unwrap() as u128 } else { 127 as u128 };

                
    if args.len() > 3 {
        println!("usage: cargo run [<n> [<m>]]");
        std::process::exit(1);
    }

    let data = generate_random_data::<u8>(n as usize, m as usize);


    let mut value_counts = vec![std::collections::HashMap::new() ; m as usize];

    for i in 0..n as usize {
        for j in 0..m as usize {
            let val = data[i][j];
            match val {
                None => (),
                Some(val) => {
                    let count = value_counts[j].entry(val).or_insert(0 as u128);
                    *count += 1;
                }
            }
        }
    }
    println!("length of data: {}", data.len());
    println!("length of data[0]: {}", data[0].len());
    
    
    // let mut distance_matrix = vec![vec![0; n as usize]; n as usize];
    // let mut itercounter = 0 as u128;
    let default = n as u128;
    let n = n as usize;
    let m = m as usize;
    let mut distance_matrix = vec![vec![0 as u128; n]; n];

    let start = std::time::Instant::now();

    data.iter()
        .enumerate()
        .for_each(|(i, row_i)| {
        let row_i = row_i;
        data.iter()
            .enumerate()
            .skip(i)
            .for_each(|(j, row_j)|{
            distance_matrix[i][j] = 
            row_i
                .iter()
                .zip(row_j.iter())
                .enumerate()
                .filter(|(_, (val_i, val_j))| val_i.is_some() || val_j.is_some())
                .fold(0,|mut distance, (k, (val_i, val_j))| {
                    match val_i == val_j {
                        true => distance += value_counts[k][&val_i.unwrap()],
                        _ => distance += default
                    }
                    distance
                }
            );
        });
    });

    let end = std::time::Instant::now();    
    let mut distance_matrix2 = vec![vec![0 as u128; n]; n];
    let duration_1 = end.duration_since(start);
    let flattened = flatten::<u8, 64>(data, FlattenAxis::Column);
    // // println!("length of flattened: {}", flattened.len());
    // // println!("chunk count: {}", flattened.len() / n);
    
    let chunk_count = flattened.len() / n;
    // let bar_width = chunk_count + 1;
    let start = std::time::Instant::now();
    

    for kk in 0..chunk_count {
        let take = std::cmp::min(m - 64 * kk, 64);
        for i in 0..n {
            let data_i = flattened[kk * 64 + i];
            distance_matrix2[i][i] += distance_self(&data_i, &value_counts, kk, take);
            for j in i+1..n {
                let data_j = flattened[kk * n + j];
                distance_matrix2[i][j] += distance_other(&data_i, &data_j, &value_counts, kk, take, default);
            }
        }
    }

    // for (i, row_i) in data.iter().enumerate() {
    //     for (j, row_j) in data.iter().enumerate().skip(i) {
    //         let mut distance = 0;
    //         for (k, (val_i, val_j)) 
    //             in row_i.iter()
    //                     .zip(row_j.iter())
    //                     .enumerate()
    //                     .filter(|(_, (val_i, val_j))| val_i.is_some() || val_j.is_some()) {
    //                         match val_i == val_j {
    //                             true => distance += value_counts[k][&val_i.unwrap()],
    //                             _ => distance += default
    //                         }
    //         }
    //         distance_matrix[i][j] = distance;
    //     }
    // }
    
    let end = std::time::Instant::now();
    let duration_2 = end.duration_since(start);
    
    let iter_counter_calc = m * n * (n + 1) / 2;
    // println!("iter_counter: {}", itercounter);

    println!("\nduration per iter: {:.4}ns", duration_1.as_nanos() as f64 / iter_counter_calc as f64);
    println!("duration_2: {:?}", duration_2);
    println!("duration per iter: {:.4}ns", duration_2.as_nanos() as f64 / iter_counter_calc as f64);
}
