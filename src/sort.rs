fn _partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let mut i = low - 1;
    for j in low..high {
        if arr[j as usize] > arr[high as usize] {
            continue;
        }
        i += 1;
        arr.swap(i as usize, j as usize);
    }
    i += 1;
    arr.swap(i as usize, high as usize);
    i
}

fn _quick_sort<T: Ord>(arr: &mut [T], mut low: isize, high: isize) {
    while low < high {
        let pivot_index = _partition(arr, low, high);
        _quick_sort(arr, low, pivot_index - 1);
        low = pivot_index + 1;
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    _quick_sort(arr, 0, arr.len() as isize - 1)
}
