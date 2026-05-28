#!/usr/bin/env bash
# ============================================================
#  test_hangman.sh — Automated Test Runner
# ============================================================
#  Usage:
#    chmod +x test_hangman.sh
#    ./test_hangman.sh
#
#  This script:
#    1. Verifies the project compiles
#    2. Runs all cargo unit tests
#    3. Validates timer / SIGALRM signal handler usage
#    4. Reports a pass/fail summary and TOTAL_SCORE (out of 100)
# ============================================================

set -uo pipefail

# ── Colors ────────────────────────────────────────────────
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

PASS=0
FAIL=0
TOTAL_SCORE=0

# Always print TOTAL_SCORE on exit (even if we bail out early)
print_total_score() {
    echo "TOTAL_SCORE: $TOTAL_SCORE"
}
trap print_total_score EXIT

pass()    { local pts="${2:-0}"; echo -e "${GREEN}  [PASS +${pts}]${NC} $1"; PASS=$((PASS+1)); TOTAL_SCORE=$((TOTAL_SCORE + pts)); }
fail()    { local pts="${2:-0}"; echo -e "${RED}  [FAIL +0/${pts}]${NC} $1"; FAIL=$((FAIL+1)); }
info()    { echo -e "${CYAN}  [INFO]${NC} $1"; }
section() { echo -e "\n${YELLOW}══ $1 ══${NC}"; }

# ── Locate project root ───────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

# ── 0. Sanity checks ──────────────────────────────────────
section "0. Environment"

if ! command -v cargo &>/dev/null; then
    echo -e "${RED}[ERROR] cargo not found. Install Rust: https://rustup.rs${NC}"
    exit 1
fi
echo "  Rust/Cargo is available ($(cargo --version))"

if [[ ! -f "words/words.txt" ]]; then
    fail "words/words.txt not found"
    exit 1
fi
echo "  words/words.txt exists"

# ── 1. Compilation ────────────────────────────────────────
#  5 pts awarded only if all unit tests also pass (checked at
#  the end of section 2). A project full of todo!() macros
#  compiles cleanly but fails every test — no reward for that.
section "1. Compilation [5 pts — awarded only if all unit tests pass]"

if cargo build 2>&1 | grep -q "^error"; then
    fail "Project does not compile" 5
    echo ""
    cargo build 2>&1 | grep "^error" | head -20
    echo ""
    echo -e "${RED}Fix compilation errors before running tests.${NC}"
    exit 1
fi
echo -e "${GREEN}  [OK]${NC} cargo build succeeded (5 pts pending unit tests)"

# ── 2. Unit Tests [55 pts] ────────────────────────────────
section "2. Unit Tests (cargo test) [60 pts]"

set +e
TEST_OUTPUT=$(cargo test 2>&1)
CARGO_TEST_EXIT=$?
set -e

FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep "^test .* FAILED" || true)
PASSED_TESTS=$(echo "$TEST_OUTPUT" | grep "^test .* ok"     || true)

PASS_COUNT=$(echo "$PASSED_TESTS" | grep -c "ok"     || true)
FAIL_COUNT=$(echo "$FAILED_TESTS" | grep -c "FAILED" || true)

# Also treat a non-zero cargo exit as a failure (e.g. test compilation error)
if [[ "$CARGO_TEST_EXIT" -ne 0 && "$FAIL_COUNT" -eq 0 ]]; then
    FAIL_COUNT=1
fi

info "Tests passed: $PASS_COUNT"
info "Tests failed: $FAIL_COUNT"

if [[ "$FAIL_COUNT" -gt 0 ]]; then
    echo "$FAILED_TESTS"
fi

# Individual test group checks — run each test binary separately
check_test_group() {
    local group="$1"
    local test_binary="$2"
    local pts="$3"
    local output
    output=$(cargo test --test "$test_binary" 2>&1 || true)
    local count
    count=$(echo "$output" | grep -c "^test .* ok" || true)
    if [[ "$count" -gt 0 ]]; then
        pass "$group ($count tests)" "$pts"
    else
        fail "$group — no tests passed in $test_binary" "$pts"
    fi
}

check_test_group "game::tests"   "game"  20
check_test_group "words::tests"  "words" 25

# Only the three TimeOutForLevel tests — signal handler tests belong to section 3
TIMER_TIMEOUT_OUTPUT=$(cargo test --test timer test_timeout_level 2>&1 || true)
TIMER_TIMEOUT_COUNT=$(echo "$TIMER_TIMEOUT_OUTPUT" | grep -c "^test .* ok" || true)
if [[ "$TIMER_TIMEOUT_COUNT" -gt 0 ]]; then
    pass "timer::tests — TimeOutForLevel ($TIMER_TIMEOUT_COUNT tests)" 10
else
    fail "timer::tests — TimeOutForLevel tests did not pass" 10
fi

# Award compilation pts only if all unit tests passed
if [[ "$FAIL_COUNT" -eq 0 ]]; then
    pass "Project compiles and all unit tests pass" 5
else
    fail "Compilation points withheld — fix failing tests first" 5
fi

# ── 3. Signal Handler Validation [30 pts] ─────────────────
section "3. Signal Handler Validation [40 pts]"

run_single_test() {
    local test_name="$1"
    cargo test --test timer "$test_name" 2>&1 | grep -q "test $test_name ... ok"
}

# 3.1 — RegisterSignalHandler wires up the SIGALRM handler (20 pts)
if run_single_test "test_signal_handler_registered"; then
    pass "RegisterSignalHandler: handler fires on manual SIGALRM" 20
else
    fail "RegisterSignalHandler: handler did not fire on SIGALRM" 20
fi

# 3.2 — Start arms setitimer and handler fires automatically (20 pts)
if run_single_test "test_start_fires_handler_twice"; then
    pass "Start: setitimer armed — handler fired at least twice in 2.5 s" 20
else
    fail "Start: handler did not fire via setitimer within 2.5 s" 20
fi

# ── Summary ───────────────────────────────────────────────
echo ""
echo "══════════════════════════════════════"
TOTAL=$((PASS + FAIL))
echo -e "  Results: ${GREEN}$PASS passed${NC} / ${RED}$FAIL failed${NC} (${TOTAL} checks)"
echo -e "  Score:   ${GREEN}${TOTAL_SCORE}${NC} / 100"
echo "══════════════════════════════════════"

if [[ "$FAIL" -eq 0 ]]; then
    echo -e "${GREEN}All checks passed! ✓${NC}"
else
    echo -e "${RED}$FAIL check(s) failed. Review output above.${NC}"
fi
