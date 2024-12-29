# The Temple of texts

![Icon](icon.svg)

The Temple of Texts is a variation of [The Library of Babel website](https://en.wikipedia.org/wiki/The_Library_of_Babel_(website)).

It modifies it so that:   
    1. It contains every possible finite string of text of a certain character set (see below for details), instead of just 410*3200 charactes.   
    2. The feeling of being lost is enhanced by allowing only moving and reading, instead of being dinimished by extraneous features that break that feeling (like searching).   

the character set is `abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.!? `.

# How it works

You are dropped into a random part of the infinite grid of octagons, facing a random wall/door. 
The program also generates a random key, with which it encrypts the position in the grid to obtain the texts on the walls.
The encryption algorithm itself is quite simple and probably is broken cryptographically. (You can check it in the source code.)
Which is why, if you are able to recover the key and enter it in the "game", you get a winning end text and the program halts.
So this could be interpreted as a text adventure in which you are lost and try to find out where you are, 
since recovering the key means you've figured out your position as well.

# Contents

Just to help you comprehend it somewhat, I've developed a list of things it contains somewhere in one of its infinite walls (inexhaustive, of course):

- Assuming some encoding which allows it, every single work of humanity.
- A complete history of the observable universe, down to the atom.
- Every single way your life could have played out.
- The strongest arguments for and against every philosophical idea humanity can ever think of.
- All of the above concatenated to form a single string.
