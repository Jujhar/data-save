// Copyright (c) 2016 The Rouille developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#[macro_use]
extern crate rouille;

fn main() {
    println!("Now listening on localhost:8000");

    rouille::start_server("localhost:8000", move |request| {

        router!(request,
            (GET) (/) => {
                rouille::Response::text("Endpoints \n\n /save/:name/:value \n /get/:name/")
            },

            (GET) (/save/{name: String}/{value: String}) => {
                println!("will save {}", name);
                rouille::Response::text("save")
            },

            (GET) (/get/{name: String}) => {
                println!("will get {}", name);
                rouille::Response::text("get")
            },

            (GET) (/panic) => {
                // If the request's URL is `/panic`, we jump here.
                //
                // This block panics. Fortunately rouille will automatically catch the panic and
                // send back a 500 error message to the client. This prevents the server from
                // closing unexpectedly.
                panic!("Oops!")
            },

            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
    });
}
