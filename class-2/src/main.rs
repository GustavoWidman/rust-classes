enum Boolean {
    True,
    False,
    Maybe,
    Null,
}

impl ToString for Boolean {
    fn to_string(&self) -> String {
        match self {
            Boolean::True => "true".to_string(),
            Boolean::False => "false".to_string(),
            _ => "sla".to_string(),
        }
    }
}

struct Pessoa<T: ToString> {
    altura: u64,
    idade: u8,
    nome: String,
    tipo_favorito: T,
}

impl<T: ToString> Pessoa<T> {
    fn greet(&self) {
        println!(
            "ola, sou {}, tenho {}, e {}cm de altura",
            self.nome.to_string(),
            self.idade.to_string(),
            self.altura
        )
    }

    fn show_favorite_type(&self) {
        println!("{}", self.tipo_favorito.to_string());
    }
}

fn main() {
    println!("Hello, world!");
    let mut string = "hello, world".to_string();
    tell_me_why(&string);
    string = "goodbye, world".to_string();
    tell_me_why(&string);
    drop(string);

    let my_real_boolean = true;
    if my_real_boolean == true {
        println!("this is true");
    }

    let my_boolean: Boolean = match give_me_a_boolean() {
        Ok(boolean) => boolean,
        Err(erro) => tratar_erro(erro),
    };

    let my_boolean: Boolean = give_me_a_boolean().unwrap_or_else(|erro: String| {
        println!("deu erro: {erro}, vamos voltar para o default");

        Boolean::Null
    });

    if let Boolean::True = my_boolean {
        println!("this is true");
    }
    match my_boolean {
        Boolean::True => {
            println!("this is true");
        }
        Boolean::False => {}
        Boolean::Maybe => {}
        Boolean::Null => {}
    }

    let my_option: Option<String> = Option::Some("my optional string".to_string());

    if let Option::Some(my_string) = &my_option {
        println!("{my_string}");
    };

    match &my_option {
        Option::Some(string) => {
            println!("{string}");
        }
        Option::None => {
            println!("nao temos uma string");
        }
    }

    let ian = Pessoa {
        altura: 173,
        idade: 21,
        nome: "Ian Simao".to_string(),
        tipo_favorito: Boolean::True,
    };
    ian.greet();
}

fn give_me_a_boolean() -> Result<Boolean, String> {
    if true == true {
        return Result::Err("deu merda".to_string());
    };

    return Result::Ok(Boolean::False);
}

fn tell_me_why(x: &String) {
    println!("Aint nothing but a heart ache. {x}")
}

fn mutate_string(x: &mut String) {
    let new_string = "goodybye, world".to_string();
    *x = new_string;
}

fn tratar_erro(erro: String) -> Boolean {
    println!("deu erro: {erro}, vamos voltar para o default");

    Boolean::Null
}
