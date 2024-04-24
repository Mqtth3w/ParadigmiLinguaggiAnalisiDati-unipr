#!/usr/bin/env python3
"""
@author  Michele Tomaiuolo - http://www.ce.unipr.it/people/tomamic
@license This software is free - http://www.gnu.org/licenses/gpl.html
"""

import re
from math import isclose
from syntree import *

class Tokenizer:
    def __init__(self, text):
        regex = r"\s*([A-Za-z0-9\.]+|.?)"
        self._tokens = re.finditer(regex, text.rstrip())
        self._next = next(self._tokens)

    def peek(self) -> str:
        return self._next.group(1)

    def consume(self, x):
        if self.peek() != x:
            raise SyntaxError("Expected " + x)
        self._next = next(self._tokens)

    def end(self):
        if self.peek():
            raise SyntaxError("Extra tokens")


# expr = term {( "+" | "-" ) term}
# term = factor {( "*" | "/" ) factor}
# factor = "-" factor | "(" expr ")" | identifier | number | "if" expr "then" expr "else" expr | | "let" identifier "=" expr {"," identifier "=" expr} "in" expr
# (identifiers start with a letter, numbers are float)

# expr = term {( "+" | "-" ) term}
def expr(tok: Tokenizer) -> Expr:
    x = term(tok)
    while tok.peek() in ("+", "-"):
        op = tok.peek()
        tok.consume(op)
        y = term(tok)
        x = BinaryOp(op, x, y)
    return x

# term = factor {( "*" | "/" ) factor}
def term(tok: Tokenizer) -> Expr:
    x = factor(tok)
    while tok.peek() in ("*", "/"):
        op = tok.peek()
        tok.consume(op)
        y = factor(tok)
        x = BinaryOp(op, x, y)
    return x

# factor = "-" factor | "(" expr ")" | identifier | number | "if" expr "then" expr "else" expr | "let" identifier "=" expr {"," identifier "=" expr} "in" expr
def factor(tok: Tokenizer) -> Expr:
    nxt = tok.peek()
    if nxt == "-":
        tok.consume("-")
        x = factor(tok)
        return UnaryOp("~", x)
    elif nxt == "if":
        tok.consume("if")
        cond = expr(tok)
        tok.consume("then")
        true_ex = expr(tok)
        tok.consume("else")
        false_ex = expr(tok)
        return TernaryOp(cond, true_ex, false_ex)
    elif nxt == "let":
        tok.consume("let")
        var = factor(tok)
        tok.consume("=")
        as_ex = expr(tok)
        variables = {var:as_ex}
        while tok.peek() in (",",):
            tok.consume(",")
            var = factor(tok)
            tok.consume("=")
            as_ex = expr(tok)
            variables[var] = as_ex
        tok.consume("in")
        in_ex = expr(tok)
        return LetOp(variables, in_ex)
    elif nxt == "(":
        tok.consume("(")
        x = expr(tok)
        tok.consume(")")
        return x
    elif nxt.isalpha():
        tok.consume(nxt)
        return Var(nxt)
    else:
        tok.consume(nxt)
        return Num(float(nxt))


# Tests
def main():
    ctx = {"w": 0.0}
    '''
    tests2 = [("1 + ( if 0 then 3 else 3 + 4 )", 8.0),
              ("1 + ( if 1 then 3 else 3 + 4 )", 4.0)]
    
    for infix, val in tests2:
        tok = Tokenizer(infix)
        ast = expr(tok)
        #print(ast.eval(ctx))
        #print(ast.infix())
        tok.end()
        assert isclose(ast.eval(ctx), val)
    '''
    
    tests3 = [("let x = 3 in x * x + 5", 14.0),  #op1
              ("let y = 10 in 1 + (let y = 3 in y * y) + y", 20.0), #op1
              ("let x = 3, y = 2 in x + y", 5.0),
              ("let u = 1 in 1 + (let y = 3 in w * y) + u", 2.0),
              ("let x = 1, y = 10 in x + (let y = 3 in x - 1 + y * y) + y", 20.0)]
    
    for infix, val in tests3:
        tok = Tokenizer(infix)
        ast = expr(tok)
        print(ast.eval(ctx))
        print(ast.infix())
        tok.end()
        assert isclose(ast.eval(ctx), val)
    
    '''#before dict for multi var
    prod1 = BinaryOp("*", Var("x"), Var("x"))
    sum1 = BinaryOp("+", prod1, Num(5))
    let_op1 = LetOp(Var("x"), Num(3), sum1)
    print(let_op1.eval(ctx))
    prod2 = BinaryOp("*", Var("y"), Var("y"))
    ind_let = LetOp(Var("y"), Num(3), prod2)
    sum2 = BinaryOp("+", ind_let, Var("y"))
    sum22 = BinaryOp("+", Num(1), sum2)
    #print(sum2.eval(ctx))
    let_op2 = LetOp(Var("y"), Num(10), sum22)
    print(let_op2.eval(ctx))
    '''
    
    
if __name__ == "__main__":
    main()