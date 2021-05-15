
//create function to have three values
// fn value_state(value1:i32, value2:i32, state:bool) -> bool {
//     //create the condition for the negative
//     if value1 < 0 && value2 < 0 && state == true {
//         let state2 = true;
//         state2
//     }else if (value1 < 0 || value2 < 0) && state == false {
//             let state3 = true;
//             state3
//         }else {
//         let state2 = false;
//         state2
//     }
// }

fn pos_neg(a:i32, b:i32, negative:bool) -> bool{
    if negative {
        (a<0) && (b<0)
    }else {
        (a>0) != (b>0)
    }
}

fn main() {
    //create value for value1 , value2 and state
    println!("{}", pos_neg(2, 3, false));
}
