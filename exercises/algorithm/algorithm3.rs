/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]){
	let n = array.len();
    for i in 1..n { adjust_up(array, i); }
    for i in (0..n).rev() {
        array.swap(0, i);
        adjust_down(array, i);
    }
}

fn adjust_up<T: PartialOrd>(array: &mut [T], mut index: usize){
    if index == 0 { return; }
    let mut root = (index - 1) >> 1;
    while index > 0 && array[index] > array[root] {
        array.swap(root, index);
        index = root;
        if root == 0 {break}
        root = (root - 1) >> 1;
    }
}

fn adjust_down<T: PartialOrd>(array: &mut [T], size: usize){
    if(size == 1) { return; }
    let mut root = 0;
    while root < size {
        let left = (root << 1) + 1;
        if left >= size {break;}
        let right = left + 1;
        let mut to_swap = root;
        if left < size && array[left] > array[to_swap] { to_swap = left; }
        if right < size && array[right] > array[to_swap] { to_swap = right; }
        if to_swap == root { return; }
        array.swap(to_swap, root);
        root = to_swap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}