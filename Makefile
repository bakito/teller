xsetup-mac:
	brew install golangci-lint
	go install github.com/golang/mock/mockgen@v1.5.0
mocks: mockgen
	$(MOCKGEN) -source pkg/providers/aws_secretsmanager.go -destination pkg/providers/mock_providers/aws_secretsmanager_mock.go
	$(MOCKGEN) -source pkg/providers/aws_ssm.go -destination pkg/providers/mock_providers/aws_ssm_mock.go
	$(MOCKGEN) -source pkg/providers/cloudflare_workers_kv.go -destination pkg/providers/mock_providers/cloudflare_workers_kv_mock.go
	$(MOCKGEN) -source pkg/providers/cloudflare_workers_secrets.go -destination pkg/providers/mock_providers/cloudflare_workers_secrets_mock.go
	$(MOCKGEN) -source pkg/providers/consul.go -destination pkg/providers/mock_providers/consul_mock.go
	$(MOCKGEN) -source pkg/providers/dotenv.go -destination pkg/providers/mock_providers/dotenv_mock.go
	$(MOCKGEN) -source pkg/providers/doppler.go -destination pkg/providers/mock_providers/doppler_mock.go
	$(MOCKGEN) -source pkg/providers/etcd.go -destination pkg/providers/mock_providers/etcd_mock.go
	$(MOCKGEN) -source pkg/providers/google_secretmanager.go -destination pkg/providers/mock_providers/google_secretmanager_mock.go
	$(MOCKGEN) -source pkg/providers/hashicorp_vault.go -destination pkg/providers/mock_providers/hashicorp_vault_mock.go
	$(MOCKGEN) -source pkg/providers/heroku.go -destination pkg/providers/mock_providers/heroku_mock.go
	$(MOCKGEN) -source pkg/providers/vercel.go -destination pkg/providers/mock_providers/vercel_mock.go
	$(MOCKGEN) -source pkg/providers/onepassword.go -destination pkg/providers/mock_providers/onepassword_mock.go
	$(MOCKGEN) -source pkg/providers/gopass.go -destination pkg/providers/mock_providers/gopass_mock.go
	$(MOCKGEN) -source pkg/providers/github.go -destination pkg/providers/mock_providers/github_mock.go
	$(MOCKGEN) -source pkg/providers/azure_keyvault.go -destination pkg/providers/mock_providers/azure_keyvault_mock.go
	$(MOCKGEN) -source pkg/providers/keeper_secretsmanager.go -destination pkg/providers/mock_providers/keeper_secretsmanager_mock.go
readme:
	yarn readme
lint: golangci-lint
	$(GOLANGCI_LINT) run --fix
test:
	go test -v ./pkg/... -cover

integration:
	go test -v ./pkg/integration_test -cover -tags=integration

integration_api:
	go test -v ./pkg/integration_test -cover -tags="integration_api integration"

deps:
	go mod tidy && go mod vendor

release:
	goreleaser --rm-dist

build:
	go build -ldflags "-s -w -X main.version=0.0.0 -X main.commit=0000000000000000000000000000000000000000 -X main.date=2022-01-01"

e2e: build
	BINARY_PATH="$(shell pwd)/teller" go test -v ./e2e

coverage:
	go test ./pkg/... -coverprofile=coverage.out
	go tool cover -func=coverage.out

.PHONY: deps setup-mac release readme lint mocks coverage

## toolbox - start
## Current working directory
LOCALDIR ?= $(shell which cygpath > /dev/null 2>&1 && cygpath -m $$(pwd) || pwd)
## Location to install dependencies to
LOCALBIN ?= $(LOCALDIR)/bin
$(LOCALBIN):
	mkdir -p $(LOCALBIN)

## Tool Binaries
GOLANGCI_LINT ?= $(LOCALBIN)/golangci-lint
MOCKGEN ?= $(LOCALBIN)/mockgen

## Tool Versions
GOLANGCI_LINT_VERSION ?= v1.59.1
MOCKGEN_VERSION ?= v0.4.0

## Tool Installer
.PHONY: golangci-lint
golangci-lint: $(GOLANGCI_LINT) ## Download golangci-lint locally if necessary.
$(GOLANGCI_LINT): $(LOCALBIN)
	test -s $(LOCALBIN)/golangci-lint || GOBIN=$(LOCALBIN) go install github.com/golangci/golangci-lint/cmd/golangci-lint@$(GOLANGCI_LINT_VERSION)
.PHONY: mockgen
mockgen: $(MOCKGEN) ## Download mockgen locally if necessary.
$(MOCKGEN): $(LOCALBIN)
	test -s $(LOCALBIN)/mockgen || GOBIN=$(LOCALBIN) go install go.uber.org/mock/mockgen@$(MOCKGEN_VERSION)

## Update Tools
.PHONY: update-toolbox-tools
update-toolbox-tools:
	@rm -f \
		$(LOCALBIN)/golangci-lint \
		$(LOCALBIN)/mockgen
	toolbox makefile -f $(LOCALDIR)/Makefile \
		github.com/golangci/golangci-lint/cmd/golangci-lint \
		go.uber.org/mock/mockgen@github.com/uber-go/mock
## toolbox - end
