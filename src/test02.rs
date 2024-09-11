fn main()
{
    // 数据类型

    // 整型
    let a = 98_222_333_1; // 982223331，类似于1,222,333,444的表示方法
    println!("{}", a);
    // 十六进制：0xaa，八进制：0o23。二进制：0b1111_0000，字节（只能表示一个字节）：b'A'
    // arch x86 ? x64

    // 浮点型 f32 | f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32指定类型

    // 操作与其他语言基本一致
    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余

    // 布尔型 true | false

    // 字符型 (char) 4个字节 Unicode编码

    // 复合类型
    // 元组()
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    let (x, y, z) = tup;
    // y 等于 6.4

    // 数组，表示同类型的数据（区别于Python）
    let a = [1, 2, 3, 4, 5]; // a 是一个长度为 5 的整型数组
    let b = ["January", "February", "March"]; // b 是一个长度为 3 的字符串数组
    let c: [i32; 5] = [1, 2, 3, 4, 5]; // c 是一个长度为 5 的 i32 数组
    let d = [3; 5]; // 等同于 let d = [3, 3, 3, 3, 3];

    // 数组访问
    let first = a[0];
    let second = a[1];

    // a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确

    return;
}