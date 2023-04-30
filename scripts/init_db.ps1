$CONTAINER_NAME = "zero2prod"
$DB_USER = "postgres"
$DB_PASSWORD = "password"
$DB_NAME = "newsletter"
$DB_PORT = "5432"

docker pull postgres

docker run `
  --name $CONTAINER_NAME `
  -e POSTGRES_USER=$DB_USER `
  -e POSTGRES_PASSWORD=$DB_PASSWORD `
  -e POSTGRES_DB=$DB_NAME `
  -p ${DB_PORT}:5432 `
  -d postgres `
  postgres -N 1000

$env:DATABASE_URL = "postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/newsletter"
