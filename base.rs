let 绑定
1.在Rust中通过 let 来绑定标识符的值
2.Rust是静态类型语言，需要先确定我们需要的类型，在Rust中有类型推断的功能，能确定这是什么类型，不需要明确的指出来。
3.值得绑定默认是不可变的，需要通过mut来绑定标识符来确定值是可变的。
4.在Rust中标识符必须要初始化，Rust 是不会让我们使用一个没有经过初始化的值的。
  let x = 5;  //不可变
  let mut x = 5;  //可变
当然也可以明确的指出来，只需要在将类型卸载（:）后面。
  let x:i32 = 5;
//END

函数
1.在Rust中使用 fn 来声明函数。
2.函数的参数必须声明类型。
3.使用 -> 符号来声明函数的返回值，且函数的返回值只能有一个。
4.在下面函数中 x + y 当作函数的返回值，注意 x + y 后面并没有分号（；），如果加有分号，则是一个表达式语句，不能作为一个表达式当作函数的返回值。
  fn print(x:i32, y: i32) -> i32{
      x + y
  }
//END

语句与表达式
1.变量声明语句：主要是指 let 语句。
2.ltem语句：是指函数（function）、结构体（structure）、类型别名（type）、静态变量（static）、特质（trait）、实现（implementation）或模块（module）的声明。这些声明可以嵌套在任意块（block）中。
3.表达式语句：由一个表达式和一个分号组成，即在表达式后面加一个分号就将一个表达式转变为了一个语句。所以，有多少种表达式，就有多少种表达式语句。
4.语句始终返回一个()，这是一个 unit 类型。
//END

函数指针
  fn plus_one(i: i32) -> i32 { i + 1 }
  let f = plus_one;
  let six = f(5);
//END

元组：

1.元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    2.可以使用 `let` 来绑定元组的各个变量
    let (integer, boolean) = pair;
    (boolean, integer)
}
3.包含各种不同类型的元组
let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);

4.通过元组的索引来访问具体的值
println!("long tuple first value: {}", long_tuple.0);
println!("long tuple second value: {}", long_tuple.1);

5.元组也可以充当元组的元素
let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

6.元组可以打印
println!("tuple of tuples: {:?}", tuple_of_tuples);

let pair = (1, true);
println!("pair is {:?}", pair);

println!("the reversed pair is {:?}", reverse(pair));

7.创建单元素元组需要一个额外的逗号，这是为了和括号包含的普通数据作区分。
println!("one element tuple: {:?}", (5u32,));
println!("just an integer: {:?}", (5u32));

8.解构元组，将值赋给创建的绑定变量
let tuple = (1, "hello", 4.5, true);

let (a, b, c, d) = tuple;
println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
//END

数组：

1.固定大小的数组（类型标记是多余的）
let xs: [i32; 5] = [1, 2, 3, 4, 5];

2.所有元素可以初始化成相同的值
let ys: [i32; 500] = [0; 500];

3.索引从 0 开始
println!("first element of the array: {}", xs[0]);
println!("second element of the array: {}", xs[1]);

4.`len` 返回数组的大小
println!("array size: {}", xs.len());

5.数组是在堆中分配
println!("array occupies {} bytes", mem::size_of_val(&xs));

6.数组可以自动地借用成为 slice
println!("borrow the whole array as a slice");
analyze_slice(&xs);

7.slice 可以指向数组的一部分
println!("borrow a section of the array as a slice");
analyze_slice(&ys[1 .. 4]);

8.越界的索引会引发 panic（中文意思是：惊恐、恐慌等意）
println!("{}", xs[5]);
//END


所有权（Ownership）
    1.Rust中的绑定 let 有一个属性：它们有它们所绑定的值的所有权。
    2.Rust 确保了对于任何给定的资源都正好（只）有一个绑定与之对应。

一开始绑定到 v ， v 得到这个向量的所有权
let v = vec![1, 2, 3];

将所有权转移给 v2
let v2 = v;

因为将所有权转移到了 v2 则 v 失去了对向量的所有权，当再次调用 v 的时候，向量跟随着 v2 一起被释放。
println!("v[0] is: {}", v[0]);
//END

