use crate::Todo;
use console::Term;
use std::io::Result;

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        Terminal
    }

    pub fn input(&self) -> Result<String> {
        Term::stdout().read_line()
    }

    pub fn ask_for_new_todo(&self) -> Result<Todo> {
        println!("Qual será seu novo todo? 💬");
        let todo_name = self.input()?;
        Ok(Todo::new(todo_name))
    }

    pub fn ask_if_new_todo(&self) -> Result<bool> {
        println!("Você deseja criar um novo TODO? ⛏  (insira apenas n ou s)");
        let response: String = self.input()?;

        loop {
            match response.as_str() {
                "s" => break,
                "n" => panic!("O gerador de todo foi fechado como solicitado"),
                _ => {
                    println!("Resposta inválida. Insira apenas 's' para sim ou 'n' para não.");
                    self.ask_if_new_todo()?;
                    break;
                }
            }
        }
        Ok(true)
    }

    pub fn show_todo(&mut self, todo: &Todo) -> Result<()> {
        println!("Qual o seu todo?");
        writeln!(self.stdout, "Seu todo é: {}", todo.message)?;

        Ok(())
    }
}
