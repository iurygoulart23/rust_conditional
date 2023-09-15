use std::io;

fn authentication(test:&str) -> bool {
    if test == "iury_gulart@hotmail.com" {
        true
    } else {
        false
    }    
}

fn main() {
    println!("Entre com seu email!");
    let mut email_input = String::new();

    // pede para por o email
    io::stdin()
        .read_line(&mut email_input)
        .expect("Erro no email");

    println!("Seu email é {}? (Responda sim ou nao)", email_input);

    let mut email_confirma = String::new();

    io::stdin()
        .read_line(&mut email_confirma)
        .expect("Digite sim ou nao.");
    
    // remove espaços
    let email_confirma = email_confirma.trim();

    // caso confirme o email ele vai autenticar
    if email_confirma == "sim" {
        
        let variavel: &str = "iury@sp.senai.br";   
        let result = authentication(variavel);
        
        if result {
            println!("Autenticado");
        } else {
            println!("Não Autenticado");
        }

    }
    
}
