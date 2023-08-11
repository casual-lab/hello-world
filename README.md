# A Flows.network API

## Introduction

This API is designed for validating a password.

Following are implemented rules:

- Password's length must be equal or greater than 5;
- Password can only and must contain letters (upper/lowercase) and decimal digits
- Username cannot be part of Password

## Usage

Following is the API URL:

<https://code.flows.network/lambda/fhARrkEOXD>

You can serialize the username and password as a JSON string and pass it in request body like this:

```bash
curl -d '{"username":"xyz", "password":"+==123456qwerQWER"}' https://code.flows.network/lambda/fhARrkEOXD
```

There are some example usage

```bash
$ curl -d '{"username":"xyz", "password":}' https://code.flows.network/lambda/fhARrkEOXD
Cannot resolve request body.
$ curl -d '{"username":"xyz", "password":"123456qwerQWER"}' https://code.flows.network/lambda/fhARrkEOXD
vlidation success.
$ curl -d '{"username":"xyz", "password":"qwerQWER"}' https://code.flows.network/lambda/fhARrkEOXD
Your password is in valid. Reason: 
Password must contain A-Z, a-z, and 0-9 to be complex enough.
$ curl -d '{"username":"xyz", "password":"123QWER"}' https://code.flows.network/lambda/fhARrkEOXD
Your password is in valid. Reason: 
Password must contain A-Z, a-z, and 0-9 to be complex enough.
$ curl -d '{"username":"xyz", "password":"+==123456qwerQWER"}' https://code.flows.network/lambda/fhARrkEOXD
Your password is in valid. Reason: 
Password can only contain A-Z, a-z or 0-9.
$ curl -d '{"username":"xyz", "password":"xyz123QWER"}' https://code.flows.network/lambda/fhARrkEOXD
Your password and username is in valid. Reason: 
Username cannot be part of password.
```
