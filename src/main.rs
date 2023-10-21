use std::io::{Stdin, Stdout, Write};

fn main() {
    let mut terminal = Terminal::new();
    loop {
        terminal.ask_if_new_todo();
        let new_todo = terminal.ask_for_new_todo();
        terminal.show_todo(&new_todo);
    }
}

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn input(&self) -> String {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }

    fn ask_for_new_todo(&self) -> Todo {
        println!("Qual será seu novo todo? 💬");
        let todo_name = self.input();
        Todo::new(todo_name)
    }
    fn ask_if_new_todo(&self) -> bool {
        println!("Você deseja criar um novo TODO? ⛏  (insira apenas n ou s)");
        let response: String = self.input();

        loop {
            match response.as_str() {
                "s" => break,
                "n" => {
                    println!("Tchau brigado 🙃");
                    std::process::exit(0);
                },
                _ => {
                    println!("Resposta inválida. Insira apenas 's' para sim ou 'n' para não.");
                    self.ask_if_new_todo();
                    break;
                }
            }
        }
        true
    }
    fn show_todo(&mut self, todo: &Todo) {
        println!("Qual o seu todo?");
        writeln!(self.stdout, "Seu todo é: {}", todo.message).unwrap();
    }
}

impl Todo {
    const fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}
