use rustler::{Encoder, Env, Error, Term};

rustler::rustler_export_nifs! {
    "Elixir.RustPhxSampleWeb.SampleController",
    [
        ("add", 2, add)
    ],
    None
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((num1 + num2).encode(env))
}
