#!/usr/bin/env python3
"""parse file
@author  Matteo Gianvenuti - https://github.com/mqtth3w
@license This software is free - http://www.gnu.org/licenses/gpl.html
"""

import re
from math import isclose
from syntree import *

class Tokenizer:
    def __init__(self, text):
        #regex = r"\s*([A-Za-z0-9\.]+|.?)"
        regex = r"\s*([A-Za-z0-9\.]+|\*{2}|\|x\||.?)"   
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
            print(self.peek())
            raise SyntaxError("Extra tokens")

# stmt = expr | if expr ? expr : expr 
# expr = term {( "+" | "-" ) term}
# term = power {( "*" | "/" ) power}
# power = factor {( "**" ) factor}
# factor = "-" factor | "(" expr ")" | identifier | number | "|x|" factor 
# (identifiers start with a letter, numbers are float)

# stmt = expr | if expr ? expr : expr | ":"name "=" expr 
def stmt(tok: Tokenizer) -> Expr:
    if tok.peek() == "if":
        tok.consume("if")
        cond = expr(tok)
        tok.consume("?")
        true_val = expr(tok)
        tok.consume(":")
        false_val = expr(tok)
        return TernaryOp(cond, true_val, false_val)
    elif tok.peek() == ":":
        tok.consume(":")
        name = factor(tok)
        tok.consume("=")
        value = expr(tok)
        update_ctx(name, value)
        return value
    else:
        return expr(tok)

# expr = term {( "+" | "-" ) term}
def expr(tok: Tokenizer) -> Expr:
    x = term(tok)
    while tok.peek() in ("+", "-"):
        op = tok.peek()
        tok.consume(op)
        y = term(tok)
        x = BinaryOp(op, x, y)
    return x

# term = power {( "*" | "/" ) power}
def term(tok: Tokenizer) -> Expr:
    x = power(tok)
    while tok.peek() in ("*", "/"):
        op = tok.peek()
        tok.consume(op)
        y = power(tok)
        x = BinaryOp(op, x, y)
    return x

# power = factor {( "**" ) factor}
def power(tok: Tokenizer) -> Expr:
    x = factor(tok)
    while tok.peek() in ("**",):
        op = tok.peek()
        tok.consume(op)
        y = factor(tok)
        x = BinaryOp(op, x, y)
    return x

# factor = "-" factor | "(" expr ")" | identifier | number | "|x|" factor 
def factor(tok: Tokenizer) -> Expr:
    nxt = tok.peek()
    if nxt == "-":
        tok.consume("-")
        x = factor(tok)
        return UnaryOp("~", x)
    elif nxt == "(":
        tok.consume("(")
        x = expr(tok)
        tok.consume(")")
        return x
    elif nxt == "|x|":
        tok.consume("|x|")
        x = factor(tok)
        return UnaryOp("|x|", x)
    elif nxt.isalpha(): # True if a-z or A-Z 
        tok.consume(nxt)
        return Var(nxt)
    else:
        tok.consume(nxt)
        return Num(float(nxt))

def update_ctx(name: Var, value: Expr):
    global ctx
    ctx[name.prefix()] = value.eval(ctx)
    
    
# Tests
def main():
    global ctx
    ctx = {"w": 0.0, "x": 1.0, "y": 1.5, "z": 0.5}

    tests = [("(((1.5)))", "1.5", 1.5),
             ("w * -z", "* w ~z", 0.0),
             ("x / z * -y", "* / x z ~y", -3.0),
             ("x / 0.5 * --y", "* / x 0.5 ~~y", 3.0),
             ("w", "w", 0.0),
             ("(x + w) * (x + y)", "* + x w + x y", 2.5),
             ("x ** y", "** x y", 1.0),
             ("2.0 + 2.0 * x ** y", "+ 2.0 * 2.0 ** x y", 4.0),
             ("(x + w) ** (x + y)", "** + x w + x y", 1.0),
             ("if 1.0 ? 60.0 : 20.0", "60.0 if 1.0 else 20.0", 60.0),
             ("|x|(1.0 - 4.0)", "|x|- 1.0 4.0", 3.0),
             (":x = 8.0", "8.0", 8.0),
             (":uu = 5.0", "5.0", 5.0)]  

    for infix, prefix, val in tests:
        tok = Tokenizer(infix)
        #ast = expr(tok)
        ast = stmt(tok)
        print("infix no ast:", infix, "prefix no ast:", prefix)
        print("infix:", ast.infix(), "prefix:", ast.prefix(), "eval:", ast.eval(ctx))
        tok.end()
        #print(ast.prefix(), "   ", prefix)
        assert ast.prefix() == prefix
        assert isclose(ast.eval(ctx), val)
    print(ctx)
    
if __name__ == "__main__":
    main()