// Copyright (c) 2016 The Rouille developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::fs;

#[macro_use]
extern crate rouille;

fn main() {
    println!("Now listening on localhost:8000");

    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::text("Endpoints \n\n /save/:name/:value \n /load/:name/")

                // To do list items here
            },

            // Create a new file with contents as value param and filename as name param
            (GET) (/save/{name: String}/{value: String}) => {
                let output = value.to_string();
                print!("Save {}", name);
                let checkmark = '\u{2713}';
                println!(" {}", checkmark);
                fs::write(format!("save/{}", name), value).expect("Unable to write file");
                rouille::Response::text(format!("saved {}", output))
            },

            // Retrieve file based on filename and display data
            (GET) (/load/{name: String}) => {
                print!("Get {}", name);
                let data = fs::read_to_string(format!("save/{}", name)).expect("Unable to read file");
                println!(" --> Got {}", data);
                rouille::Response::text(data)
            },

            // Redirect in case someone accidentally uses /get instead of /load
            (GET) (/get/{name: String}) => {
                rouille::Response::redirect_302(format!("/load/{}", name))
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
