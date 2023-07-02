# mparith
Multi-precision arithmetic allows users to work with integers that cannot fit in the primitive types. 
To avoid these limits, we store integers in a vector and treat each number as being a digit in some appropriate base.
For example, if we stored the number 2^32 in base 2^32, our vector might look something like [1,0] to represent 1 * 2^32 + 0 * 2^0.
To implement the addition, subtraction, division, and multiplication operations, we have based our implementations off of those described in Hans Riesel's "Prime Numbers and Computer Methods for Factorization" and Donald Knuth's TAOCP.
We have also implemented the comparison operations, bit operations, exponentiation, isqrt, and negation (ie a = -a is computed differently than a = -1 * a).

## functions
- `build_bigint(&str)` - converts a string containing a number in decimal format into a bigint
- `build_bigint_bin(&str)` - converts a string containing a number in binary format into a bigint
- `pow(&BigInt, &BigInt)` - raises the left argument to the power of the right argument
- `abs(&BigInt)` - returns the absolute value of a bigint
- `isqrt(&BigInt)` - returns the floor of the square root of a bigint

## methods
- `to_string(&self)` - converts a bigint to a string containing a number in decimal format
- `to_string_bin(&self)` - converts a bigint to a string containing a number in binary format
- `pow(self, BigInt)` - raises a bigint to the power of the argument provided
- `pow(self, &BigInt)` - raises a bigint to the power of the argument provided
- `abs(self)` - returns the absolute value of a bigint
- `isqrt(self)` - returns the floor of the square root of a bigint

## testing
Since Python supports bignums, we used the language to generate 1000 pairs of random numbers ranging from -10^100 to 10^100 (using a log scale to distribute numbers more evenly between the different orders of magnitude). 
We then checked that these numbers with the +,-,\*,/,%,==,>,<,|,^,& operations all outputted the proper result. 
Checking for the isqrt was done with the absolute value of the first number of each pair.
We also added a few edge cases, mostly those involving operations that resulted in 0.
To test the bit shifts, we generated a random number from 0-300 and shifted the first number from each of the thousand pairs by this amount.
We then used this number from 0-300 and raised a number between 0-100 to this power (note: we are planning on testing exponentiation more extensively after implementing fast multiplication).

We have provided mul.txt which contains 100 pairs of numbers each in the range [2**(62 * 249), 2**(62 * 250) - 1] to show that multiplying these numbers using karatsuba is faster as expected, but the exact cutoff to use karatsuba vs gradeschool multiplication hasn't been determined yet.

## future work
We are interested in adding faster multiplication/division algorithms and determining the cutoffs to be used for each algorithm.
This library is intended to be used in a future personal project involving pell equations.
