#![allow(dead_code)]

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

#[cfg(feature = "log")]
#[macro_use]
extern crate error_chain;

#[cfg(feature = "log")]
#[cfg(test)]
mod log_ext_tests {

    #[test]
    fn logext_macro_call_for_error() {
        macro_rules! check {
            ( $what:ident , $fun:expr ) => (
                match $what {
                    Error(ErrorKind::Msg(_), ..) => (),
                    _ => panic!("{} did not return an error: {:?}",$fun, $what)
                }
            )
        }
        error_chain! {
                    errors {
                            Test
            }
        }
        let msg1 = "My test error";
        let msg2 = "My test warn"; 
        let msg3 = "My test info"; 
        let msg4 = "My test debug"; 
        let msg5 = "My test trace";
        fn base() -> Error { Error::from(ErrorKind::Test) }
        let erre = base().chain_err(|| msg1).loge();
        let errw = base().chain_err(|| msg2).logw();
        let erri = base().chain_err(|| msg3).logi();
        let errd = base().chain_err(|| msg4).logd();
        let errt = base().chain_err(|| msg5).logt();

        check!(erre,"loge");
        check!(errw,"logw");
        check!(erri,"logi");
        check!(errd,"logd");
        check!(errt,"logt");
    }

    #[test]
    fn logext_macro_call_for_result() {
        macro_rules! check {
            ( $what:ident , $fun:expr) => (
                match $what {
                    Err(Error(..)) => (),
                    _ => panic!("{} did not return a result type!",$fun)
                }
            )
        }
        error_chain! {
                    errors {
                            Test
            }
                
        }

        fn base() -> Result<()>  { Err( Error::from(ErrorKind::Test) ) }

        let rese = base().chain_err(|| "My test error").loge();
        let resw = base().chain_err(|| "My test warn").logw();
        let resi = base().chain_err(|| "My test info").logi();
        let resd = base().chain_err(|| "My test debug").logd();
        let rest = base().chain_err(|| "My test trace").logt();
        check!(rese,"loge");
        check!(resw,"logw");
        check!(resi,"logi");
        check!(resd,"logd");
        check!(rest,"logt");
    }
}
