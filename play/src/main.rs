//! 算法
//! 排序
//! - 选择
//! - 冒泡
//! - 插入
//! - 希尔
//! - 快速
//! - 归并
//! - 堆
//! - 基数
//! 查找
//! - 顺序
//! - 二分
//! - ...

fn main() {
    println!("seletion:");
    let mut a = [123,5,785,23,563542,1,54,47];
    algo::selection(&mut a);
    for n in a.iter() {
        println!("{}",n);
    }

    println!("bubble sort:");
    let mut b = [4,5,7825,3,35,3251,54,47,35,567];
    algo::bubble_sort(&mut b);
    for n in b.iter() {
        println!("{}",n);
    }

}

mod algo {

    /// 选择排序 selection sort
    /// 思想：从待排序的序列中选择最小的元素，将它放到已排序序列的末尾，知道未排序的元素个数为零。
    pub fn selection<T: PartialOrd>(a: &mut [T]) {
        for i in 0..a.len() {
            for j in (i+1)..a.len() {
                if a[i] > a[j] {
                    a.swap(i,j);
                }
            }
        }
    }

    /// 冒泡排序 bubble sort
    /// 思想是：比较相邻元素，大的在前，则交换位置；每次遍历都会找到最大的元素，重复比较，则到序列中没有元素需要比较。
    pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
        for i in 0..arr.len() {
            for j in 0..arr.len() - 1 - i {
                if arr[j] > arr[j+1] {
                    arr.swap(j,j+1);
                }
            }
        }
    }

}
