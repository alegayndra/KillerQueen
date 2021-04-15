use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::many0,
    IResult,
    sequence::tuple,
};

  
use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::funcion::*;
use crate::parser::variables::*;


fn lista_estautos(input: &str) -> IResult<&str, Vec<&str>> {
  many0(tuple((ws, tag("estatuto"))))(input)
  .map(|(next_input, res)| {
    let mut lista = Vec::new();
    for val in res {
      let (_, estatuto) = val;
      lista.push(estatuto);
    }
    (next_input, lista)
  })
}

fn bloque(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("{"), ws, lista_estautos, ws, tag("}")))(input)
  .map(|(next_input, res)| {
    let (_, _, estatuto, _, _,) = res;
    (next_input, estatuto)
  })
}