fn main() {
    println!("Hello, world!");
    // assert_eq!(quick_sort(vec![3,2,1,6,5,4,7]), vec![1,2,3,4,5,6,7]);
}

fn partition(array_to_sort: &mut [isize], low: usize, high: usize) -> usize {
    // let mid = (high + low)/2;
    let mid = high;
    let pivot = array_to_sort[mid];

    let mut j = low;

    for i in low..high {
        if array_to_sort[i] <= pivot {
            array_to_sort.swap(i, j);
            j += 1;
        }
    }

    array_to_sort.swap(j, mid);

    j
}

fn qs(array_to_sort: &mut [isize], low: usize, high: usize) {
    // We may want to use `std::cmp::Ord` here later 
    // (https://doc.rust-lang.org/std/cmp/trait.Ord.html)
    println!("qs before - l: {}, h: {}, {:?}", low, high, array_to_sort);
    if low < high {
        let mid = partition(array_to_sort, low, high);
        qs(array_to_sort, low, mid -1);
        qs(array_to_sort, mid + 1, high);
    }
    // println!("qs after: {:?}", array_to_sort);
}

pub fn quick_sort(array_to_sort: &mut [isize]) {
    // let pivot = array_to_sort.len()/2;
    let low = 0;
    let high = array_to_sort.len();
    let mut output = array_to_sort;
    qs(&mut output, high, low);

    // output.to_vec()
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
        let mut array = [1,2,3];
        let array_length = array.len();
        qs(&mut array, 0, array_length - 1);
        assert_eq!(array, [1,2,3]);
    }

    #[test]
    fn basic_sort_2() {
        let mut array = [3,2,1];
        let array_length = array.len();
        qs(&mut array, 0, array_length - 1);
        assert_eq!(array, [1,2,3]);
    }

    #[test]
    fn basic_sort_3() {
        let mut array = [16,9,4,6,12,3,8,7];
        let array_length = array.len();
        qs(&mut array, 0, array_length - 1);
        assert_eq!(array, [3,4,6,7,8,9,12,16]);
    }

    #[test]
    fn basic_sort_4() {
        let mut array = [3, 5, 1, 4, 2];
        let array_length = array.len();
        qs(&mut array, 0, array_length - 1);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn basic_sort_5() {
        let mut array = [3, 1, 4, 2];
        let array_length = array.len();
        qs(&mut array, 0, array_length - 1);
        assert_eq!(array, [1, 2, 3, 4]);
    }

    #[test]
    fn basic_sort_6() {
        let mut array = [10, 7, 8, 9, 1, 5];
        let array_length = array.len();
        qs(&mut array, 0, array_length - 1);
        assert_eq!(array, [1, 5, 7, 8, 9, 10]);
    }
}
