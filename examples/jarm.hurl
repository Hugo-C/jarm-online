# Request with host and port
GET {{host}}/jarm?host=google.com&port=443

HTTP 200
Content-Type: application/json

# Use default https port
GET {{host}}/jarm?host=google.com

HTTP 200
Content-Type: application/json

# Host is extracted from Url
GET {{host}}/jarm?host=https://google.com

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.host" == "google.com"

# NGINX correctly redirect API calls
GET {{nginx_host}}/api/v1/jarm?host=google.com

HTTP 200
Content-Type: application/json

# Error validation test
GET {{host}}/jarm?host=google.com&port=wow

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.error.error_type" == "Dns resolve error"

# Host is known to not respond
GET {{host}}/jarm?host=163.com

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.error.error_type" == "Connection error"