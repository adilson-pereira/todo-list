use std::io;
extern crate postgres;
#[macro_use]extern crate text_io;
use postgres::{Connection, TlsMode};


fn main() {
    // Criando os links de acesso ao BD
        let link = "postgresql://adilson:teste@localhost:5432/adilson";
        let connection = Connection::connect(link, TlsMode::None).unwrap();

    // Criando querys que serão utilizadas

        let starter_query = "CREATE TABLE IF NOT EXISTS todolist(
        id SERIAL PRIMARY KEY,
        task VARCHAR(300) NOT NULL)";

        let show_tasks_query = "SELECT task FROM todolist";

        let delete_task_query = "DELETE FROM todolist WHERE id = $1";

    // Executando as querys
        connection.execute(starter_query,&[]).unwrap();

    // Main Task
        // Boas vindas do programa
            println!("Bem vindo ao programa que vai te deixar mais organizado!");
            println!("Primeiramente vamos precisar das credenciais para acessar o banco de dados PostgreSQL!");
            let mut user_answer:i8;
        // Loop principal
            loop {
                user_answer = application_menu();

                if(user_answer == 1){
                    // Imprimindo todas as linhas resultadas da consulta SQL ao banco
                    for i in &connection.query(show_tasks_query,&[]).unwrap(){
                        let row:String = i.get("task");
                        println!("{}", row);
                    }
                }

                if(user_answer == 2){

                }

                if(user_answer == 3){
                    // Perguntando ao usuário o índice da tarefa a ser excluída
                        println!("Insira o índice a ser excluído:");
                        let del_index_row:u32= read!();
                        let result = connection.execute(delete_task_query,&[&del_index_row]).unwrap();
                    // Informando linha alterada

                }

                if(user_answer == 4){
                    connection.finish();
                    break;
                }
            }

    println!("Obrigado por utilizar o programa!")
}
fn get_db_credentials() -> String{
    // Definição das variáveis
        let mut db_adress = String::new();
        let mut db_name = String::new();
        let mut db_user = String::new();
        let mut db_user_password = String::new();
        let mut param: String = "postgresql://".to_owned();
    // Obtenção dos dados
        println!("Insira o endereço do banco de dados:");
        db_adress = read!();
        println!("Insira o nome do banco de dados:");
        db_name = read!();
        println!("Insira o nome de usuário do banco de dados:");
        db_user = read!();
        println!("Insira a senha do usuário:");
        db_user_password = read!();
    // Criando o link de acesso ao DB
        param.push_str(&db_user);
        param.push_str(":");
        param.push_str(&db_user_password);
        param.push_str("@");
        param.push_str(&db_adress);
        param.push_str(":5432");
        param.push_str("/");
        param.push_str(&db_name);
    // Retornando a variável
        param
} // Get the database credentials to remote/local access

fn application_menu()  -> i8 {
    // Imprimindo o MENU
        println!("Selecione uma opção:");
        println!("1 - Listar tarefas");
        println!("2 - Inserir tarefa");
        println!("3 - Excluir tarefa");
        println!("4 - Sair do programa");
    // Lendo a opção
        let answer = read!();
    // Retornando a opção selecionada
        answer
} // Return a selected option