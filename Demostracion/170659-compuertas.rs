// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *  
// # Nombre: Daniela Guadalupe Ramirez Castillo                                         #
// # Matricula: 170659                                                                  #
// # Carrera: ITI                                                                       #
// #                                                                                    #       
// # Descripcion: Codigo en Rust de las tablas de verdad                                #
// #                                                                                    #
// # Written: 09/21/2020                                                                #
// # Last Update: 09/22/2020                                                            #
//* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * 
fn main() {
let booleanos=[false,true];

//Tabla de verdad de or
println!("x\ty\tx or y");
println!("-----------------------");


for x in booleanos.iter()
{
    for y in booleanos.iter()
    {
        println!("{}\t{}\t{}",x,y, (x|y));
    }
}
//Tabla de verdad de and
println!("x\ty\tx AND y");
println!("-----------------------");

for x in booleanos.iter()
{
    for y in booleanos.iter()
    {
        println!("{}\t{}\t{}",x,y, (x&y));
    }
}
//Tabla de verdad de not
println!("x\t NOT x");
println!("--------------");

for x in booleanos.iter()
{
    println!("{}\t{}",x,!x);
}
//Tabla de verdad de ^
println!("x\ty\tx ^ y");
println!("-----------------------");

for x in booleanos.iter()
{
    for y in booleanos.iter()
    {
        println!("{}\t{}\t{}",x,y, (x^y));
    }
}

}
