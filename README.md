# RESTful Todo
RESTful Todo API with [Actix](https://actix.rs) and [SeaORM](https://www.sea-ql.org/). Documented by [swagger-ui](https://swagger.io/)

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)

### Usage
Clone the repository and run the following commands:
```bash
cargo run
```
Just like that, you have a RESTful API running on your machine.

### Documentation
- The API documentation is available at [http://localhost:8080/docs/swagger/](http://localhost:8080/docs/swagger/)
- The OpenAPI specification is available at [http://localhost:8080/docs/openapi.json](http://localhost:8080/docs/openapi.json)

### Environment variables
<!-- Table of enviroment variables -->
| Name | Description | Default |
| --- | --- | --- |
| `DATABASE_URL` | The database url | `sqlite://db.sqlite3` |
| `SECRET_KEY` | The secret key for JWT | ` ` |
| `HOST` | The host to bind | `localhost` |
| `PORT` | The port to run the server | `8080` |
| `RATE_LIMIT_BURST_SIZE` | The burst size for rate limiter | `30` |
| `RATE_LIMIT_PER_SECOND` | The time to reset the burst | `60` |
| `API_CONTACT_NAME` | The name of the API contact | ` ` |
| `API_CONTACT_URL` | The url of the API contact | ` ` |
| `API_CONTACT_EMAIL` | The email of the API contact | ` ` |
| `API_TITLE` | The title of the API | `RESTful Todo API documentation` |

### Testing
#### Prerequisites
- [dotenv cli](https://pypi.org/project/python-dotenv/)
```bash
echo > db.sqlite3 ; dotenv cargo test tests::register -- --test-threads 1
dotenv cargo test tests::login -- --test-threads 1
dotenv cargo test tests::revoke -- --test-threads 1
```

## Soon
- [X] Swagger UI for API documentation
- [X] Rate limiting
- [ ] CI with Github Actions
- [ ] Unit tests
- [ ] Dockerize the server
- [ ] Makefile for easy setup, useing [cargo-make](https://github.com/sagiegurari/cargo-make)

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
