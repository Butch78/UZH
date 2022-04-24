// Gorramuth P.
// 6280632

extern crate rand;
extern crate rayon;
use rand::Rng;

use std::time::Instant;
fn par_partition(list: &mut [i64], low: i64, high: usize) -> i64 {
    // Random pivot is almost twice as much slower
    // let num = rand::thread_rng().gen_range(0, list.len());
    // list.swap(num,high);
    //Using middle pivot
    let mid = list.len() / 2;
    list.swap(mid, high);
    let pivot = list[high];
    let mut index = low - 1;
    for i in (low as usize)..high {
        if list[i] <= pivot {
            index += 1;
            list.swap(index as usize, i as usize);
        }
    }
    list.swap((index + 1) as usize, high);
    index + 1
}

fn partition(list: &mut [i64], low: i64, high: usize) -> i64 {
    // Using rightmost pivot
    // To "Randomize" since actual randomization costs more
    let pivot = list[high];
    let mut index = low - 1;
    for i in (low as usize)..high {
        if list[i] <= pivot {
            index += 1;
            list.swap(index as usize, i as usize);
        }
    }
    list.swap((index + 1) as usize, high);
    index + 1
}

fn insertion_sort(ls: &mut [i64]) {
    // Insertion sort is used since it is extremely fast to sort smaller arrays
    for i in 1..ls.len() {
        let mut j = i;
        while (j > 0) && (ls[j] < ls[j - 1]) {
            ls.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn quicksort(ls: &mut [i64], low: i64, high: usize) {
    // Normal quicksort is faster at a moderate array length
    if (low < high as i64) && (ls.len() > 1) {
        let part = partition(ls, low, high);
        quicksort(ls, low, (part - 1) as usize);
        quicksort(ls, part + 1, high);
    }
}

use rayon::prelude::*;
fn parallel_quicksort(ls: &mut [i64], low: i64, high: usize) {
    if (low < high as i64) && (ls.len() > 1) {
        if ls.len() >= 500 {
            // Parallel quicksort is faster at at around 500+ range
            let part = par_partition(ls, low, high - 1);

            let (l, r) = ls.split_at_mut(part as usize);
            // Parallelization is here, divide the arrays into 2 portions and create threads to process
            // each portion independently
            use rayon::join;
            join(
                || parallel_quicksort(l, 0, l.len()),
                || parallel_quicksort(r, 0, r.len()),
            );
        } else if ls.len() <= 50 {
            // Insertion sort is faster at 50- range
            insertion_sort(ls);
        } else {
            // Using normal quicksort within the 500 range is faster
            quicksort(ls, low, high - 1)
        }
    }
}

fn main() {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i64> = (0..50000000)
        .map(|_| {
            // 1 (inclusive) to 21 (exclusive)
            rng.gen_range(1..500000000)
        })
        .collect();
    let mut numbers2 = numbers.clone();
    let mut numbers3 = numbers.clone();
    //----------------------------- BENCHMARK -----------------------------------
    // Parallel quicksort with optimization is much faster than rust's normal sort
    // for very large arrays. Effectiveness increases from 6000+ length onwards

    // println!("{:?}",&numbers2);
    let sz = numbers.len();
    let start = Instant::now();
    let _x = parallel_quicksort(&mut numbers, 0, sz);
    println!("Parallel Time: {:.2?}", start.elapsed());
    // println!("{:?}", &numbers);
    // --- Enhanced parallel sort
    // 50 numbers ~ 13.60us
    // 100 numbers ~ 30.70us
    // 6000 numbers ~ 2.33ms
    // 10000 numbers ~ 2.92ms
    // 500000 numbers ~ 159.93ms
    // 3 million ~ 856.16ms
    // 50 million numbers ~18s
    // 100 million numbers ~ 37.28s
    // 300 million numbers ~130.19s

    let sz2 = numbers2.len();
    let start2 = Instant::now();
    // numbers2.sort();
    let _y = quicksort(&mut *numbers2, 0, sz2 - 1);
    println!("Normal Quicksort Sort Time: {:.2?}", start2.elapsed());
    // println!("{:?}",&numbers2);
    // --- Normal Quicksort
    // 50 numbers ~ 12.80us
    // 100 numbers ~ 28.80us
    // 6000 numbers ~ 5.24ms
    // 10000 numbers ~ 6.72ms
    // 500000 numbers ~ 565.54ms
    // 3 million ~ 3.34s
    // 50 million numbers ~115s
    // 100 million numbers ~ 152.25s
    // 300 miilion numbers ~ 534.75s

    let start3 = Instant::now();
    numbers3.sort_unstable();
    println!("Default Sort Time: {:.2?}", start3.elapsed());
    // --- Default sort
    // 50 numbers ~ 21.20us
    // 100 numbers ~ 31.00us
    // 6000 numbers ~ 2.46ms
    // 10000 numbers ~ 3.92ms
    // 500000 numbers ~ 299.56ms
    // 3 million ~ 1.78s
    // 50 million numbers ~40s
    // 100 million numbers ~ 79.14s
    // 300 million numbers ~ 312.92s
}
