CARGO = cargo
PROJECT_NAME = smart_home

GREEN = \033[0;32m
YELLOW = \033[0;33m
NC = \033[0m

.PHONY: all build test clean format lint doc run help

help:
	@echo "$(GREEN)Available commands:$(NC)"
	@echo "  $(YELLOW)make build$(NC)      - Build the project"
	@echo "  $(YELLOW)make test$(NC)       - Run all tests"
	@echo "  $(YELLOW)make clean$(NC)      - Remove build artifacts"
	@echo "  $(YELLOW)make format$(NC)     - Format code using rustfmt"
	@echo "  $(YELLOW)make lint$(NC)       - Run clippy linter"
	@echo "  $(YELLOW)make doc$(NC)        - Generate documentation"
	@echo "  $(YELLOW)make run$(NC)        - Run the main binary"
	@echo "  $(YELLOW)make all$(NC)        - Build, test, and lint the project"

build:
	@echo "$(GREEN)Building project...$(NC)"
	$(CARGO) build
	@echo "$(GREEN)Build complete.$(NC)"

test:
	@echo "$(GREEN)Running tests...$(NC)"
	$(CARGO) test
	@echo "$(GREEN)All tests passed.$(NC)"

clean:
	@echo "$(GREEN)Cleaning project...$(NC)"
	$(CARGO) clean
	@echo "$(GREEN)Clean complete.$(NC)"

format:
	@echo "$(GREEN)Formatting code...$(NC)"
	$(CARGO) fmt --all
	@echo "$(GREEN)Code formatted.$(NC)"

lint:
	@echo "$(GREEN)Running linter...$(NC)"
	$(CARGO) clippy -- -D warnings
	@echo "$(GREEN)Linting complete.$(NC)"

doc:
	@echo "$(GREEN)Generating documentation...$(NC)"
	$(CARGO) doc --no-deps --open
	@echo "$(GREEN)Documentation generated.$(NC)"

run:
	@echo "$(GREEN)Running main binary...$(NC)"
	$(CARGO) run
	@echo "$(GREEN)Execution complete.$(NC)"

all: build test lint
	@echo "$(GREEN)Full build process complete.$(NC)"

update:
	@echo "$(GREEN)Updating dependencies...$(NC)"
	$(CARGO) update
	@echo "$(GREEN)Dependencies updated.$(NC)"

release:
	@echo "$(GREEN)Building release version...$(NC)"
	$(CARGO) build --release
	@echo "$(GREEN)Release build complete.$(NC)"

profile:
	@echo "$(GREEN)Running with profiling...$(NC)"
	$(CARGO) build --release && \
	cargo install flamegraph && \
	flamegraph target/release/$(PROJECT_NAME)
	@echo "$(GREEN)Profiling complete.$(NC)"