# oauth2_provider_rs
OAuth2.0 provider is built on Rust.

## Road map
- [ ] Authorize endpoint (/oauth/authorize)
- [ ] Token endpoint (/oauth/token)
- [ ] Token refresh (/oauth/token)
- [ ] Token introspection (/oauth/introspection)

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
name | varchar(36)
email | varchar(256)
password | varchar(36)

### oauth_clients
column | type
---- | ----
*id | int
name | varchar(36)
scope | varchar(36)
revoked | tinyint(1)
redirect_uri | varchar(256)

### access_tokens
column | type
---- | ----
*token | varchar
user_id | int
client_id | int
scope | varchar(36)
revoked | tinyint(1)
expires_at | datetime

### refresh_tokens
column | type
---- | ----
*refresh_token | varchar(36)
access_token | varchar(36)
revoked | tinyint(1)
expires_at | datetime

### authrize_codes
column | type
---- | ----
*code | varchar(36)
user_id | int
client_id | int
revoked | tinyint(1)
expires_at | datetime
