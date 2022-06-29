fn is_sub_array(arr:&[i32], sub_arr:&[i32]) -> bool {
    let mut res:bool = false;
    for i in 0..arr.len() {
        if arr[i] == sub_arr[0] && arr.len() - i > sub_arr.len() {
            let mut k = i + 1;
            res = true;
            for j in 1..sub_arr.len() {
                if arr[k] != sub_arr[j] {
                    res = false;
                    break;
                }
                else {
                    k += 1;
                }
            }
            if res {
                break;
            }
        }
    }
    return res;
}

fn count_string_appear(str1:String, str2:String) -> usize {
    let str_slice: &str = &str2.to_lowercase();
    let res = str1.to_lowercase().matches(str_slice).count();
    return res;
}

fn main() {
    // ex1
    let org_arr = &[1, 2,3,5,6,8, 10, 11];
    let my_arr = &[6,8,10];
    println!("{}", is_sub_array(org_arr, my_arr));

    // ex2
    let main_str:String = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");
    let sub_str:String = String::from("is IS");
    println!("{}", count_string_appear(main_str, sub_str));
}
