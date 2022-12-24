use std::cmp::Ordering;
use std::collections::HashMap;

// My first real Rust code, not super happy with it
fn main() {
    // 19, 19, 32, 37, 38, 50, 102
    // median: 37
    // mode: 19
    let mut median: Option<f32> = None;
    let mut mode: i32;
    let mut v = vec![50, 38, 32, 37, 19, 19, 102];

    vec_sort(&mut v);

    println!("Arr: {:?}", v);
    println!("Array size: {}", v.len());

    if v.len() % 2 == 0 {
        println!("Array even")
    } else {
        unsafe {
            let midpoint = v.len() as f32;
            let real_midpoint = midpoint / 2.0;
            median = Some(v[real_midpoint.floor() as usize] as f32);
        }
    }

    println!("Median: {}", median.unwrap_or(-1.0));
    let mut modemap = HashMap::new();
    for i in &v {
        let count = modemap.entry(i).or_insert(0);
        *count += 1;
    }

    let mut largestValue = &0;
    let mut largestKey = None;
    for (k, v) in &modemap {
        if v > largestValue {
            largestKey = Some(k);
            largestValue = v;
        }
    }

    println!("HashMap: {:?}", modemap);
    println!("Mode is {} with it occurring {} times", largestKey.unwrap(), largestValue);
}

fn vec_sort(vecin: &mut Vec<i32>) -> &mut Vec<i32> {
    // if vecin.len() == 0 {
    //     vecin;
    // }

    // println!("Len {}", vecin.len());

    // let mut last_int = &vecin[0];
    let mut sorted = false;
    let mut index = 0;
    let mut found_greater = false;

    while !sorted {
        let v0 = *vecin.get(index).unwrap(); // This will always contain a value
        let v1temp = vecin.get(index + 1);
        let v1 = match v1temp {
            Some(v1) => *v1,
            None => { // If at the end of the Vector
                if !found_greater {
                    sorted = true;
                }

                index = 0;
                found_greater = false;
                continue;
            }
        };

        match (v0).cmp(&v1) {
            Ordering::Less => println!("Less Than"),
            Ordering::Equal => println!("Equal"),
            Ordering::Greater => {
                found_greater = true;
                println!("Greater than");
                vecin.swap(index, index + 1);
            }
        };

        index = index + 1;

        println!("Arr: {:?}", vecin);

        println!("V0: {}, V1: {}", v0, v1);
        // println!("Not sorted");
    }

    // for i in &mut vecin {
    //     if i > last_int
    // }

    vecin
}