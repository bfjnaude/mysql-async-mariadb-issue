# mysql-async-mariadb-issue

Example of how to reproduce an issue with mysql_async and mariadb (10.3.23).

mysql_async 0.27.1 works with Mariadb. Upgrading to 0.28.1 causes authentication issues.

Running the example program using mysql_async 0.27.1 produces:

`Connected to database with version 10.3.23-MariaDB-log`

Running the example program using mysql_async 0.28.1 produces:

`thread 'main' panicked at 'Could not connect to db: Server(ServerError { code: 1045, message: "Access denied for user 'zulzi-user'@'10.240.0.29' (using password: YES)", state: "28000" })', src/main.rs:16:47`

## How to run
Set URL in the Makefile and run using `make cargo-run`

or

Pass the database connection URL to the program as the first argument.

