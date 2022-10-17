# oauth2_provider_rs
OAuth2.0 provider is built on Rust.

## Road map
- [ ] Authorize endpoint (/oauth/authorize)
- [ ] Token endpoint (/oauth/token)
- [ ] Token refresh (/oauth/token)
- [ ] Token introspection (/oauth/introspection)

## Library
Type | Name | URL
---- | ---- | ----
HTTP Server | actix-web | https://github.com/actix/actix-web
DB | sqlx | https://github.com/launchbadge/sqlx
Error Handling | anyhow | https://github.com/dtolnay/anyhow

## Database

### users
*id: int
name: varchar
email: varchar
password: varchar

### oauth_clients
*id: int
name: varchar
scope: varchar
revoked: tinyint
redirect_uri: varchar

### access_tokens
*token: varchar
user_id: int
client_id: int
scope: varchar
revoked: tinyint
expires_at: datetime

### refresh_tokens
*refresh_token: varchar(36)
access_token: varchar(36)
revoked: tinyint
expires_at: datetime

### authrize_codes
*code: varchar(36)
user_id: int
client_id: int
revoked: tinyint
expires_at: datetime
