use std::rand::random;

fn choose_pivot(array: &[int], left: uint, right: uint) -> (int, uint) {
    // take the median of first/middle/last
    let l = array[left];
    let mIndex = (left + right - 1) / 2;
    let m = array[mIndex];
    let r = array[right - 1];
    if l < m {
        if m < r {
            (m, mIndex)
        } else if r < l {
            (r, right - 1)
        } else {
            (l, left)
        }
    } else {
        if r < l {
            (l, left)
        } else if r < m {
            (r, right - 1)
        } else {
            (m, mIndex)
        }
    }
}

fn swap(array: &mut [int], a: uint, b: uint) {
    if a == b {
        return;
    }

    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}

fn partition(array: &mut [int], left: uint, right: uint) -> uint {
    // choose a pivot
    let (pivotValue, pivotIndex) = choose_pivot(array, left, right);
    // move everything less than this value to the left, and everything greater to the right
    // move the pivot somewhere safe
    swap(array, pivotIndex, right - 1);

    let mut storeIndex = left;
    
    for index in range(left, right - 1) {
        if array[index] < pivotValue {
            swap(array, storeIndex, index);
            storeIndex += 1;
        }
    }

    // move the pivot to storeIndex
    swap(array, storeIndex, right - 1);
    storeIndex
}

fn quicksort(array: &mut [int], left: uint, right: uint) {
    if left >= right {
        return;
    }
    let pivotIndex = partition(array, left, right);
    if pivotIndex > 0 {
        quicksort(array, left, pivotIndex - 1);
    }
    quicksort(array, pivotIndex + 1, right);
}

fn arr2str(a:&[int]) -> String {
    let mut s = String::from_str("[");
    for i in a.iter() {
        s = s + format!(" {}", *i);
    }
    return s.append(" ]");
}

fn main() {
    // make an array of 100 random integers
    let mut array = [0, ..100];
    for index in range(0u, 100) {
        array[index] = random::<int>() % 100;
    }
    println!("{}", arr2str(array));

    let len = array.len();

    quicksort(array, 0, len);
    println!("{}", arr2str(array));
}
