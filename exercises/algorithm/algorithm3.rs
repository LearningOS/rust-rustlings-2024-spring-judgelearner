/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: std::cmp::PartialOrd>(arr: &mut [T]){
	//TODO
    if arr.len() <= 1 {
        return;
    }
    let size = arr.len();
    for i in 0..(size - 1) {
        // 标记当前循环是否发生元素交换
        let mut swapped = false;
        // 最后i个元素已经排好了顺序
        for j in 1..(size - i) {
            if arr[j - 1] > arr[j] { 
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        // 如果当前循环没有发生元素交换，说明数组已经有序
        if !swapped {
            break;
        }
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