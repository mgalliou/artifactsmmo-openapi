rm -rf src/ docs/ Cargo.toml git_push.sh openapitools.json README.md
npx @openapitools/openapi-generator-cli generate -i openapi.json -g rust -o . --additional-properties=supportAsync=false,packageName=artifactsmmo-openapi
