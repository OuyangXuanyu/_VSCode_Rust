// 迭代器（数据遍历方法，来访问集合中的每个元素，不需显式的管理索引或循环）
// 惰性求值，消费性，不可变访问，所有权

fn main(){
    // 创建迭代器
    // iter()创建借用迭代器，不可变引用，只读迭代
    let vec = vec![1, -2, 3, -4, 5];
    let iter_ = vec.iter();

    // iter_mut()创建可变借用迭代器，迭代过程修改元素
    let mut vec_mut = vec![1, 2, 3, 4, 5];
    let iter_mut_ = vec_mut.iter_mut();

    // into_iter()创建获取所有权的迭代器，转移元素的所有权，迭代后原始容器变空
    let into_vec = vec![1, 2, 3, 4, 5];
    let into_iter_ = into_vec.into_iter();

    // 方法：

    // map：对元素应用转换函数
    // let new_iterator = old_iterator.map(|element| some_function_or_operation(element));
    // 在|element|中，|x|为值拷贝，|&x|为不可变借用（类似C的指针）；(? WRONG)对于可变迭代器mut_iter()，可能使用|&mut x|进行可变借用
    // 两种方式，i32可以省略为_，自动判断
    let squared_vec: Vec<i32> = vec.iter().map(|x| x * x * x).collect();
    let squared_vec_2 = vec.iter().map(|x| x * x * x).collect::<Vec<i32>>();
    for i in 0..5{
        print!("{} ", squared_vec[i])
    }
    println!("");
    println!("{:?}", squared_vec_2); // 用于输出整个list

    // filter：过滤元素
    let filtered_vec: Vec<i32> = vec.into_iter().filter(|&x| x % 2 == 0).collect();
    for i in 0..filtered_vec.len(){
        print!("{} ", squared_vec[i])
    }
    return;
}
