fn main() {
    let org_arr = [1,2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];
    //let result  = compare_array(&org_arr, &sub_arr);
    /*if compare_array(&org_arr, &sub_arr) == false {
        println!("sub_array not include in org_arr");
    }*/

    
    compare_array(&org_arr, &sub_arr);

}

/*fn compare_array(org_arr : &[i32], sub_arr: &[i32]) {
    println!("Two array are: {:?} {:?}", org_arr, sub_arr);
    //let mut result = true;
    for i in sub_arr.iter() {
        let any_res = org_arr.iter().any(|v| v == i);
        println!("{:?}", any_res );
        if any_res == false {
            println!("Sub_array not include in org_arr");
            break;
        }
    }
    println!("Sub_array include in org_arr");
}*/


fn compare_array(org_arr : &[i32], sub_arr: &[i32]) {
    if org_arr.len() < sub_arr.len() {
        println!("Not include");
    }
    //println!("{:?}", org_arr.len() );
    for (i, item) in org_arr.iter().enumerate() {
        let last_index = org_arr.len() - sub_arr.len();
        if i < last_index {
            let slice_org_arr  =  &org_arr[i..i+sub_arr.len()];
            println!("{:?}", slice_org_arr );
            if slice_org_arr == sub_arr {
                println!("Sub_array includes in org_arr");
                break;
            }
        }

        println!("Sub_array not includes in org_arr");
        
    }

     

}

