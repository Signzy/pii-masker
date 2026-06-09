#!/usr/bin/env bash
#
# ci-check.sh — Full CI quality gate for pii-masker.
# Run before every commit. Fails fast on first violation.
#
# Usage:
#   ./ci-check.sh           # check-only mode
#   ./ci-check.sh --fix     # auto-fix formatting
#
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0

pass() { echo -e "  ${GREEN}PASS${NC}  $1"; PASS=$((PASS + 1)); }
fail() { echo -e "  ${RED}FAIL${NC}  $1 — $2"; FAIL=$((FAIL + 1)); }
info()  { echo -e "  ${YELLOW}INFO${NC}  $1"; }

FIX_MODE=false
if [[ "${1:-}" == "--fix" ]]; then
    FIX_MODE=true
fi

echo ""
echo "════════════════════════════════════════════════"
echo "  pii-masker CI check"
echo "════════════════════════════════════════════════"
echo ""

# ── 1. Formatting ───────────────────────────────────────────────────

echo "── Formatting ──"
if $FIX_MODE; then
    cargo fmt
    pass "cargo fmt applied"
else
    if cargo fmt --check 2>/dev/null; then
        pass "cargo fmt"
    else
        fail "cargo fmt" "run './ci-check.sh --fix' to auto-format"
    fi
fi
echo ""

# ── 2. Clippy (pedantic + nursery) ──────────────────────────────────

echo "── Clippy ──"
if cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings 2>/dev/null; then
    pass "cargo clippy"
else
    fail "cargo clippy" "fix warnings above"
fi
echo ""

# ── 3. File size ────────────────────────────────────────────────────

echo "── File size (max 500 lines per .rs file) ──"
MAX_LINES=500
while IFS= read -r -d '' file; do
    lines=$(wc -l < "$file")
    if [ "$lines" -gt "$MAX_LINES" ]; then
        fail "$file" "$lines lines (max $MAX_LINES) — split it up"
    else
        pass "$file ($lines lines)"
    fi
done < <(find src -name '*.rs' -print0 2>/dev/null)
echo ""

# ── 4. Unsafe code audit ────────────────────────────────────────────

echo "── Unsafe code ──"
UNSAFE_COUNT=$(grep -r 'unsafe ' src/ 2>/dev/null | grep -v '//.*unsafe' | grep -v 'forbid(unsafe_code)' | wc -l | tr -d ' ')
if [ "$UNSAFE_COUNT" -eq 0 ]; then
    pass "no unsafe blocks"
else
    fail "unsafe found" "$UNSAFE_COUNT block(s) — justify in AGENTS.md and add SAFETY comment"
fi
echo ""

# ── 5. Tests ────────────────────────────────────────────────────────

echo "── Tests ──"
if cargo test --quiet 2>/dev/null; then
    pass "cargo test"
else
    fail "cargo test" "fix failing tests above"
fi
echo ""

# ── 6. Docs ─────────────────────────────────────────────────────────

echo "── Docs ──"
if cargo doc --no-deps --document-private-items 2>/dev/null; then
    pass "cargo doc"
else
    fail "cargo doc" "fix broken doc links"
fi
echo ""

# ── Summary ─────────────────────────────────────────────────────────

echo "════════════════════════════════════════════════"
TOTAL=$((PASS + FAIL))
echo "  $PASS/$TOTAL checks passed"
if [ "$FAIL" -gt 0 ]; then
    echo -e "  ${RED}$FAIL failure(s)${NC} — fix before pushing"
    echo "════════════════════════════════════════════════"
    exit 1
else
    echo -e "  ${GREEN}All clear${NC}"
    echo "════════════════════════════════════════════════"
fi
