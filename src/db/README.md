# Migrations

To add a new migration, run
```shell
sqlx migrate add --sequential --source src/db/migrations/ <name>
```