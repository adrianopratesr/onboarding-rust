fn main() {
    loop {
        if generator_todo() {
            break;
        }
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn generator_todo() -> bool {
    println!("Você deseja criar um novo TODO? (insira apenas n ou s)");

    let response = input();

    if response == "n" {
        println!("Tchau brigado");
        true
    } else if response == "s" {
        println!("Qual o seu todo?");
        let todo = input();
        println!("Seu TODO é: {}", todo);
        false
    } else {
        println!("Insira apenas 's' para sim e 'n' para não");
        false
    }
}
