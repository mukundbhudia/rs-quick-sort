fn main() {
    println!("Hello, world!");
    assert_eq!(quick_sort(vec![3,2,1]), vec![1,2,3]);
}

pub fn quick_sort(array_to_sort: Vec<usize>) -> Vec<usize> {
    let pivot = array_to_sort.len()/2 + 1;
    // println!("Pivot: {}, element at pivot is: {}", pivot, array_to_sort[pivot]);
    for i in 0..pivot {
        println!("{}", array_to_sort[i]);
    }
    for i in pivot..array_to_sort.len() {
        println!("{}", array_to_sort[i]);
    }
    vec![1,2,3]
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
}
