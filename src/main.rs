/*
Option<T> enum:
This is how we handle for null references.

enum Option<T> {
    Some<T>,
    None
}
If it is the Some variant, that indicates it is not null and does have a value.
If it does not have a value, it will return None, which is basically null.

*/

fn main() {
    let countdown = [5, 4, 3, 2, 1];

    //the get method allows you to utilize the Option<T> enum.  If the value is out of range, it will return None.  If it is in range, it will return Some(number)
    let number: Option<&i32> = countdown.get(4);
    //Option<T> is its own datatype, so, if we used the unwrap(), and get() was access something out of range, it would return None
    //But, you would not be able to add None to 1.  So, when you know you need to handle for a None, use the unwrap_or()
    //and give a default value for it to return in the case that the return value is None:
    //let number = number.unwrap_or(&0) + 1;
    //Another way to do this is to use the match expression using the match operator:
    let number = match number {
        Some(number) => number,
        None => &0,
    };
    //Without literally typing in the unwrap_or(), this is literally how you implement it above ^.  I am just not adding 1 to the number like the last time.

    println!("number is {:?}", number);
}
