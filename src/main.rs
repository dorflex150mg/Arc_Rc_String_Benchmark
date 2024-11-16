use std::rc::Rc;
use std::sync::Arc;

fn main() {

    const STANDARD_STRING: &str = "STANDARD_STRING";
    const STANDARD_STRING_COPY: &str = "STANDARD_STRING";

    loop {
        let a_string: String = STANDARD_STRING.to_owned();
        let an_rc: Rc<str> = STANDARD_STRING.into(); 
        let anr_arc: Arc<str> =  STANDARD_STRING.into();
    }
}
