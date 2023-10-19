# RustySEC
*RustySEC is an API security scanner written in pure 100% rust*
## API Security Scanner


### SPEC
*Scan OPEN API spec json files*
*Scan SOAP Service WSDL definition files*
*Scan against OWASP top-10*

### TODO
 - [x] Setup Project Skeleton
 - [x] Start API Specification Type Definition Recognition
 - [x] Create SECURITY THREAT CRITICALITY INDEX - LOW, MEDIUM, HIGH, CRITICAL
 - [ ] Start Parsing Definition FILES - Swagger.json for OPENAPI and WSDLs
    - [x] Lexing JSON already implementation
    - [ ] Parsing JSON
 - [ ] Collate Fixables -> Issues from Results of Parsing
 - [ ] Add OWASP-Top-10 LINKS to Fixable Display
 - [ ] Make it into one big badass CLI, API tool etc.