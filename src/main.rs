fn main() {
    println!("Hello, world!");
    assert_eq!(quick_sort(&mut vec![3,2,1,6,5,4,7]), vec![1,2,3,4,5,6,7]);
}

fn partition(array_to_sort: &mut Vec<usize>, low: usize, high: usize) -> usize {
    let pivot = array_to_sort[high];

    let mut i = low;
    let mut j = low;

    while j < high - 1 {
        if array_to_sort[j] < pivot {
            array_to_sort.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    if array_to_sort[high] < array_to_sort[i] {
        array_to_sort.swap(i, high);
    }

    i
}

fn qs(array_to_sort: &mut Vec<usize>, low: usize, high: usize) {
    // We may want to use `std::cmp::Ord` here later (https://doc.rust-lang.org/std/cmp/trait.Ord.html)
    if low < high {
        let mid = partition(array_to_sort, low, high);
        qs(array_to_sort, low, mid);
        qs(array_to_sort, mid + 1, high);
    }
}

pub fn quick_sort(array_to_sort: &mut Vec<usize>) -> Vec<usize> {
    let mut output = array_to_sort;
    let pivot = output.len()/2;
    let low = 0;
    let high = output.len();
    // let value_at_pivot = array_to_sort[pivot];

    partition(output, high, low);
    qs(output, low, pivot);
    qs(output, pivot + 1, high);

    output.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_arrays_are_same() {
        assert_eq!(vec![1,2,3], vec![1,2,3]);
    }

    #[test]
    fn basic_sort_1() {
        assert_eq!(quick_sort(&mut vec![1,2,3]), vec![1,2,3]);
    }

    #[test]
    fn basic_sort_2() {
        assert_eq!(quick_sort(&mut vec![3,2,1]), vec![1,2,3]);
    }

    #[test]
    fn basic_sort_3() {
        assert_eq!(quick_sort(&mut vec![16,9,4,6,12,3,8,7]), vec![3,4,6,7,8,9,12,16]);
    }
}
