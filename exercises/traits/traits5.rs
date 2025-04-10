// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.



/*
pub 关键字：
表示这个特质是公开的，可以在其他模块中访问。
trait SomeTrait：
定义了一个名为 SomeTrait 的特质。特质可以包含方法签名（即函数定义，但没有具体实现），这些签名必须由实现该特质的类型（如结构体或枚举）提供具体实现。
fn some_function(&self) -> bool { true }：
这是特质中的一个方法签名，定义了一个名为 some_function 的方法。
&self 表示这是一个实例方法，接受一个指向 self 的不可变引用。
-> bool 表示这个方法返回一个布尔值。
{ true } 是这个方法的默认实现。这意味着：
如果某个类型实现了 SomeTrait 特质，但没有显式地提供 some_function 的实现，那么它会自动继承这个默认实现。
如果某个类型需要自定义 some_function 的行为，它可以覆盖这个默认实现。
特质的作用
定义共享行为：特质允许你为不同的类型定义一组共同的行为。例如，SomeTrait 可能定义了一种“可检查某些功能”的行为，任何实现了 SomeTrait 的类型都必须遵循这一约定。
多态性：通过特质，你可以编写通用的代码，这些代码可以操作任何实现了特定特质的类型。例如，一个函数可以接受任何实现了 SomeTrait 的类型作为参数。
代码复用：特质中的默认方法实现允许你在定义通用行为的同时，减少重复代码。
pub trait SomeTrait 定义了一个公开的特质，它包含了一个带有默认实现的方法 some_function。这个特质可以被任何类型实现，从而使这些类型具备 some_function 定义的行为。通过特质，Rust 实现了强大的多态性和代码复用能力。
*/
pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T:SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}
/*
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool 定义了一个泛型函数，
它利用特质约束来接受任何同时实现了 SomeTrait 和 OtherTrait 的类型。
这使得函数具有多态性、类型安全性和代码复用的优势。
*/
/*
泛型函数定义：
fn some_func<T: ...>：定义了一个泛型函数 some_func，它有一个类型参数 T。
<T: ...>：表示类型参数 T 需要满足一定的约束条件。
特质约束：
T: SomeTrait + OtherTrait：这是对类型参数 T 的约束，表示 T 必须同时实现 SomeTrait 和 OtherTrait 这两个特质。
+ 符号表示逻辑上的“与”关系，即 T 需要同时满足两个特质的实现。
函数参数：
(item: T)：函数接受一个参数 item，其类型为 T。由于 T 被约束为实现了 SomeTrait 和 OtherTrait，因此 item 可以调用这两个特质中定义的方法。
返回值：
-> bool：函数返回一个布尔值。
函数体：
item.some_function() && item.other_function()：函数体调用了 item 的 some_function 和 other_function 方法，并返回它们的逻辑与（&&）结果。
由于 T 被约束为实现了 SomeTrait 和 OtherTrait，因此可以安全地调用这两个方法。
*/

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
