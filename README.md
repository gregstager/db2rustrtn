# db2rustrtn

Db2 routines in Rust

This is a basic example of a C-compatible Db2 UDF created in Rust. It is meant to demonstrate that FFI-style Rust code can be called from Db2. Our routine is very simple and the output is doulbe the input integer.

## Compile Rust Code

Compile the shared library as any other Rust project:

```sh
cargo build --release
```

## Copy the library to the Db2 server

Copy the file `target/release/libdb2rustrn.so` to the `~/sqllib/function/` directory on the Db2 server.

## Create the Db2 Function

The file `create.db2` has the DDL statement to create the function.

You will need the following authorites in order to run it:

One of:

* CREATEIN on the current schema
* IMPLICITSCHEMA if the schema does not exist
* DBADM

As well one of:

* SYSADM
* CREATE_EXTERNAL_ROUTINE and CREATE_NOT_FENCED_ROUTINE

Create the function using the CLP:

```sh
db2 connect to <dbname>
db2 -td@ -f create.db2
```


## Call the function from Db2

```sql
db2 "values DoubleExample(17)"
1          
-----------
         34

  1 record(s) selected.
```