use graph::input_module;

fn main() {
    let input = input_module::get_input();
    println!("input: {}", input);
}

#[cfg(test)]
mod test;
