# Request for no overlap
GET {{host}}/alexa-overlap?jarm_hash=123

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.overlapping_domains" isEmpty

# Request with 1 overlap
GET {{host}}/alexa-overlap?jarm_hash=3fd3fd20d3fd3fd21c3fd3fd3fd3fd2b66a312d81ed1efa0f55830f7490cb2

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.overlapping_domains" count == 1
jsonpath "$.overlapping_domains.[0].rank" == 9
jsonpath "$.overlapping_domains.[0].domain" == "zhihu.com"

# Request with 2 overlap
GET {{host}}/alexa-overlap?jarm_hash=21d19d00021d21d00021d19d21d21d1a46380b04d662f0848f508dd171125d

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.overlapping_domains" count == 2
jsonpath "$.overlapping_domains.[0].rank" == 11
jsonpath "$.overlapping_domains.[0].domain" == "fake_site_1.com"
jsonpath "$.overlapping_domains.[1].rank" == 12
jsonpath "$.overlapping_domains.[1].domain" == "fake_site_2.com"