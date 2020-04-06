fn main() {
    println!("Hello, world!");
    assert_eq!(quick_sort(vec![3,2,1,6,5,4,7]), vec![1,2,3,4,5,6,7]);
}

fn partition(array_to_sort: &Vec<usize>, high: usize, low: usize) -> usize {
    0
}

fn qs(array_to_sort: &Vec<usize>, high: usize, low: usize) {
    if low < high {
        qs(array_to_sort, high, low);
    }
}

pub fn quick_sort(array_to_sort: Vec<usize>) -> Vec<usize> {
    let pivot = array_to_sort.len()/2;
    let low = 0;
    let high = array_to_sort.len();
    let value_at_pivot = array_to_sort[pivot];

    partition(&array_to_sort, high, low);
    qs(&array_to_sort, low, pivot);
    qs(&array_to_sort, pivot + 1, high);

    array_to_sort
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
        assert_eq!(quick_sort(vec![1,2,3]), vec![1,2,3]);
    }

    #[test]
    fn basic_sort_2() {
        assert_eq!(quick_sort(vec![3,2,1]), vec![1,2,3]);
    }

    #[test]
    fn basic_sort_3() {
        assert_eq!(quick_sort(vec![16,9,4,6,12,3,8,7]), vec![3,4,6,7,8,9,12,16]);
    }
}
