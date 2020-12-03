//! Algorithms Using Rust
//! 
//! ## 排序
//! 
//! - 选择
//! - 冒泡
//! - 插入
//! - 希尔
//! - 归并
//! - 快速
//! - 堆
//! - 基数
//! 
//! ## 查找
//! 
//! - 顺序
//! - 二分
//! - ...
//! 
//! ## 递归与迭代
//! 

/// 选择排序
/// 
/// 从待排序的序列中选择最小的元素，将它放到已排序序列的末尾，直到未排序的元素个数为零。
pub fn selection_sort<T: PartialOrd>(a: &mut [T]) {
    for i in 0..a.len() {
        for j in (i+1)..a.len() {
            if a[i] > a[j] {
                a.swap(i,j);
            }
        }
    }
}

/// 冒泡排序
/// 
/// 从无序序列末尾开始，比较相邻元素，大的在前，小的在后时，则交换位置；每次遍历都会找到最小的元素，且被交换到最前面位置，重复比较，则到序列中没有元素需要比较。
pub fn bubble_sort<T: PartialOrd>(a: &mut [T]) {
    for i in 0..a.len() {
        for j in 0..a.len() - 1 - i {
            if a[j] > a[j+1] {
                a.swap(j,j+1);
            }
        }
    }
}

/// 插入排序
/// 
/// 将一个元素插入到有序表中适当的位置。
pub fn insertion_sort<T: PartialOrd>(a: &mut [T]) {
    if a.len() < 1 { return; }

    for i in 1..a.len() {
        if a[i-1] > a[i] {
            let mut j = i;
            while j > 0 && a[j-1] > a[j] {
                a.swap(j-1,j);
                j -= 1;
            }
        }
    }
}

/// 希尔排序/缩小增量排序
/// 
/// 按增量进行分组，对每组分别进行插入排序，直到分组只有一组。
pub fn shell_sort<T:PartialOrd>(a: &mut [T]) {
    let mut inc = a.len() / 2;
    while inc >= 1 {
        for g in 0..inc {
            let mut i = g + inc;
            while i < a.len() {
                if a[i-inc] > a[i] {
                    let mut j = i;
                    while j > g && a[j-inc] > a[j] {
                        a.swap(j-inc, j);
                        j -= inc;
                    }
                }
                i += inc;
            }
        }
        inc /= 2;
    }
}

/// 归并排序
/// 
/// 将有序子序列合并为一个有序序列。
pub fn merge_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    let mut pairs: Vec<(usize,usize)> = Vec::new();
    pairs.push((0,a.len()));

    // 分解
    // [b,e)
    let mut cur_idx: usize = 0;
    while cur_idx != pairs.len() {
        let (b,e) = pairs[cur_idx];
        if b != e && b + 1 < e {
            let mid = (b + e) / 2;
            pairs.push((b,mid));
            pairs.push((mid,e));
        }
        cur_idx += 1;
    }

    while pairs.len() >= 2 {
        let (right_begin, right_end) = pairs.pop().unwrap();
        let (left_begin, left_end) = pairs.pop().unwrap();
        let mut left_idx = left_begin;
        let mut right_idx = right_begin;

        let mut tmp_vec = Vec::new();

        while left_idx < left_end || right_idx < right_end {
            if left_idx >= left_end {
                tmp_vec.push(a[right_idx]);
                right_idx += 1;
            } else if right_idx >= right_end {
                tmp_vec.push(a[left_idx]);
                left_idx += 1;
            } else {
                if a[left_idx] <= a[right_idx] {
                    tmp_vec.push(a[left_idx]);
                    left_idx += 1;
                } else {
                    tmp_vec.push(a[right_idx]);
                    right_idx += 1;
                }
            }
        }

        for i in left_begin..right_end {
            a[i] = tmp_vec[i - left_begin];
        }
    }
}

/// 快速排序
/// 
/// 根据一个基准值，将小于或等于该值的元素放到左侧，大于该值的元素放到右边。对左右两侧子序列继续采用上述步骤，直至子序列只有一个元素，排序完成。
pub fn qsort<T: PartialOrd>(a: &mut [T]) {
    // 基准值就取序列中第一个元素
    let mut base_index = 0;
    let mut range_begin = base_index + 1;

    let mut end_index = a.len() - 1;
    let range_end = a.len();

    while range_begin <= end_index {
        if a[base_index] >= a[range_begin] {
            // 如果当前元素小于或等于基准值，两个元素交换位置即可
            a.swap(base_index, range_begin);
            base_index = range_begin;
            range_begin += 1;
        } else {
            // 如果当前元素大于基准值，则将当前元素和还未比较的最后一个元素交换位置。
            a.swap(range_begin, end_index);
            end_index -= 1;
        }
    }

    if 0 < base_index {
        qsort(&mut a[0..base_index]);
    }

    let base_index = base_index + 1;
    if base_index < range_end {
        qsort(&mut a[base_index..range_end]);
    }
}
