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

use algorithms::{
    selection_sort,
    bubble_sort,
    insertion_sort
};

fn main() {
    println!("seletion:");
    let mut a = [123,5,785,23,563542,1,54,47];
    selection_sort(&mut a);
    for n in a.iter() {
        println!("{}",n);
    }

    println!("bubble sort:");
    let mut b = [4,5,7825,3,35,3251,54,47,35,567];
    bubble_sort(&mut b);
    for n in b.iter() {
        println!("{}",n);
    }

    println!("insertion sort:");
    let mut c = [235,36,373452,232,2535,3632,23425];
    insertion_sort(&mut c);
    for n in c.iter() {
        println!("{}", n);
    }
    
}
