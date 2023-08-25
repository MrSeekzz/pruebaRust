const MAX_BAR_HEIGHT: u32 = 10;

struct MergeSortStackItem {
    left: usize,
    right: usize,
    level: usize,
}

fn merge_sort_with_print(elements: &mut [u32]) {
    let mut stack: Vec<MergeSortStackItem> = Vec::new();
    let mut level = 0;
    let mut left = 0;
    let mut right = elements.len();

    stack.push(MergeSortStackItem { left, right, level });

    while let Some(item) = stack.pop() {
        left = item.left;
        right = item.right;
        level = item.level;

        if right - left <= 1 {
            continue;
        }

        let mid = (left + right) / 2;

        stack.push(MergeSortStackItem {
            left: mid,
            right,
            level: level + 1,
        });

        stack.push(MergeSortStackItem {
            left,
            right: mid,
            level: level + 1,
        });
    }

    // Imprimir el estado del array después de cada llamada recursiva
    print_elements(elements);
}

fn print_elements(elements: &[u32]) {
    let max_value = *elements.iter().max().unwrap();
    let scale_factor = MAX_BAR_HEIGHT as f32 / max_value as f32;

    for &element in elements {
        let bar_height = (element as f32 * scale_factor) as u32;
        let bar = "#".repeat(bar_height as usize);
        println!("{}", bar);
    }
    println!("---");
}

fn main() {
    let mut elements = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    assert_eq!("a".is_empty(), false);
    println!("Estado inicial:");
    print_elements(&elements);

    merge_sort_with_print(&mut elements);

    println!("Estado final:");
    print_elements(&elements);
}

//fn main() {
//
//    println!("{:?}",smallest(209917));
//}
//
//fn smallest(n: i64) -> (i64, usize, usize) {
//
//    let mut smallest_number: usize;
//    let mut position: usize;
//    let mut position_new_smaller: usize = 999;
//    let mut zero_case = false;
//
//    let mut string_n = n.to_string();
//    smallest_number = string_n.chars().min().unwrap().to_digit(10).unwrap() as usize;
//    if smallest_number == 0{
//        position = string_n.rfind(char::from_digit(smallest_number as u32, 10).unwrap()).unwrap();
//    }else{
//        position = string_n.chars().position(|x| x == smallest_number.to_string().chars().min().unwrap()).unwrap();
//    }
//
//    let primer_num = string_n.chars().next().unwrap();
//
//    if primer_num == '9' {
//        position = 0;
//        smallest_number = 9;
//    }
//
//    if position == 0 && primer_num != '0' && primer_num != '9'{
//        string_n = String::from(&string_n[1..]);
//        smallest_number = string_n.chars().min().unwrap().to_digit(10).unwrap() as usize;
//        position = string_n.chars().position(|x| x == smallest_number.to_string().chars().min().unwrap()).unwrap() + 1;
//    }
//    if smallest_number == 0 &&
//       1 == string_n.chars().position(|x| x == smallest_number.to_string().chars().min().unwrap()).unwrap(){
//
//        position = 0;
//        smallest_number = string_n[..1].to_string().chars().min().unwrap().to_digit(10).unwrap() as usize;
//        zero_case = true;
//    }
//
//    let mut number: String = "".to_owned();
//    let mut found = false;
//
//    for num in n.to_string().char_indices(){
//        let num_to_digit = num.1.clone().to_digit(10).unwrap();
//        //if num_to_digi
//        if !found{
//            if (smallest_number as u32) < num_to_digit {
//                number.push_str(&smallest_number.to_string());
//                found = true;
//                position_new_smaller = num.0.clone();
//                if zero_case{
//                    number.push_str(&num_to_digit.to_string());
//                    if num.0 > 0{
//                        position_new_smaller = num.0.clone() - 1;
//                    }else{
//                        position_new_smaller = num.0.clone();
//                    }
//
//                }
//                zero_case = false;
//
//            }
//        }
//
//
//        if !zero_case{
//            number.push_str(&num_to_digit.to_string());
//        }
//    }
//
//    if smallest_number == 9{
//        number.push_str(&smallest_number.to_string());
//        number.remove(position);
//        position_new_smaller = number.len() - 1;
//    }else{
//        number.remove(position + 1);
//    }
//
//
//
//    return (number.parse::<i64>().unwrap(), position, position_new_smaller);
//}
