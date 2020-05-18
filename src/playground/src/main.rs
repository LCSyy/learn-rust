/*
size, alignment

所有类型的数据都会按照字节对齐(alignment)。

一个类型的对齐方式决定了哪些地址能合法地存储该类型的值。

一个按n字节对齐的值，必须存储在n的倍数的地址上。

对齐的值至少是1，且总是2的指数。

基础类型的对齐和类型大小一致。具体大小是和平台相关的。

类型的大小是对齐的倍数。

对于动态尺寸类型，静态时期无法知道其大小和对齐。

Rust提供了以下几种方式来布局复合数据：
- 结构
- 元组
- 数组
- 枚举
- 联合

一般结构的对齐等于其最大的字段的对齐值。
Rust会尝试插入空白字节，保证最终结构的大小时期对齐的倍数。

repr(c) 采用C语言的布局方式

*/

use std::mem;

// alignment: size_of(u32)
// size: size_of(u8) + size_of(u8[3]) + size_of(u32)
struct Of {
    one: u8,
    two: u32,
    three: u16,
}

trait Object {
    fn object_name(&self) -> String;
}

trait Widget: Object {
    fn size(&self) -> (u32,u32);
}


fn main() {
    println!("{}", mem::size_of::<Of>());
}
