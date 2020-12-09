use regex::Regex;
//Llamar a los otros archivos .rs
mod tokenMatch;
mod analyzer;
mod tokens;
//Que usar de los archivos llamados
use tokenMatch::TokenMatch;
use analyzer::Analyzer;
use tokens::Tokens;

//Librerias para leer los archivos
use std::fs::File;
use std::io;
use std::io::BufReader;	
use std::io::prelude::*;




fn main() 
{
	let mut filename;
	'choose_file: loop 
	{	//Elegir cuál archivo revisar
		println!("Choose a file: ");
		println!("  1.- hello.txt");
		println!("  2.- if.txt");
		println!("  3.- loop.txt");
		io::stdout().flush().unwrap();

		//Abrir el archivo
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let input = input.trim();
		println!("");
		
		filename = match input.as_ref() 
		{
			"1" => "assembly/hello.txt",
			"2" => "assembly/if.txt",
			"3" => "assembly/loop/txt",
			_ => { println!("Wrong option!"); "" },
		};
		if filename != "" 
		{
			break 'choose_file;
		}
	}

	//Leer el archivo
    let file = File::open(filename).unwrap();
	let mut buf_reader = BufReader::new(file);
	let mut text = String::new();
	buf_reader.read_to_string(&mut text).unwrap();

	let analyzer = Analyzer::new();
	let mut tokens_collector = Tokens::new();
	for (i, line) in text.lines().enumerate() 
	{	//Enumerar las lineas del programa a revisar
		let line = Vec::from(line);
		for token in line.split_whitespace() 
		{	//Separar los token cada espacio en blanco
			let token_result = analyzer.analyze_token(token);	//Analizar los tokens
			match token_result {	//Agregar segun el tipo de token que es
				TokenMatch::Keyword => 
				{
					tokens_collector.add_keyword(token)	//Palabra reservada
				},
				TokenMatch::Identifier => {
					tokens_collector.add_identifier(token)	//Identificador
				},
				TokenMatch::Value => {
					tokens_collector.add_value(token)	//Valor
				},
				TokenMatch::Operator => {
					tokens_collector.add_operator(token)	//Operadores
				},
				TokenMatch::Symbol => {
					tokens_collector.add_symbol(token)	//Simbolos
				},
				TokenMatch::Error => {
					println!("Error at line {} with token: '{}'", i+1, token);	
					tokens_collector.add_error(token)	//Errores
				},
			}
		}
	}

	println!("{}", tokens_collector);	//Imprimir todos los tokens
}
