// vector slice is preferable than vector reference
fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements.iter().for_each(|element| println!("{}", element));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|element| {element.truncate(1)});
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter()
        .map(|element| element.to_uppercase())
        .collect()  // collect use type annotation (returned, variable) or type argument (Turbofish) to determine the collected type
    // collect::<Vec<String>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|element| vec_b.push(element))
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("yellow"),
        String::from("blue"),
    ];

    // print_elements(&colors);
    // shorten_strings(&mut colors);
    // println!("{:#?}", colors);

    // let uppercase_colors = to_uppercase(&colors);
    // println!("{:#?}", uppercase_colors);

    let mut destination = vec![];
    move_elements(colors, &mut destination);
    println!("{:#?}", destination);
}
