# BLS12-381-PRIMARY-KEY-CALCULATION

# Solution for finding Primary key using Priavate key for BLS12-381
Explanation below
The curve, as usually represented, by the "short Weierstrass equation" y^2 = x^3 + A*x + B, is a subset of a plane of points (x,y) (plus one extra point at infinity).

This is a very special subset because it has an additional group structure, meaning you can "add together" curve points, and the result will be again a curve point. 

If you can add together points, you can also add a point G together n times, that's denoted by n*G.

In ECC the curve is defined over a finite field. The point G, often called the "generator" on the curve is also fixed, the private key is a random number n between 0 and the number of points on the curve N (which is the same order of magnitude, but not equal to P). Now the public key is the single point n*G (remember that G is a fixed publicly known parameter), but you can think a point also to have two coordinates (x,y) (just remember that not any two coordinates will work, it must be on the curve). Using Lambdaworks MSM, this has been achieved.

