#[cfg(test)]
mod testes {
    use crate::*;

    #[test]
    fn teste_criacao_personagem() {
        let arma = Arma::Espada(75);
        let personagem =
            Personagem::novo(String::from("Aldric"), ClassePersonagem::Guerreiro, arma);
        assert_eq!(personagem.vida, 100);
    }

    #[test]
    fn teste_dano_arma() {
        let espada = Arma::Espada(80);
        let personagem = Personagem::novo(
            String::from("Cavaleiro"),
            ClassePersonagem::Guerreiro,
            espada,
        );
        assert_eq!(personagem.obter_dano_arma(), 80);
    }

    #[test]
    fn teste_receber_dano() {
        let arma = Arma::Cajado(50);
        let mut personagem = Personagem::novo(String::from("Merlin"), ClassePersonagem::Mago, arma);
        personagem.receber_dano(30);
        assert_eq!(personagem.vida, 70);
    }

    #[test]
    fn teste_vida_nao_negativa() {
        let arma = Arma::Adaga(40);
        let mut personagem =
            Personagem::novo(String::from("Sombra"), ClassePersonagem::Ladino, arma);
        personagem.receber_dano(150);
        assert_eq!(personagem.vida, 0);
    }

    #[test]
    fn teste_esta_vivo() {
        let arma = Arma::Espada(60);
        let mut personagem =
            Personagem::novo(String::from("Heroi"), ClassePersonagem::Guerreiro, arma);
        assert!(personagem.esta_vivo());
        personagem.receber_dano(100);
        assert!(!personagem.esta_vivo());
    }

    #[test]
    fn teste_descrever() {
        let arma = Arma::Cajado(70);
        let personagem = Personagem::novo(String::from("Gandalf"), ClassePersonagem::Mago, arma);
        let descricao = personagem.descrever();
        assert!(descricao.contains("Gandalf"));
        assert!(descricao.contains("Mago"));
        assert!(descricao.contains("100"));
    }

    #[test]
    fn teste_emprestimo_posse() {
        let arma1 = Arma::Espada(50);
        let arma2 = Arma::Espada(50);
        let pers1 = Personagem::novo(String::from("Alice"), ClassePersonagem::Guerreiro, arma1);
        let pers2 = Personagem::novo(String::from("Bob"), ClassePersonagem::Guerreiro, arma2);

        // Isso demonstra empréstimo - ambos personagens ainda existem depois
        assert!(mesma_classe(&pers1, &pers2));

        // Ainda podemos usar ambos os personagens
        assert_eq!(pers1.vida, 100);
        assert_eq!(pers2.vida, 100);
    }
}
