use std::io;
extern crate postgres;
#[macro_use]extern crate text_io;
use postgres::{Connection, TlsMode};


fn main() {
    println!("Bem vindo ao programa que vai te deixar mais organizado!");
    println!("Primeiramente vamos precisar das credenciais para acessar o banco de dados PostgreSQL!");
    //let link: String = get_db_credentials();
    let link = "postgresql://adilson:teste@localhost:5432/adilson";
    let connection = Connection::connect(link, TlsMode::None).unwrap();
    // Querys utilizadas
    let starter_query = "CREATE TABLE todolist IF NOT EXISTS(\
    id SERIAL PRIMARY KEY,\
    task VARCHAR(300) NOT NULL)";
    let show_tasks_query = "SELECT task FROM todolist";
    // Executando as querys
    connection.execute(starter_query,&[]).unwrap();
    print!("Criou OK!");
    
    //let resultado = connection.query(query,&[]);
    for i in &connection.query(show_tasks_query,&[]).unwrap(){
        let row:String = i.get("task");
        println!("{}",row);
    }
    print!("Tchau!")
}
fn get_db_credentials() -> String{
    // Função para retornar a url de conexão pro módulo postgres
    // Definição das variáveis
    let mut db_adress = String::new();
    let mut db_name = String::new();
    let mut db_user = String::new();
    let mut db_user_password = String::new();
    // Obtenção dos dados
    println!("Insira o endereço do banco de dados:");
    db_adress = read!();
    println!("Insira o nome do banco de dados:");
    db_name = read!();
    println!("Insira o nome de usuário do banco de dados:");
    db_user = read!();
    println!("Insira a senha do usuário:");
    db_user_password = read!();
    let mut param: String = "postgresql://".to_owned();
    // Criando o link de acesso ao DB
    param.push_str(&db_user);
    param.push_str(":");
    param.push_str(&db_user_password);
    param.push_str("@");
    param.push_str(&db_adress);
    param.push_str(":5432");
    param.push_str("/");
    param.push_str(&db_name);
    param
}

fn application_menu(){

}