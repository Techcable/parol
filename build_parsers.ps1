$ErrorCont = 0

Write-Host "Building release. Please wait..." -ForegroundColor Cyan
cargo build --release
if ($LASTEXITCODE -ne 0) {
    ++$ErrorCont    
}


# --------------------------------------------------------------------------------------------------
Write-Host "Building parol..." -ForegroundColor Cyan
./target/release/parol -f ./src/parser/parol-grammar.par -e ./src/parser/parol-grammar-exp.par -p ./src/parser/parol_parser.rs -a ./src/parser/parol_grammar_trait.rs -t ParolGrammar -m parser::parol_grammar
if ($LASTEXITCODE -ne 0) {
    ++$ErrorCont    
}

# --------------------------------------------------------------------------------------------------
Write-Host "Building Calc example..." -ForegroundColor Cyan
./target/release/parol -f ./examples/calc/calc.par -e ./examples/calc/calc-exp.par -p ./examples/calc/calc_parser.rs -a ./examples/calc/calc_grammar_trait.rs -t CalcGrammar -m calc_grammar
if ($LASTEXITCODE -ne 0) {
    ++$ErrorCont    
}

# --------------------------------------------------------------------------------------------------
Write-Host "Building List example..." -ForegroundColor Cyan
./target/release/parol -f ./examples/list/list.par -e ./examples/list/list-exp.par -p ./examples/list/list_parser.rs -a ./examples/list/list_grammar_trait.rs -t ListGrammar -m list_grammar
if ($LASTEXITCODE -ne 0) {
    ++$ErrorCont    
}

# --------------------------------------------------------------------------------------------------
Write-Host "Building Oberon-0 example..." -ForegroundColor Cyan
./target/release/parol -f ./examples/oberon_0/oberon_0.par -e ./examples/oberon_0/oberon_0-exp.par -p ./examples/oberon_0/oberon_0_parser.rs -a ./examples/oberon_0/oberon_0_grammar_trait.rs -t Oberon0Grammar -m oberon_0_grammar
if ($LASTEXITCODE -ne 0) {
    ++$ErrorCont    
}

# --------------------------------------------------------------------------------------------------
Write-Host "Building Scanner States example..." -ForegroundColor Cyan
./target/release/parol -f ./examples/scanner_states/scanner_states.par -e ./examples/scanner_states/scanner_states-exp.par -p ./examples/scanner_states/scanner_states_parser.rs -a ./examples/scanner_states/scanner_states_grammar_trait.rs -t ScannerStatesGrammar -m scanner_states_grammar
if ($LASTEXITCODE -ne 0) {
    ++$ErrorCont    
}

# --------------------------------------------------------------------------------------------------
Write-Host "Building Boolean Parser example..." -ForegroundColor Cyan
./target/release/parol -f ./examples/boolean_parser/boolean-parser.par -e ./examples/boolean_parser/boolean-parser-exp.par -p ./examples/boolean_parser/boolean_parser.rs -a ./examples/boolean_parser/boolean_grammar_trait.rs -t BooleanGrammar -m boolean_grammar

# --------------------------------------------------------------------------------------------------
# Final message
# --------------------------------------------------------------------------------------------------
if ($ErrorCont -gt 0) {
    $Msg = "$ErrorCount error(s) occurred during builds."
    Write-Host -Object $Msg  -ForegroundColor Red
} else {
    Write-Host "All builds successfully executed." -ForegroundColor Green
}
