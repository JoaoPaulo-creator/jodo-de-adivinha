use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adividinhe um número");

    //atribuindo um número inteiro aleatório a variável numero_secreto
    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    //criando um loop
    loop{
        println!("Digite seu número: ");
        //O valor atribuído a variável guess é uma string
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler input");

        //realizando o parse da varíavel guess, para u32 (32 bits)
        //assim é possível fazer a comparação do input do usuário com o número secreto        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

    

        println!("Seu palpite: {guess}");

        //verificando se o número secreto é igual ao palpite do usuário
        match guess.cmp(&numero_secreto){
            Ordering::Less => println!("Muito menor"),
            Ordering::Greater => println!("Muito maior"),
            Ordering::Equal => {
                println!("Acertou");
                break;
            }
        }
    }

}