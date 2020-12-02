// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *  
// # Nombre: Daniela Guadalupe Ramirez Castillo                                         #
// # Matricula: 170659                                                                  #
// # Carrera: ITI                                                                       #
// #                                                                                    #       
// # Descripcion: Codigo en Rust para una maquina de turing efectuando una division     #
// #                                                                                    #
// # Written: 11/30/2020                                                                #
// # Last Update: 12/1/2020                                                            #
//* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * 

use std::process;

fn turing(state: char, blank: char, in_tape: Vec<char>, fin: char, rules: Vec<Vec<char>>, position: usize)
{
    //Declaracion de variables
    let mut st= state;
    let mut pos = position;
    let mut tape: Vec<char>;
    tape = in_tape;
    let mut v=' ';
    let mut d=' ';
    let mut s=' ';


    if tape.is_empty()
    {
        tape.push(blank);
    }

    if pos < 0
    {
        pos += tape.len()
    }

    if pos>= tape.len() || pos < 0
    {
        println!("Error");
    }

    while st != fin
    {
        print! ("{}\t|",st);
        for i in 0..tape.len()
        {
            if i==pos
            {
                print!("[{}]", tape[i]);
            }
            else
            {
                print!(" {}", tape[i]);
            }
        }
        print! ("");

        for rule in &rules
        {
            if st==rule[0]&& tape[pos]==rule[1]
            {
                v=rule[2];
                d=rule[3];
                s=rule[4];
            }
        }
        for x in 0..tape.len()
        {
            if x == pos
            {
                pos=pos-1;
            }
            else
            {
                tape.insert(0,blank);
            }
        }
        if d== 'R'
        {
            pos=pos+1;
            if pos>=tape.len()
            {
                tape.push(blank);
            }
        }
        st=s;
    }

}
fn main() {
   let rules: Vec<Vec<char>>;
   let tape: Vec<char>;


   rules=vec![
          vec!['0','B','B','0','0'],
          vec!['0','1','1','R','0'],
          vec!['0','/','0','R','0'],
          vec!['0','=','=','L','0'],

          vec!['1','1','x','L','2'],

          vec!['2','1','1','L','2'],
          vec!['2','/','/','L','3'],


          vec!['3','B','B','L','3'],
          vec!['3',' ',' ','R','9'],
          vec!['3','1','B','R','4'],

          vec!['4','x','x','R','4'],
          vec!['4','B','B','R','4'],
          vec!['4','/','/','R','B'],
          vec!['4','=','=','R','5'],

          vec!['5','1','1','R','5'],
          vec!['5','1','1','R','1'],

          vec!['6','=','=','R','6'],
          vec!['6','1','1','L','6'],
          vec!['6','x','1','R','6'],
          vec!['6',' ',' ','L','7'],
          vec!['6','/','/','R','6'],

          vec!['7','=','=','R','7'],
          vec!['7','1','1','L','7'],
          vec!['7','1',' ','R','7'],
          vec!['7','/',' ','L','7'],
          vec!['7','=','=','L','4'],
   ];

   turing('0',' ',tape,'8',rules,0);
}
