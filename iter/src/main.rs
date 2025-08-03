fn print_elements(elements: &Vec<String>) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements.iter().for_each(|element| println!("{}", element));
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("yellow"),
        String::from("blue"),
    ];

    print_elements(&colors);
}
