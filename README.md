# oauth2_provider_rs
OAuth2.0 provider is built on Rust.

## Road map
- [x] User signup (/signup)
- [x] User login (/login)
- [x] Authorize endpoint (/oauth/authorize)
- [x] Register OAuthClient (/oauth_client/new)
- [ ] Token endpoint (/oauth/token)
- [ ] Token refresh (/oauth/token)
- [ ] Token introspection (/oauth/introspection)
- [ ] Resource endpoint (/resources) 

## Usage
### Build development
```sh
$ docker-compose build
$ docker-compose up -d
```

### DB migration
```
$ sqlx migrate run --database-url {DB_URL}/oauth2_development
```

## Library
### Backend
Type | Name | URL
---- | ---- | ----
HTTP Server | actix-web | https://github.com/actix/actix-web
DB | sqlx | https://github.com/launchbadge/sqlx
Error Handling | anyhow | https://github.com/dtolnay/anyhow

## Database
### users
column | type
---- | ----
*id | int
name | varchar(32)
email | varchar(255)
password | varchar(36)

### oauth_clients
column | type
---- | ----
*id | int
name | varchar(32)
client_id | varchar(255)
client_secret | varchar(255)
scope | varchar(32)
revoked | tinyint(1)
redirect_uri | varchar(255)

### access_tokens
column | type
---- | ----
*token | varchar
user_id | int
client_id | int
scope | varchar(32)
revoked | tinyint(1)
expires_at | datetime

### refresh_tokens
column | type
---- | ----
*refresh_token | varchar(32)
access_token | varchar(32)
revoked | tinyint(1)
expires_at | datetime

### authorize_codes
column | type
---- | ----
*code | varchar(32)
user_id | int
client_id | int
revoked | tinyint(1)
expires_at | datetime
