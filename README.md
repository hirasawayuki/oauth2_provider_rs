# README.md
OAuth2.0 authorization server in Rust.

## Road map
- [x] User signup (/signup)
- [x] User login (/login)
- [ ] Delete User Account
- [x] Authorize endpoint (/oauth/authorize)
- [x] OAuthClient registration (/oauth_client/new)
- [x] Delete OAuthClient
- [x] Token generate (/oauth/token)
- [x] Token refresh (/oauth/token)
- [x] Resource endpoint (/api/resources)
- [ ] PKCE
- [ ] Add Log create

## Build development
```sh
$ docker-compose build
$ docker-compose up -d
```

## DB migration
```
$ sqlx migrate run --database-url {DB_URL}/oauth2_development
```

## Usage
### Preparation
1. Access http://localhost/signup and create user
2. After login, access http://oauth_client/new and create OAuthClient
3. Logout

### Authorization
GET /oauth/authorize

Params | Required | example
---- | ---- | ----
client_id | ○ | 0b32d324-5284-46a7-b71f-1b4c228415d7
redirect_uri | ○ | http://localhost:8080/callback
response_type | ○ | code
scope | ○ | all
state | ○ | IuEInQ6TzROoFlZf4gbA0WaE19OyDl5TmJ9sddX9PRqykrP1Fb9F0oHBxTVHcMa

**Response**<br>
HTTP status 302<br>
Location: http://localhost:8080/callback?code={authorization_code}&state={state}

### Get AccessToken
POST /oauth/token

Params | Required | example
---- | ---- | ----
code | ○ | IuEInQ6TzROoFlZf4gbA0WaE19OyDl5TmJ9sddX9PRqykrP1Fb9F0oHBxTVHcMa
grant_type | ○ | authorization_code

**Response**<br>
HTTP status 200<br>
Body
```json
{
  "access_token": "{access_token}",
  "refresh_token": "{refresh_token}",
  "expires_at": "{access_token expires_at}"
}
```

### Refresh token
POST /oauth/token

Params | Required | example
---- | ---- | ----
refresh_token | ○ | IuEInQ6TzROoFlZf4gbA0WaE19OyDl5TmJ9sddX9PRqykrP1Fb9F0oHBxTVHcMa
grant_type | ○ | refresh_token

**Response**<br>
HTTP status 200<br>
Body
```json
{
  "access_token": "{access_token}",
  "refresh_token": "{refresh_token}",
  "expires_at": "{access_token expires_at}"
}
```

### Get Protected Resource
POST /api/resources
Header: Authorization: Bearer {Access token}

**Response**<br>
HTTP status 200<br>
Body
```json
{ "message": "Verify access token successful" }
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
name | varchar(63)
email | varchar(255)
password | varchar(100)

### oauth_clients
column | type
---- | ----
*client_id | varchar(63)
name | varchar(63)
client_secret | varchar(63)
scope | varchar(63)
redirect_uri | varchar(255)

### access_tokens
column | type
---- | ----
*token | varchar(63)
user_id | int
client_id | int
scope | varchar(63)
expires_at | datetime

### refresh_tokens
column | type
---- | ----
*refresh_token | varchar(63)
access_token | varchar(63)
expires_at | datetime

### authorization_codes
column | type
---- | ----
*code | varchar(63)
user_id | int
client_id | varchar(63)
expires_at | datetime
