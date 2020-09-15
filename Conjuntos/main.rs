// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *  
// # Nombre: Daniela Guadalupe Ramirez Castillo                                         #
// # Matricula: 170659                                                                  #
// # Carrera: ITI                                                                       #
// #                                                                                    #       
// # Descripcion: Codigo en Rust acerca de las principales operaciones con conjuntos    #
// #                                                                                    #
// # Written: 09/13/2020                                                                #
// # Last Update: 09/15/2020                                                            #
//* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * 
use std::collections::HashSet;
fn main() {
    let mut A = HashSet::new();
    let mut B = HashSet::new();
    let mut C = HashSet::new();

    A.insert(1);
    A.insert(2);
    A.insert(3);
    A.insert(4);
    A.insert(5);

    B.insert(3);
    B.insert(4);
    B.insert(5);
    B.insert(6);
    B.insert(7);

    C.insert(1);
    C.clear();

   println!("\n------Conjunto A:------");
   for num in &A 
   {
    print!("{}", num);
   }

   println!("\n------Conjunto B:------");
   for num in &B 
   {
    print!("{}", num);
   }

   println!("\n------Conjunto C:------");
   for num in &C 
   {
    print!("{}", num);
   }
   //pertenencia
   println!("\nPertenencia");
   println!("\n1 in A:");
   print!("{}", A.contains(&1));
   println!("\n1 not in A:");
   print!("{}", A.contains(&1));
   println!("\n10 in A:");
   print!("{}", A.contains(&10));
   println!("\n10 not in A:");
   print!("{}", A.contains(&10));
   //println!("\n1 not in A ", A.contains(&1));
   //println!("\n10 in A ", A.contains(&10));
   //println!("\n10 not in A ", A.contains(&1));
   

   //Eliminar datos del Conjunto

    println!("\nEliminar datos del conjunto A: ");
    A.remove(&2);

   println!("\n------Conjunto A actualizado:------");
   for num in &A 
   {
    print!("{}", num);
   }

   //Limpiar todo el conjunto
   B.clear();
   println!("\n------Conjunto B actualizado:------");
   for num in &B 
   {
    print!("{}", num);
   }

   //Copiar un conjunto 


   //Insertar un nuevo elemento
   B.insert(987);
    println!("\n------Conjunto A actualizado:------");
   for num in &A 
   {
    print!("{}", num);
   }
//------Operacion de Conjuntos--------
    println!("\n------Operacion de Conjuntos:------");
    println!("Union\n");
    let q: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let w: HashSet<_> = [4, 2, 3, 4,9,7,5].iter().cloned().collect();

    for x in q.union(&w) 
    {
        print!("{ }", x);
    }
    println!("\n Interseccion\n");
    for x in q.intersection(&w) 
    {
    print!("{}", x);
    }
    println!("\n Diferencia\n");

    for x in q.difference(&w) {
        print!("{}", x);
    }
    println!("\n Diferencia simetrica\n");
    for x in q.symmetric_difference(&w) 
    {
    print!("{}", x);
    }   

    println!("\nSubconjunto\n");
    print!("q subjconjunto de w:");
    println!("{}",q.is_subset(&w));

    println!("\nSuperconjunto\n");
    print!("q superconjunto de w:");
    println!("{}",q.is_superset(&w));
   
}
