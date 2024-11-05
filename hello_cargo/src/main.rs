fn main() {
    // rust变量默认是不可变的
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // error: cannot assign twice to immutable variable `x`

    // 使用mut关键字定义可变变量
    let mut y = 4;
    println!("The value of y is: {}", y);
    y = 5;
    println!("The value of y+1 is: {}", y+1);

    // rust常量
    const MAX_POINTS: u32 = 100000; //常量必须注明类型，如u32；常量名约定大写，用下划线分隔单词；不能使用mut关键字
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
