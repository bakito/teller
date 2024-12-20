# Include toolbox tasks
include ./.toolbox.mk

mocks: tb.mockgen
	$(TB_MOCKGEN) -source pkg/providers/aws_secretsmanager.go -destination pkg/providers/mock_providers/aws_secretsmanager_mock.go
	$(TB_MOCKGEN) -source pkg/providers/aws_ssm.go -destination pkg/providers/mock_providers/aws_ssm_mock.go
	$(TB_MOCKGEN) -source pkg/providers/cloudflare_workers_kv.go -destination pkg/providers/mock_providers/cloudflare_workers_kv_mock.go
	$(TB_MOCKGEN) -source pkg/providers/cloudflare_workers_secrets.go -destination pkg/providers/mock_providers/cloudflare_workers_secrets_mock.go
	$(TB_MOCKGEN) -source pkg/providers/consul.go -destination pkg/providers/mock_providers/consul_mock.go
	$(TB_MOCKGEN) -source pkg/providers/dotenv.go -destination pkg/providers/mock_providers/dotenv_mock.go
	$(TB_MOCKGEN) -source pkg/providers/doppler.go -destination pkg/providers/mock_providers/doppler_mock.go
	$(TB_MOCKGEN) -source pkg/providers/etcd.go -destination pkg/providers/mock_providers/etcd_mock.go
	$(TB_MOCKGEN) -source pkg/providers/google_secretmanager.go -destination pkg/providers/mock_providers/google_secretmanager_mock.go
	$(TB_MOCKGEN) -source pkg/providers/hashicorp_vault.go -destination pkg/providers/mock_providers/hashicorp_vault_mock.go
	$(TB_MOCKGEN) -source pkg/providers/heroku.go -destination pkg/providers/mock_providers/heroku_mock.go
	$(TB_MOCKGEN) -source pkg/providers/vercel.go -destination pkg/providers/mock_providers/vercel_mock.go
	$(TB_MOCKGEN) -source pkg/providers/onepassword.go -destination pkg/providers/mock_providers/onepassword_mock.go
	$(TB_MOCKGEN) -source pkg/providers/gopass.go -destination pkg/providers/mock_providers/gopass_mock.go
	$(TB_MOCKGEN) -source pkg/providers/github.go -destination pkg/providers/mock_providers/github_mock.go
	$(TB_MOCKGEN) -source pkg/providers/azure_keyvault.go -destination pkg/providers/mock_providers/azure_keyvault_mock.go
	$(TB_MOCKGEN) -source pkg/providers/keeper_secretsmanager.go -destination pkg/providers/mock_providers/keeper_secretsmanager_mock.go
readme:
	yarn readme
lint: tb.golangci-lint
	$(TB_GOLANGCI_LINT) run --fix
test:
	go test -v ./pkg/... -cover

integration:
	go test -v ./pkg/integration_test -cover -tags=integration

integration_api:
	go test -v ./pkg/integration_test -cover -tags="integration_api integration"

deps:
	go mod tidy && go mod vendor

release: tb.goreleaser tb.semver
	@version=$$($(TB_SEMVER)); \
	git tag -s $$version -m"Release $$version"
	$(TB_GORELEASER) --clean

test-release: tb.goreleaser
	$(TB_GORELEASER) --skip=publish --snapshot --clean

build:
	go build -ldflags "-s -w -X main.version=0.0.0 -X main.commit=0000000000000000000000000000000000000000 -X main.date=2022-01-01"

e2e: build
	BINARY_PATH="$(shell pwd)/teller" go test -v ./e2e

coverage:
	go test ./pkg/... -coverprofile=coverage.out
	go tool cover -func=coverage.out

.PHONY: deps setup-mac release readme lint mocks coverage

