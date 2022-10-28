# oauth2_provider_rs
OAuth2.0 provider is built on Rust.

## Road map
- [x] User signup (/signup)
- [x] User login (/login)
- [x] Authorize endpoint (/oauth/authorize)
- [x] OAuthClient　registration (/oauth_client/new)
- [x] Token generate (/oauth/token)
- [x] Token refresh (/oauth/token)
- [ ] PKCE
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
password | varchar(100)

### oauth_clients
column | type
---- | ----
*client_id | varchar(32)
name | varchar(32)
client_secret | varchar(32)
scope | varchar(32)
redirect_uri | varchar(255)

### access_tokens
column | type
---- | ----
*token | varchar(32)
user_id | int
client_id | int
scope | varchar(32)
expires_at | datetime

### refresh_tokens
column | type
---- | ----
*refresh_token | varchar(32)
access_token | varchar(32)
expires_at | datetime

### authorization_codes
column | type
---- | ----
*code | varchar(32)
user_id | int
client_id | varchar(32)
expires_at | datetime
