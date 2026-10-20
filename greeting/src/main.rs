use std::io;

fn main() {
    println!("Hello, world!");
    println!("Digite seu nome: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Erro ao ler.");
    let nome = nome.trim();
    println!("Seja bem vindo {nome}.");
}
