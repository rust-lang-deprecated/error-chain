#[macro_use]
extern crate error_chain;
use error_chain::CausedError;

error_chain! {
    errors {
        FileNotFound
    }
}

fn main() {}

pub fn pote() -> Result<()> {
    let res: ::std::result::Result<(), _> = get_result().map_err(|e| {
        e.caused_err(|| ErrorKind::FileNotFound)
    });
    //let _: () = match res {
    //    Ok(o) => o,
    //    e => return e.into(),
    //};
    let _: () = try!(res);
    //let _: () = res?;
    Ok(())
}

pub fn get_result() -> std::result::Result<(), std::io::Error> {
    unimplemented!();
}