借用和引用
    1.&T类型为一个”引用“，而与其拥有这个资源，它借用了所有权。一个借用变量的绑定在它离开作用域时并不释放资源。
    2.&T类型是不可变的，就像绑定一样。
    3.&mut T：一个“可变引用”允许你改变你借用的资源。
    4.任何借用必须位于比拥有者更小的作用域。
    5.对于同一个资源（resource）的借用，以下情况不能同时出现在同一个作用域下：
        1.1 个或多个不可变引用（&T）
        2.唯一 1 个可变引用（&mut T）

    fn foo(v: &Vec<i32>) {
        v.push(5);  //&T引用不可改变
    }

    let v = vec![];

    foo(&v);    //出现错误

    let mut x = 5;
    {
        //使用可变引用：将所有权暂时交给 y
        let y = &mut x;
        //y是一个&mut引用。需要使用他们（*）来访问引用的内容。
        *y += 1;
    }
    //退出作用域后，x 的值不会被释放
    println!("{}",x);

    6.引用必须与它引用的值存活得一样长。Rust 会检查你的引用的作用域来保证这是正确的。
    let y: &i32;
    {
        //定义一个 x 绑定
        let x = 5; 
        // y 引用 x 的值 
        y = &x;
    }
    //当离开当前作用域后，x 与 它的值会被释放

    //应为 x 与值一起被释放了，y没有可引用的值，所以会报错
    println!("{}", y);
//END

生命周期
&mut i32和&'a mut i32，他们是一样的，只是后者在&和mut i32之间夹了一个'a生命周期。
&mut i32读作“一个i32的可变引用”，而&'a mut i32读作“一个带有生命周期'a的i32的可变引用”。
//END



结构体（struct）

// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
//END

枚举（enum）：enum 关键字允许创建一个代表数个可能变量的数据的类型
// 隐藏未使用代码警告的属性。
#![allow(dead_code)]

// 创建一个 `enum` （枚举）来划分人的类别。注意命名和类型的信息是如何一起
// 明确规定变量的：
// `Engineer != Scientist` 和 `Height(i32) != Weight(i32)`。每者都不相同且
// 相互独立。
enum Person {
    // 一个 `enum` 可能是个 `unit-like`（类单元结构体），
    Engineer,
    Scientist,
    // 或像一个元组结构体，
    Height(i32),
    Weight(i32),
    // 或像一个普通的结构体。
    Info { name: String, height: i32 }
}

// 此函数将一个 `Person` enum 作为参数，无返回值。
fn inspect(p: Person) {
    // `enum` 的使用必须覆盖所有情形（无可辩驳的），所以使用 `match`
    // 以分支方式覆盖所有类型。
    match p {
        Person::Engineer    => println!("Is engineer!"),
        Person::Scientist       => println!("Is scientist!"),
        // 从 `enum` 内部解构 `i`
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // 将 `Info` 解构成 `name` 和 `height`。
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person   = Person::Height(18);
    let amira    = Person::Weight(10);
    // `to_owned()` 从一个字符串 slice 创建一个具有所有权的 `String`。
    let dave     = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca  = Person::Scientist;
    let rohan    = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
//END


let x = 1;

模式（match）
match x {
    //使用 | 匹配多个模式
    1 | 2 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),、
    //_ 作为“任何类型”
    _ => println!("anything"),
}

struct Point {
    x: i32,
    y: i32,
}
//END

let origin = Point { x: 0, y: 0};

match origin {
    //模式可以用来结构一个复合数据类型
    //分别输出 Point 结构中的 x 与 y 的值
    //也可以给 Point 结构中的 x 与 y 给出一个不同的标识符
    Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
}
//END

struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 2, y: 3 };

match point {
    //如果只关系一部分的值，则可以使用..来忽略其他的标识符
    Point { y, .. } => println!("x is {}", y),
}
//END

let x = 5;

match x {
    //使用引用：ref  可变引用使用：ref mut
    ref r => println!("Got a reference to {}", r),
}
//END

let x = 1;

match x {
    //使用 ... 匹配一个范围的值
    //使用 @ 把值绑定到一个标识符上
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}
//END
