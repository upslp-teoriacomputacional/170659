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

//definimos los tres sets

fn creacion()
{
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
}

//Remueve un elemento del conjunto

fn quitar()
{
   let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
   let mut B: HashSet<_> = [3, 4, 5, 6, 7].iter().cloned().collect();
   println!("\nEliminar datos del conjunto A: ");
   A.remove(&2);
   println!("\n------Conjunto A actualizado:------");
   for num in &A 
   {
    print!("{}", num);
   }
}

fn pertenencia()
{
   let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
   let mut B: HashSet<_> = [3, 4, 5, 6, 7].iter().cloned().collect();
   println!("\nPertenencia");
   println!("\n1 in A:");
   print!("{}", A.contains(&1));
   println!("\n1 not in A:");
   print!("{}", A.contains(&1));
   println!("\n10 in A:");
   print!("{}", A.contains(&10));
   println!("\n10 not in A:");
   print!("{}", A.contains(&10));
}
//Remueve todos los elementos del set
fn clearSet()
{
   let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
   A.clear();
   println!("\n------Conjunto A actualizado:------");
   for num in &A 
   {
    print!("{}", num);
   }
}
//Copia un conjunto
fn copiar()
{
  let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
  let mut B: HashSet<_> = A.clone();
  println!("Set A = {:?} compare set B = {:?}", &A, &B);
}
//Agrega un elemento
fn agregar()
{
   let mut B = HashSet::new();
   B.insert(1);
   B.insert(2);
   B.insert(3);
   B.insert(4);
   B.insert(5);
   B.insert(987);
   println!("\n------Conjunto B actualizado:------");
   for num in &B 
   {
    print!("{}", num);
   }
}

//union
fn union()
{
 let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
 let mut B: HashSet<_> = [1, 2, 3, 4, 5, 987].iter().cloned().collect();
 for x in A.union(&B) 
 {
  print!("{ }", x);
 }
}

//Interseccion
fn interseccion()
{
 let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
 let mut B: HashSet<_> = [1, 2, 3, 4, 5, 987].iter().cloned().collect();
 for x in A.intersection(&B) 
 {
  print!("{}", x);
 }
}
//diferencia
fn diferencia()
{
 let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
 let mut B: HashSet<_> = [1, 2, 3, 4, 5, 987].iter().cloned().collect();
 for x in A.difference(&B) 
 {
  print!("{}", x);
 }
}

//Diferencia simetrica
fn simetrica()
{
 let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
 let mut B: HashSet<_> = [1, 2, 3, 4, 5, 987].iter().cloned().collect();
 let mut C = HashSet::new();
 C.insert(1);
 C.clear();

 for x in A.symmetric_difference(&B) 
 {
  print!("{}", x);
 }
 for x in B.symmetric_difference(&A) 
 {
  print!("{}", x);
 }
 for x in A.symmetric_difference(&C) 
 {
  print!("{}", x);
 }
 for x in B.symmetric_difference(&C) 
 {
  print!("{}", x);
 }
}

//Subconjunto
fn subconjunto()
{
 let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
 let mut B: HashSet<_> = [1, 2, 3, 4, 5, 987].iter().cloned().collect();
 print!("El subconjunto= ");
 println!("{}",A.is_subset(&B));
 print!("El subconjunto= ");
 println!("{}",B.is_subset(&A));
}

//superconjunto
fn superconjunto()
{
 let mut A: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
 let mut B: HashSet<_> = [1, 2, 3, 4, 5, 987].iter().cloned().collect();
 print!("El superconjunto= ");
 println!("{}",B.is_superset(&A));
 print!("El superconjunto= ");
 println!("{}",A.is_superset(&B));
   
}

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
 
 quitar();
 clearSet();
 copiar();
 agregar();
 union();
 interseccion();
 diferencia();
 simetrica();
 subconjunto();
 superconjunto();
}

