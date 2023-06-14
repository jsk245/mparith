# mparith
Multi-precision arithmetic allows users to work with integers that cannot fit in the primitive types. 
To avoid these limits, we store integers in a vector and treat each number as being a digit in some appropriate base.
For example, if we stored the number 2^32 in base 2^32, our vector might look something like [1,0] to represent 1 * 2^32 + 0 * 2^0.
To implement the addition, subtraction, division, and multiplication operations, we have based our implementations off of those described in Hans Riesel's "Prime Numbers and Computer Methods for Factorization".
We have also implemented the comparison operations (ie ==, >, <, >=, <=).

## functions
- `build_bigint(&str)` - converts a string containing a number in decimal format into a bigint
- `build_bigint_bin(&str)` - converts a string containing a number in binary format into a bigint

## methods
- `to_string(&self)` - converts a bigint to a string containing a number in decimal format
- `to_string_bin(&self)` - converts a bigint to a string containing a number in binary format

## testing
Since Python supports bignums, we used the language to generate 1000 pairs of random numbers ranging from -10^100 to 10^100 (using a log scale to distribute numbers more evenly between the different orders of magnitude). 
We then checked that these numbers with the +,-,\*,/,%,==,>,< operations all outputted the proper result. 
We also added a few edge cases, mostly those involving operations that resulted in 0.

## future work
We are interested in adding faster multiplication/division algorithms and adding some commonly used operations (like comparisons).
This library is intended to be used in a future personal project involving pell equations.
