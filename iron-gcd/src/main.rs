extern crate iron;
#[macro_use] extern crate mime;

use iron:prelude::*;
use iron::status;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();  // .unwrap() Suppreses errors!
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Test/Html;Charset=Utf8));
    // The r#" "# is rusts raw text format, meaning nothing inside will be escaped. NEATO!
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type = "text" name = "n"/>
            <input type = "text" name = "n"/>
            <button type = "submit">Compute GCD</button>
        </form>
    "#);

    Ok(response);
}
