## toolbox - start
## Generated with https://github.com/bakito/toolbox

## Current working directory
TB_LOCALDIR ?= $(shell which cygpath > /dev/null 2>&1 && cygpath -m $$(pwd) || pwd)
## Location to install dependencies to
TB_LOCALBIN ?= $(TB_LOCALDIR)/bin
$(TB_LOCALBIN):
	mkdir -p $(TB_LOCALBIN)

## Tool Binaries
TB_GOLANGCI_LINT ?= $(TB_LOCALBIN)/golangci-lint
TB_GORELEASER ?= $(TB_LOCALBIN)/goreleaser
TB_MOCKGEN ?= $(TB_LOCALBIN)/mockgen

## Tool Versions
TB_GOLANGCI_LINT_VERSION ?= v1.62.2
TB_GORELEASER_VERSION ?= v2.5.0
TB_MOCKGEN_VERSION ?= v0.5.0

## Tool Installer
.PHONY: tb.golangci-lint
tb.golangci-lint: $(TB_GOLANGCI_LINT) ## Download golangci-lint locally if necessary.
$(TB_GOLANGCI_LINT): $(TB_LOCALBIN)
	test -s $(TB_LOCALBIN)/golangci-lint || GOBIN=$(TB_LOCALBIN) go install github.com/golangci/golangci-lint/cmd/golangci-lint@$(TB_GOLANGCI_LINT_VERSION)
.PHONY: tb.goreleaser
tb.goreleaser: $(TB_GORELEASER) ## Download goreleaser locally if necessary.
$(TB_GORELEASER): $(TB_LOCALBIN)
	test -s $(TB_LOCALBIN)/goreleaser || GOBIN=$(TB_LOCALBIN) go install github.com/goreleaser/goreleaser/v2@$(TB_GORELEASER_VERSION)
.PHONY: tb.mockgen
tb.mockgen: $(TB_MOCKGEN) ## Download mockgen locally if necessary.
$(TB_MOCKGEN): $(TB_LOCALBIN)
	test -s $(TB_LOCALBIN)/mockgen || GOBIN=$(TB_LOCALBIN) go install go.uber.org/mock/mockgen@$(TB_MOCKGEN_VERSION)

## Reset Tools
.PHONY: tb.reset
tb.reset:
	@rm -f \
		$(TB_LOCALBIN)/golangci-lint \
		$(TB_LOCALBIN)/goreleaser \
		$(TB_LOCALBIN)/mockgen

## Update Tools
.PHONY: tb.update
tb.update: tb.reset
	toolbox makefile -f $(TB_LOCALDIR)/Makefile \
		github.com/golangci/golangci-lint/cmd/golangci-lint \
		github.com/goreleaser/goreleaser/v2 \
		go.uber.org/mock/mockgen@github.com/uber-go/mock
## toolbox - end