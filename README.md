This is a personal website written in Rust, HTML, and CSS (the Rust compiles 
to JavaScript and WebAssembly). The website contains simple plaintext and a
button that, when pressed, activates gravity on the page that causes the text
to fall and collide.

I use the Rust crate nphysics to simulate the world of the web page with
gravity and I use the Rust crate stdweb to interact with the website's DOM
through Rust.

Feel free to explore the code to see how it works!