use crate::Todo;
use console::Term;
use std::io::Result;

pub struct Terminal {
    term: Term,
}
impl Terminal {
    pub fn new() -> Self {
        Terminal {
            term: Term::stdout(),
        }
    }

    pub fn input(&self) -> Result<String> {
        self.term.read_line()
    }

    pub fn ask_for_new_todo(&self) -> Result<Todo> {
        println!("Qual será seu novo todo? 💬");
        let todo_name = self.input()?;
        Ok(Todo::new(todo_name))
    }

    pub fn ask_if_new_todo(&self) -> Result<bool> {


        loop {
            println!("Você deseja criar um novo TODO? ⛏  (insira apenas n ou s)");
            let response = self.input()?;
            match response.as_str() {
                "s" => break,
                "n" => {
                    println!("Saindo do gerador de todo");
                    std::process::exit(0);
                }
                _ => {
                    println!("Resposta inválida. Insira apenas 's' para sim ou 'n' para não.");
                    break;
                }
            }
        }
        Ok(true)
    }

    pub fn show_todo(&self, todo: &Todo) -> Result<()> {
        println!("Qual o seu todo?");
        self.term
            .write_line(&format!("Seu todo é: {}", todo.message))?;

        Ok(())
    }
}
