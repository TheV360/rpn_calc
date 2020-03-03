Design thing

String
-> Infix Tokens from String
-> Process 
 --> Implicit Multiplication
 --> Implicit function parenthesis
 --> Unary operators
-> Infix


(helper function that does this entire String -> Expression in one step?)
(postfix_tokens_from_string????? new_from_string????)

sin 6x^2
 \
  `->	6.0
		x
		2.0
		^
		*
		sin

sin(6)x^2
 \
  `->	6.0
		sin
		x
		2.0
		^
		*

clamp(6, 0, 1)
 \
  `->	6
		0
		1
		clamp

max(-32x, x)
 \
  `->	-32
		x
		*
		x
		max

(possible feature??????)
(implicitly added length token?)
max(a, b, c, d)
 \ 
  `->	a
		b
		c
		d
		[len: 4]
		max
