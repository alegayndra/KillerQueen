use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
  combinator::opt
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::scanners::texto::*;
use crate::parser::reglas_expresion::expresion::*;
use crate::semantica::globales::*;

fn generar_cuadruplo_lectura(id_valor: &str, _dims: Vec<&str>) {
  let variable;
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  let tabla_variables = VARIABLES.lock().unwrap();
  let tabla_funciones = FUNCIONES.lock().unwrap();
  let tabla_clases = CLASES.lock().unwrap();
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();

  match tabla_variables.buscar_variable(id_valor.to_owned()) {
    Ok((_, var)) => {
      variable = var;
    },
    Err(_) => {
      if contexto_clase.clone() != "".to_owned() {
        if contexto_funcion.clone() != "".to_owned() {
          variable = match tabla_clases.buscar_variable_metodo(contexto_clase.clone(), contexto_funcion.clone(), id_valor.to_owned()) {
            Ok((_, _, _, var)) => var,
            Err(err) => {
              println!("{:?}", err);
              return;
            },
          };
        } else {
          variable = match tabla_clases.buscar_atributo(contexto_clase.clone(), id_valor.to_owned()) {
            Ok((_, _, var)) => var,
            Err(err) => {
              println!("{:?}", err);
              return;
            },
          };
        }
      } else {
        variable =match tabla_funciones.buscar_variable(contexto_funcion.clone(), id_valor.to_owned()) {
          Ok((_, _, var)) => var,
          Err(err) => {
            println!("{:?}", err);
            return;
          },
        };
      }
    }
  };

  match cuadruplos.agregar_cuadruplo_lectura(variable) {
    Ok(_res) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };

  drop(contexto_funcion);
  drop(contexto_clase);

  drop(tabla_variables);
  drop(tabla_funciones);
  drop(tabla_clases);
  drop(cuadruplos);
}

fn generar_cuadruplo_escritura() {
  match PILA_VALORES.lock().unwrap().pop() {
    Some(valor) => {
      match CUADRUPLOS.lock().unwrap().agregar_cuadruplo_escritura(valor) {
        Ok(_res) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
    },
    _ => {
      println!("Stack de valores vacío en ESCRIBIR");
    }
  };
}

pub fn leer(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match tuple((tag("lee"), ws, tag("("), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match id_con_dim(next) {
    Ok((next_input, (id_valor, _dims))) => {
      generar_cuadruplo_lectura(id_valor, _dims);
      next_input
    },
    Err(err) => return Err(err)
  };

  loop {
    next = match opt(tuple((ws, tag(","), ws)))(next) {
      Ok((next_input, Some(_))) => next_input,
      _ => break
    };

    next = match id_con_dim(next) {
      Ok((next_input, (id_valor, _dims))) => {
        generar_cuadruplo_lectura(id_valor, _dims);
        next_input
      },
      Err(err) => return Err(err)
    };
  };

  match tuple((ws, tag(")"), tag(";")))(next) {
    Ok((next_input, _)) => Ok((next_input, "leer")),
    Err(err) => Err(err)
  }
}

fn agregar_texto_a_tabla(valor: &str) {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  pila_valores.push(CONSTANTES.lock().unwrap().agregar_constante(valor.to_owned(), "texto".to_owned()));
  drop(pila_valores);
  generar_cuadruplo_escritura();
}

pub fn escribir(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match tuple((tag("escribe"), ws, tag("("), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match texto(next) {
    Ok((next_i, texto_const)) => {
      agregar_texto_a_tabla(texto_const);
      next_i
    },
    Err(_) => {
      match expresion(next) {
        Ok((next_input, _)) => {
          generar_cuadruplo_escritura();
          next_input
        },
        Err(err) => return Err(err)
      }
    }
  };

  loop {
    next = match opt(tuple((ws, tag(","), ws)))(next) {
      Ok((next_input, Some(_))) => next_input,
      _ => break
    };

    next = match texto(next) {
      Ok((next_i, texto_const)) => {
        agregar_texto_a_tabla(texto_const);
        next_i
      },
      Err(_) => {
        match expresion(next) {
          Ok((next_input, _)) => {
            generar_cuadruplo_escritura();
            next_input
          },
          Err(err) => return Err(err)
        }
      }
    };
  };

  match tuple((ws, tag(")"), tag(";")))(next) {
    Ok((next_input, _)) => Ok((next_input, "escribir")),
    Err(err) => Err(err)
  }
}

pub fn funcion_esp(input: &str) -> IResult<&str, &str> {
  alt((leer, escribir))(input)
  .map(|(next_input, _)| {
    (next_input, "funcion_esp")
  })
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_leer() {
    assert_eq!(leer("lee(id);"),        Ok(("", "leer")));
    assert_eq!(leer("lee ( id );"),     Ok(("", "leer")));
    assert_eq!(leer("lee ( id, id );"), Ok(("", "leer")));
  }

  #[test]
  fn test_escribir() {
    assert_eq!(escribir("escribe(id);"),                        Ok(("", "escribir")));
    assert_eq!(escribir("escribe(\"abr\");"),                   Ok(("", "escribir")));
    assert_eq!(escribir("escribe ( id );"),                     Ok(("", "escribir")));
    assert_eq!(escribir("escribe(\"abr\", id, id, \"abr\");"),  Ok(("", "escribir")));
  }
}