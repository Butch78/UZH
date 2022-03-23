// My solution was inpsired by the this solution:
// https://www.hackertouch.com/quick-sort-in-rust.html

fn main() {
    println!("Rust Quick Sort Algorithm");

    println!("Sort Integers in Ascending Order");

    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];

    println!("Before the Sort: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("After the Sort:  {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["Dog", "Town", "Zebra", "Kate", "Tim", "Matthew"];
    println!("Before: {:?}", strings);
    quick_sort(&mut strings);
    println!("After:  {:?}\n", strings);

    println!("Sort floats in ascending order");
    let mut floats = [0.0, 965.12, -2.0, -3.0, -4.0, -5.0, 6.45, -43.1];
    println!("Before: {:?}", floats);

    // <T: Ord> is not support for the Float type so I needed to implement a Float sort function
    float_quick_sort(&mut floats);
    println!("After:  {:?}\n", floats);
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        // Pick a pivot
        let pivot = partition(arr, 0, arr.len() as isize - 1);

        // Sort the left and right sides
        quick_sort(&mut arr[pivot as usize + 1..]);
        quick_sort(&mut arr[0..pivot as usize]);
    }
}

// Partition the array around a pivot point
fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut index = low - 1;
    let mut end = high;

    // I could have used a while loop here but I wasn't able to get it to function properly with Strings.
    loop {
        index += 1;
        while arr[index as usize] < arr[pivot] {
            index += 1;
        }
        end -= 1;
        while end >= 0 && arr[end as usize] > arr[pivot] {
            end -= 1;
        }
        if index >= end {
            break;
        } else {
            arr.swap(index as usize, end as usize);
        }
    }
    arr.swap(index as usize, pivot as usize);
    index
}

fn float_quick_sort(arr: &mut [f64]) {
    if arr.len() > 1 {
        // Pick a pivot
        let pivot = partition_float(arr, 0, arr.len() as isize - 1);

        // Sort the left and right sides

        float_quick_sort(&mut arr[pivot as usize + 1..]);
        float_quick_sort(&mut arr[0..pivot as usize]);
    }
}

//
fn partition_float(arr: &mut [f64], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut index = low - 1;
    let mut end = high;

    loop {
        index += 1;
        while arr[index as usize] < arr[pivot] {
            index += 1;
        }
        end -= 1;
        while end >= 0 && arr[end as usize] > arr[pivot] {
            end -= 1;
        }
        if index >= end {
            break;
        } else {
            arr.swap(index as usize, end as usize);
        }
    }
    arr.swap(index as usize, pivot as usize);
    index
}
