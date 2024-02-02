# Plan

## Input to server

**Rules:**

-   All input to server is done via API:s
-   All API:s through retropie/API/\*
-   All API calls are done through javascript
-   All data is sent via the request header
-   Input to server should be done with the proper request type

## Pages

-   No redircets. Change the content served
-   Content types are specified by their filepath
-   All content types need to be correct
-   If no page is requested return home page

## Data

-   All data that should be secret is encrypted. Cookies and passwords.
-   Right now that means encryption via RSA.
-   No user data except username, password and volvo card ID is stored
-   Passwords are stored hashed

## Encryption

-   Client requests public key.
-   Host sends public key
-   Client generates symmetric key
-   Client sends symmetric key encrypted with public key
-   Host decrypts symmetric key and uses it for further encrypted commmunication

## Changes

-   Only path to the OS is reading files. Therefore URL needs to be cleaned
-   Caching frequently requested data
-   Better UI
-   HTTPS certification
-   Symmetric key instead of RSA