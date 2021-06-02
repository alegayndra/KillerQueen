//! Parser del lenguaje _Killer Queen_
//! 
//! Utilizamos las librerias de _nom_ y _lazy static_, para parsear y crear variables estáticas en ejecución respectivamente.

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::fs;

pub mod scanners;
pub mod semantica;
pub mod parser;

use crate::parser::programa::*;
use crate::semantica::globales::*;

/// Escribe el archivo de salida.
///
/// # Ejemplo
///
/// ```
/// escribir_archivo();
/// ```
fn escribir_archivo() {
	let arch = "Compilador/cuadruplos/killer_queen.txt";
	let path = Path::new(arch);

	let display = path.display();

	// Open a file in write-only mode, returns `io::Result<File>`
	let mut file = match File::create(&path) {
		Err(why) => panic!("couldn't create {}: {}", display, why),
		Ok(file) => file,
	};

	// Variables globales de semántica
	let tabla_funciones = FUNCIONES.lock().unwrap();
	let id_programa = ID_PROGRAMA.lock().unwrap();
	let constantes = CONSTANTES.lock().unwrap();
	let cuadruplos = CUADRUPLOS.lock().unwrap();
	// let tabla_clases = CLASES.lock().unwrap();

	let mut texto_archivo: String = "".to_owned();

	// Escritura constantes
	{
		let mut texto_constantes: String = "".to_owned();
	
		unsafe {
			let era_constantes = format!("({}, {}, {})", ERA_CONSTANTES.0, ERA_CONSTANTES.1, ERA_CONSTANTES.2);
			texto_constantes = format!("{}{}\n", texto_constantes, era_constantes);
		}
	
		for (_key, val) in constantes.tabla.iter() {
			let tipo_var = match val.tipo.as_str() {
				"texto" => "char",
				tipo => tipo
			}.to_owned();
			let const_string: String = format!("({}, {}, {})", val.nombre, val.direccion, tipo_var);
			texto_constantes = format!("{}{}\n", texto_constantes, const_string);
			// println!("key: {} val: {}", key, val);
		}
	
		texto_archivo = format!("{}CONSTANTES\n{}FIN_CONSTANTES\n", texto_archivo, texto_constantes);
	}

	// Escritura globales
	{
		let mut texto_globales: String = "".to_owned();
	
		match tabla_funciones.tabla.get(&id_programa.to_string()) {
			Some(vars) => {
				let mut globales_string: String = "".to_owned(); // Faltan dimensiones
				for tam in vars.era.iter() {
					let tam_string: String = format!("({}, {})", tam.0, tam.1);
					globales_string = format!("{}{}\n", globales_string, tam_string);
				}
				texto_globales = format!("{}{}", texto_globales, globales_string);
				()
			},
			None => ()
		}
	
		texto_archivo = format!("{}GLOBALES\n{}FIN_GLOBALES\n", texto_archivo, texto_globales);
	}

	// Escritura funciones
	{
		let mut texto_funciones: String = "".to_owned();
	
		for (key, val) in tabla_funciones.tabla.iter() {
			if key.to_owned() != id_programa.to_string() {
				let funcion_string: String = format!("({}, {}, {})", val.nombre, val.direccion, val.num_cuadruplo); // Faltan dimensiones
				texto_funciones = format!("{}{}\n", texto_funciones, funcion_string);
				let mut tamanio_string: String = "".to_owned(); // Faltan dimensiones
				for tam in val.era.iter() {
					let tam_string: String = format!("({}, {})", tam.0, tam.1);
					tamanio_string = format!("{}{}\n", tamanio_string, tam_string);
				}
				texto_funciones = format!("{}{}", texto_funciones, tamanio_string);
				let mut lista_parametros: String = "".to_owned();
				for param in val.parametros.iter() {
					let param_string: String = format!("({}, {})", param.direccion, param.tipo);
					lista_parametros = format!("{}{}\n", lista_parametros, param_string);
				}
				texto_funciones = format!("{}PARAMS\n{}FIN_PARAMS\n", texto_funciones, lista_parametros);
			}
		}
	
		texto_archivo = format!("{}FUNCIONES\n{}FIN_FUNCIONES\n", texto_archivo, texto_funciones);
	}

	// Escritura cuadruplos
	{
		let mut lista_cuadruplos: String = "".to_owned();
	
		for cuad in cuadruplos.lista.iter() {
			let cuad_string: String = format!("({}, {}, {}, {})", cuad.0, cuad.1, cuad.2, cuad.3);
			lista_cuadruplos = format!("{}{}\n", lista_cuadruplos, cuad_string);
		}
	
		texto_archivo = format!("{}CUADRUPLOS\n{}FIN_CUADRUPLOS\n", texto_archivo, lista_cuadruplos);
	}

	// Guardado de archivo
	match file.write_all(texto_archivo.as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", display, why),
		Ok(_) => println!("successfully wrote to {}", display),
	}
}

/// Inicia todo el proceso de compilación.  
/// Lee el archivo de entrada, empieza el análisis del lenguaje y escribe el archivo de salida.  
///
/// # Ejemplo
///
/// ```
/// iniciar_compilador();
/// ```
pub fn iniciar_compilador() {
  // Consigue las variables de ambiente
  let args: Vec<String> = env::args().collect();
  let nombre_archivo = &args[1];

  // Agrega al nombre del archivo la terminación .eo
  let arch = format!("{}.eo", nombre_archivo);

  // Lee archivo
  println!("Leyendo archivo {}", arch.clone());
  let contents = fs::read_to_string(&arch).expect("Something went wrong reading the file");
  println!("Archivo leído correctamente");

  // Analiza el código fuente dado y genera el archivo de sálida en caso de que sea un éxito
  match programa(&contents) {
    Ok(_) => {
      escribir_archivo();
    },
    Err(err) => {
      println!("{:?}", err);
    }
  };
}