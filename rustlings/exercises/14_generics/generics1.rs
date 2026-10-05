// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.

    // NOTE TO SELF - I ended up having to look this up because I was expecting
    // to be able to find a 'generic' number type (or some other way to specify 
    // that I want "A Vev a numbers, generally"), but that is not the case. It 
    // seems like 'generic' numbers are just "the next size up" (
    // ie for x8 -> i16, x16 -> i32, etc).
    let mut numbers: Vec<i16> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
