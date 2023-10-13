use std::io;

fn main() {
    let mut pontuacao: i32 = 50;
    
    println!("Bem-vindo à floresta misteriosa!");
    
    loop {
        println!("Escolha sua próxima ação");

        println!("1 - Entrar na caverna escura");
        println!("2 - Seguir pelo caminho iluminado");
        println!("3 - Cruzar a ponte frágil");
        println!("4 - Descansar na beira do riacho");

        let mut escolha_str = String::new();
        let _ = io::stdin().read_line(&mut escolha_str);
        let escolha: u32 = match escolha_str.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };
        
        if escolha == 1 {
            println!("Você entrou na caverna escura e encontrou um tesouro parabéns!");
            pontuacao += 40;
        } else if escolha == 2 {
            println!("Você encontrou um Ogro poderoso, mas com sorte conseguiu escapar!");
            pontuacao -= 25;
        } else if escolha == 3 {
            println!("A ponte se quebrou com sorte você conseguiu nadar de volta pra margem");
            pontuacao -= 25;
        } else if escolha == 4 {
            println!("Você conseguiu recuperar um pouco das suas forças!");
            pontuacao += 10;
        }

        if pontuacao >= 100 {
            println!("Parabéns você é um verdadeiro aventureiro!");
            break;
        } else if pontuacao <= 0 {
            println!("Que pena você perdeu!");
            break;
        }
    }
    println!("Obrigado por jogar 'A floresta misteriosa'")
}
