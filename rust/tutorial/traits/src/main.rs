/* 
Copy is a trait shared between several data types
By using Copy like this, we allow all those data types as a potential argument
PartialOrd is a trait for comparision
So as far as I can tell, the <T: PartialOrd + Copy> puts a requirement on the arguments used
with this function to have both traits. 
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![12, 54, 15, 75, 45];

    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    let char_list = vec!['y', 'i', 'g', 'm', 'q'];

    let result = largest(&char_list);
    println!("The largest char is: {}", result);
}
