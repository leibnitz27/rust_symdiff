# rust_symdiff
Toy symbolic differentiator (to play with rust).

**Interesting things**

I tried implementing this using trait objects, and it became massively unwieldy very quickly.  
It got really ugly when Expression had to derive fmt::Display, and I still couldn't 'format!', because the expression wasn't sized.

**What's missing (Oh So Much).**

* Parsing expressions
* Precedence display
* Nice display (3*x -> 3x)
* Non integral exponents
* Trig functions
