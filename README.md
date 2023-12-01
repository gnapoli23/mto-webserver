
# mto-webserver

A simple Rust web server that calls [https://httpbin.org/post](https://httpbin.org/post) and returns a collection containing the numbers that appear More Than Once (mto).


## Project structure
```
.
├── migration
│   └── src
├── model
│   └── src
│       └── entity
├── service
│   └── src
│       ├── auth
│       └── crud
└── src
    ├── api
    │   └── request
    └── db
```

 The project is structured as a workspace made up of several members, that are considered as standalone libraries. This allows us to re-use the code in several different way, increasing maintainability and modularity:
 

 - **migration**: is the library where the migration files are defined. These files are generated using `sea-orm-cli`.
 - **model**: is the library where all the models related to the business logic are used. The `entity` folder has been created using `sea-orm-cli`.
 - **service**: is the library containing the logic related to both the `crud` service and the `auth` one.
 - **src/api**: is the module where the REST API server relies; it defines the endpoints for CRUD operations and for the `run` endpoint
 - **src/db**: is the module where the logic to connect to the underlying MySQL database


## API Design
The REST endpoints exposed are:
| Operation | Endpoint | Method | Body | Authentication
|--|--|--|--|--|
| Send requests to https://httpbin.org/post | `/run` | `GET` | *empty* | NO
| Add a new request | `/request/` | `POST` | JSON containing `id` and `value` (both *int*) | **YES**
| Get a request | `/request/:id` | `GET` | *empty* | **YES**
| Update a request | `/request/:id` | `PUT` | JSON containing `id` and `value` (both *int) | **YES**
| Delete a request | `/request/:id` | `DELETE` | *empty* | **YES**

**NOTE**: `/request/` endpoints use Basic Authentication. To use them, use a Basic Auth header in the requests with `username=admin` and `password=admin` (default one stored in the database).


## Database design
The database is `mto_db` and it is made up of two simple tables:


- **Request**
``` 
+-------+------+------+-----+---------+----------------+
| Field | Type | Null | Key | Default | Extra          |
+-------+------+------+-----+---------+----------------+
| id    | int  | NO   | PRI | NULL    | auto_increment |
| value | int  | NO   |     | NULL    |                |
+-------+------+------+-----+---------+----------------+
```


- **User**
```
+----------+--------------+------+-----+---------+----------------+
| Field    | Type         | Null | Key | Default | Extra          |
+----------+--------------+------+-----+---------+----------------+
| id       | int          | NO   | PRI | NULL    | auto_increment |
| username | varchar(255) | NO   | UNI | NULL    |                |
| password | varchar(255) | YES  |     | NULL    |                |
+----------+--------------+------+-----+---------+----------------+
```
 
 **NOTE**: the `user` table is automatically filled up with a user having `username=admin` and `password=password` on startup.

## Test
To run the test suite simply run `cargo test --workspace` in the project root folder.

## How to run
To run the project simply run

`docker compose up -d`

Once finished, Docker will create and start two containers:

- `mto-server`: where the logic relies, exposed on port `8080`
- `mto-db`: where the MySQL instance relies, exposed on default port `3306`