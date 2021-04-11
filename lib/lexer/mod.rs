use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while},
    combinator::value,
    multi::many0,
    IResult,
    sequence::tuple,
};

pub fn sumsub_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-")))(input)
}

pub fn multdiv_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("/"), tag("*")))(input)
}

pub fn op_relacional_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("<="), tag("=="), tag(">="), tag("!="), tag("<"), tag(">")))(input)
}

pub fn op_logica_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("&"), tag("|")))(input)
}

pub fn tipo_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("entero"), tag("flotante"), tag("char")))(input)
}

pub fn arit(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((sumsub_parser, multdiv_parser))(input)
}

pub fn id(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric())(input)
}

pub fn ws(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c == ' ')(input)
}

pub fn necessary_ws(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == ' ')(input)
}

pub fn leer_parser(input: &str) -> IResult<&str, (&str, &str, &str, Vec<(&str, &str)>, &str)> {
    tuple((tag("lee"), necessary_ws, tag("("), ws, tag("id"),
        many0(tuple((
            ws, tag(","), 
            ws, tag("id")
        ))),
        ws, tag(")")
    ))
    (input)
    .map(|(next_input, res)| {
        let (lee, _, lp, _, id, ids, _, rp) = res;
        let mut lista_ids = Vec::new();
        for sid in ids {
            let (_, coma, _, sid2) = sid;
            lista_ids.push((
                coma,
                sid2
            ));
        }
        (
            next_input,
            (
                lee,
                lp,
                id,
                lista_ids,
                rp
            )
        )
    })
}

#[cfg(test)]
mod tests {
    
}
