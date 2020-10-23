// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *  
// # Nombre: Daniela Guadalupe Ramirez Castillo                                         #
// # Matricula: 170659                                                                  #
// # Carrera: ITI                                                                       #
// #                                                                                    #       
// # Descripcion: Codigo en Rust acerca de las expresines regulares                     #
// #                                                                                    #
// # Written: 10/23/2020                                                                #
// # Last Update: 10/23/2020                                                            #
//* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * 
extern crate regex;  // for performing regex expressions
use regex::Regex; 

fn main() {
    let mut tokens = Vec::new();
    let source_code = "int result = 100;".split(" ");
    let entero = Regex::new(r"[0-9]").unwrap();
    let operador = Regex::new(r"(\+|\-|\*|\/)").unwrap();
    let identificador = Regex::new(r"[a-z]").unwrap();
    //Loop through each source code word

    for word in source_code
    {
    //This will check if a token has datatype decleration
        if word == "str" || word=="int" || word=="bool"
        {
            tokens.insert("DATATYPE".to_string(), word.to_string());
        }
        //This will look for an identifier which would be just a word
         if  identificador.is_match(word)
        {
            tokens.insert("IDENTIFIER".to_string(), word.to_string());
        }
         if  operador.is_match(word) //This will look for an operator
        {
            tokens.insert("OPERADOR".to_string, word.to_string());
        }
        //This will look for integer items and cast them as a number
         if  entero.is_match(word)
        {
            word2=word.len();

            if word[word2-1..word2].to_string()--";"
            {
                tokens.insert("INTEGER".to_string(), word[0..(word2-1)].to_string());
                tokens.insert("END_STATEMENT".to_string(),";".to_string());
            }
            else
            {
                tokens.insert("INTEGER".to_string() ,word.to_string());
            }

        }

    }
    for token in &tokens
    {
        println!("{}",token); //Outputs the token array
    }

   
}
 fn variablePROLOG(w: String)->bool
    {
        let mut w1: Vec<char> = Vec::new();
        for chars in w.chars()
        {
            w1.push(chars);
        }
        let waux: Vec<_> = w[0].to_uppercase().collect();
        if w[0].is_alphabetic() && w[0] == (waux as char) || w[0] == '_' //#El primer caracter es una mayuscula o un subrayado
        {
       // Se quita el primer caracter
            w.reverse();
            w.pop();
            w.reverse();
            while  !w.is_empty() && (w[0].is_numeric()) || w[0]=="_" //Mientras queden caracteres en "w" y el primer caracter actual sea un alfanumerico o un subrayado, todo esta bien
            {
                w.reverse(); //Quitar el primer caracter
                w.pop();
                w.reverse();

                if w.is_empty()
                {
                    return true; //Si ya no quedan elementos a revisar, es una variable PROLOG
                }
            }
        }
    return false;

    }
