//! Módulo que se encarga de los diferentes estatutos.

use nom::{
  branch::alt,
  IResult,
};

use crate::parser::asignacion::*;
use crate::parser::func_esp::*;
use crate::parser::llama_func::*;
use crate::parser::repeticion::*;
use crate::parser::decision::*;
use crate::parser::comentario::*;
use crate::parser::regresa::*;
use crate::parser::declaraciones::variables::*;

/// No terminal de los diferentes estatutos.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// COMENTARIO | REGRESA | FUNC_ESP | REPETICION | DECISION | ASIGNACION | LLAMA_FUNC
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match estatuto("id = 10;") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn estatuto(input: &str) -> IResult<&str, &str> {
  alt((comentario, funcion_esp, repeticion, decision, regresa, variables, llama_func, asignacion))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_estatuto() {
    assert_eq!(estatuto("id = 10;"),              Ok(("", "asignacion")));
    assert_eq!(estatuto("lee(expresion);"),       Ok(("", "funcion_esp")));
    assert_eq!(estatuto("metodo();"),             Ok(("", "llama_func")));
    assert_eq!(estatuto("mientras(10 > 10) {}"),  Ok(("", "repeticion")));
    assert_eq!(estatuto("si (10 > 10) {}"),       Ok(("", "decision")));
    assert_eq!(estatuto("%% comentario %%"),      Ok(("", "comentario")));
    assert_eq!(estatuto("regresa char ;"),        Ok(("", "regresa")));
  }
}
