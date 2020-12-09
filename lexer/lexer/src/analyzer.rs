use regex::Regex;

use crate::tokenMatch::TokenMatch;

pub struct Analyzer 
{
	keywords: Vec<String>,
}

impl Analyzer 
{		//Palabras reservadas en ensamblador
	pub fn new() -> Analyzer 
	{
		Analyzer 
		{
			keywords: vec![	
				String::from("section"),
				String::from(".text"),
				String::from("global"),
				String::from("_start"),
				String::from("mov"),
				String::from("ecx"),
				String::from("cmp"),
				String::from("jg"),
				String::from("_exit"),
				String::from("msg"),
				String::from("eax"),
				String::from("edx"),
				String::from("len"),
				String::from("ebx"),
				String::from(".data"),
				String::from("dd"),
				String::from("segment"),
				String::from(".bss"),
				String::from("int"),
				String::from("db"),
				String::from("equ"),
				String::from("push"),
				String::from("sub"),
				String::from("inc"),
				String::from("add"),
				String::from("pop"),
				String::from("loop"),
				String::from("resb"),

			
			],
		}
	}

	pub fn analyze_token(&self, token: &str) -> TokenMatch 
	{	//Revisar de que tipo es

		if self.token_is_keyword(token) 
		{
			TokenMatch::Keyword
		} 
		else if self.token_is_operator(token) 
		{
			TokenMatch::Operator
		} 
		else if self.token_is_symbol(token) 
		{
			TokenMatch::Symbol
		} 
		else if self.token_is_value(token) 
		{
			TokenMatch::Value
		} 
		else if self.token_is_identifier(token) 
		{
			TokenMatch::Identifier
		} 
		else 
		{
			TokenMatch::Error	//No es valido
		}
	}

	fn token_is_keyword(&self, token: &str) -> bool 
	{	//Es palabra reservada
		self.keywords.contains(&String::from(token))
	}

	fn token_is_identifier(&self, token: &str) -> bool 
	{	//Es identificador
		let regex = Regex::new(r"^[a-zA-Z|_]*$").unwrap();
		regex.is_match(token)
	}

	fn token_is_operator(&self, token: &str) -> bool 
	{	//Es un operador
		let regex = Regex::new(r"^[>|<|.|=|+|\-|*|/]$").unwrap();
		regex.is_match(token)
	}

	fn token_is_value(&self, token: &str) -> bool 
	{	//Es un valor
		let regex = Regex::new(r"^(['].*[']|\d+|\d*\.\d+)$").unwrap();
		regex.is_match(token)
	}

	fn token_is_symbol(&self, token: &str) -> bool 
	{	//Es un simbolo
		let regex = Regex::new(r"^[:|;|,|(|)]$").unwrap();
		regex.is_match(token)
	}
}