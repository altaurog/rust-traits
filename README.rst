Rust Traits
============

This is a Rust port of `UPenn CS194 Homework 5`_, which explores
Haskell typeclasses.

.. _UPenn CS194 Homework 5: https://www.seas.upenn.edu/~cis194/spring13/hw/05-type-classes.pdf

Conclusions
-----------
* Rust traits work just like Haskell typeclasses.  They can even be
  implemented for function and closure types.
* Rust is much more verbose, not nearly as elegant as Haskell.
* Rust makes you do the memory management.

Exercises
----------
:Exercise 1: `src/eval.rs <src/eval.rs>`_
:Exercise 2: `src/parse.rs <src/parse.rs>`_
:Exercise 3: `src/exprt.rs <src/exprt.rs>`_
:Exercise 4:
    | `src/boolean.rs <src/boolean.rs>`_
    | `src/integer.rs <src/integer.rs>`_
    | `src/minmax.rs <src/minmax.rs>`_
    | `src/mod7.rs <src/mod7.rs>`_
:Exercise 5: `src/program.rs <src/program.rs>`_
:Exercise 6: `src/hasvars.rs <src/hasvars.rs>`_
