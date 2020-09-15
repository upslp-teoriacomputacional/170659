# Conjuntos

El programa realiza las operaciones basicas de conjuntos, tales como crear un conjunto, eliminar
un elemento, etc. 

##Datos personales
Institucion: Universidad Politecnica de San Luis Potosi
Matricula: 170659
Nombre: Ramirez Castillo Daniela Guadalupe
Carrera: Ingenieria en Tecnologias de la Informacion
Materia: Teoria computacional

##Como solucionaste el problema
Lo inicial era encontrar los comandos basicos de Rust, asi como familiarizarse con la programacion basica de este, una vez hecho esto, se procede a descargar los complementos necesarios para que Visual Code pueda soportar Rust.

Para poder trabajar con conjuntos en Rust, es necesario utilizar la siguiente estructura: std::collections::HashSet, donde desde su sitio web maneja muchas opciones disponibles para trabajar con conjuntos.

Para crear un conjunto, es necesario seguir la siguiente estructura: let mut books = HashSet::new(); de esta manera se crea un conjunto vacio, y con books.insert("A Dance With Dragons".to_string()); puede insertar elementos al conjunto.

Rust nos permite trabajar con las siguientes modificaciones a los conjuntos:
.clear() donde se limpia por completo el conjunto
.insert() donde se inserta un elemento al conjunto
.remove() donde elimina un elemento del conjunto

y nos permite hacer las siguientes operaciones
.union() para lograr la union de dos conjuntos
.intersection() para ver la interseccion de dos conjuntos
.difference() para ver la diferencia entre dos conjuntos
.symmetric_difference() para obtener la diferencia simetrica de dos conjuntos
.is_subset() para saber si un conjunto es subconjunto de otro, esta funcion devuelve true o false
.is_superset() para saber si un conjunto es superconjunto de otro, al igual que subset() solo devuelve true o false

## Comentarios finales
Realmente fue complicado realizar la programacion, no por que el programa fuese complejo en si mismo, si no por que hay muy poca informacion acerca del lenguaje, ademas de la falta de un compilador propio hizo que las cosas se complicaran mucho para mi.

