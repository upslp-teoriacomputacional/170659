# Demostracion
El siguiente problema toma en cargo las expresiones regulares

Datos personales
Institucion: Universidad Politecnica de San Luis Potosi.

Matricula: 170659.

Nombre: Ramirez Castillo Daniela Guadalupe.

Carrera: Ingenieria en Tecnologias de la Informacion.

Materia: Teoria computacional.

## Como solucionaste el problema
Se utilizo la libreria ```bash regex``` para poder utilizar las expresiones regulares en el lenguaje.
Ademas, para poder comparar con strings o enteros se tuvo que crear una variable como la siguiente: ```bash et entero = Regex::new(r"[0-9]").unwrap();```
para que pudiese entrar a los if, ademas de que en la funcion variablePROLOG para eliminar el primer caracter se utilizaron
varias funciones repetidas veces:
```bash w.reverse(); 
                w.pop();
                w.reverse();```
## Comentarios finales
Fue un poco complicado debido a la utilizacion y modificacion del archivo CARGO.toml, ademas de que no se cuenta con mucha documentacion
del lenguaje, sobretodo en la ultima parte.
