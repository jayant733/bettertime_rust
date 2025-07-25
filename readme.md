cargo .tome initialize our monorepo by just the 2 command 
resolver = "3"

2) cargo new api will create a new api set 

3) we use  poem for api and tokio for aysnchrounos work

4) now we gonna use multihtread library

5) macros are compared to decorationr in tyhpescript or annotation in javascript 

6) serde is the framework for serailizating and deserailizaing 

📌 Routing with dynamic path parameters (/hello/:name/:city)

📌 Defining HTTP methods (GET and POST)

📌 Handling request headers, cookies (mentioned but not implemented)

📌 Parsing JSON body into Rust structs (serde-like functionality via poem::web::Json)

📌 Returning JSON responses by serializing Rust structs

📌 Object destructuring in Rust (analogy to JavaScript destructuring)

📌 Alternative way to create TcpListener by constructing it manually instead of using .bind()


sqlx is the alternative of pg in rust world 
or diesel is the alternative or prisma 