use std::fmt;

#[derive(Debug)]
pub struct Token(String);

#[derive(Debug)]
pub struct Tokens 
{	//Estructura para los tokens
	keywords: Vec<Token>,
	identifiers: Vec<Token>,
	values: Vec<Token>,
	operators: Vec<Token>,
	symbols: Vec<Token>,
	errors: Vec<Token>,
}

impl Tokens {
	pub fn new() -> Tokens 
	{
		Tokens 
		{
			keywords: Vec::new(),
			identifiers: Vec::new(),
			values: Vec::new(),
			operators: Vec::new(),
			symbols: Vec::new(),
			errors: Vec::new(),
		}
	}

	pub fn add_keyword(&mut self, keyword: &str) 
	{	//Agregar a palabras reservadas
		self.keywords.push( Token(String::from(keyword)) );
	}

	pub fn add_identifier(&mut self, identifier: &str) 
	{	//Agregar a identificadores
		self.identifiers.push( Token(String::from(identifier)) );
	}

	pub fn add_value(&mut self, value: &str) 
	{	//Agregar a valores
		self.values.push( Token(String::from(value)) );
	}

	pub fn add_symbol(&mut self, symbol: &str) 
	{	//Agregar a simbolos
		self.symbols.push( Token(String::from(symbol)) );
	}

	pub fn add_error(&mut self, error: &str) 
	{	//Agregar a esrrores
		self.errors.push( Token(String::from(error)) );
	}
}
//Imprimir todos los tokens
impl fmt::Display for Tokens 
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		writeln!(f, "_.-* Keywords *-._\n")?;

		for keyword in self.keywords.iter() 
		{
			write!(f, " {} |", keyword.0)?;
		}

		writeln!(f, ">\n")?;
		writeln!(f, "_.-* Identifiers *-._\n")?;
		for identifier in self.identifiers.iter() 
		{
			write!(f, " {} |", identifier.0)?;
		}


		writeln!(f, ">\n")?;
		writeln!(f, "_.-* Values *-._\n")?;
		for value in self.values.iter() 
		{
			write!(f, " {} |", value.0)?;
		}


		writeln!(f, ">\n")?;
		writeln!(f, "_.-* Symbols *-._\n")?;
		for symbol in self.symbols.iter() 
		{
			write!(f, " {} |", symbol.0)?;
		}


		writeln!(f, ">\n")?;
		writeln!(f, "_.-* Errors *-._\n")?;
		for error in self.errors.iter() 
		{
			write!(f, " {} |", error.0)?;
		}
		writeln!(f, ">\n")?;
	}
}