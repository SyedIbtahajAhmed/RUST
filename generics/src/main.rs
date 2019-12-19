use std::cmp::PartialOrd;
use std::ops::Add;
use std::ops::Div;
use std::ops::Rem;
use std::ops::AddAssign;
//Average Calculator Using Generic Function
fn average_g <T> (x: &[T]) -> T
where T: Add<Output=T> + Div<Output=T> + Copy + Clone + PartialOrd + From<i32>
{
    let count = T::from(x.len() as i32);
    let mut sum = x[0];
    for i in 0..x.len(){
        sum = sum + x[i];
    };
    sum/count
}

//Even Numbers Detectors
fn even_g <T, U> (list: &[T], num: &U) -> T
where T: Div<Output=T> + Rem<Output=T> + Copy + Clone + PartialOrd + From<T> + From<U>
{
    let mut my_vec = T::from(vec![] as T);
    for x in 0..list.len(){
        if list[x] % num == 0 {
            my_vec[x].push(list[x]);
        }
    }
    my_vec
}

fn main() {

    let my_arr: Vec<i32>= vec![3,5,7,9,12,45,56];
    println!("{}", average_g(&my_arr));
    println!("{}", even_g(&my_arr, &2));
}